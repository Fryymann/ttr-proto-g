use std::collections::HashMap;
use std::env;
use std::sync::Arc;

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tracing::{error, info};

use ttrpg_protocol::{to_json_line, CharacterDraft, ClientMessage, ServerMessage};

type ClientTx = mpsc::UnboundedSender<ServerMessage>;
type SharedState = Arc<Mutex<ServerState>>;

#[derive(Clone)]
struct Room {
    id: String,
    name: String,
    description: String,
    exits: HashMap<String, String>,
}

#[derive(Clone)]
struct Player {
    id: u64,
    character: CharacterDraft,
    room_id: String,
}

impl Player {
    fn display_name(&self) -> &str {
        &self.character.name
    }
}

struct ServerState {
    next_player_id: u64,
    players: HashMap<u64, Player>,
    player_name_index: HashMap<String, u64>,
    sessions: HashMap<u64, ClientTx>,
    rooms: HashMap<String, Room>,
}

impl ServerState {
    fn new() -> Self {
        Self {
            next_player_id: 1,
            players: HashMap::new(),
            player_name_index: HashMap::new(),
            sessions: HashMap::new(),
            rooms: build_world(),
        }
    }
}

#[derive(Default)]
struct SessionContext {
    player_id: Option<u64>,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let addr = env::var("TTRPG_SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:7000".to_owned());
    let listener = TcpListener::bind(&addr).await?;
    let state = Arc::new(Mutex::new(ServerState::new()));

    info!("server listening on {}", addr);

    loop {
        let (socket, peer_addr) = listener.accept().await?;
        let state = Arc::clone(&state);
        tokio::spawn(async move {
            if let Err(err) = handle_connection(socket, state).await {
                error!("connection {} failed: {}", peer_addr, err);
            }
        });
    }
}

async fn handle_connection(socket: TcpStream, state: SharedState) -> io::Result<()> {
    let (reader, mut writer) = socket.into_split();
    let (tx, mut rx) = mpsc::unbounded_channel::<ServerMessage>();

    let writer_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            let line = match to_json_line(&message) {
                Ok(line) => line,
                Err(err) => {
                    error!("failed to serialize server message: {}", err);
                    continue;
                }
            };

            if writer.write_all(line.as_bytes()).await.is_err() {
                break;
            }
        }
    });

    send_to_client(
        &tx,
        ServerMessage::Info {
            text: "Welcome to TTRPG M1. Use /create (editor) or /login <name>.".to_owned(),
        },
    );
    send_prompt(&tx, false);

    let mut session = SessionContext::default();
    let mut lines = BufReader::new(reader).lines();

    while let Some(line) = lines.next_line().await? {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            send_prompt(&tx, session.player_id.is_some());
            continue;
        }

        match serde_json::from_str::<ClientMessage>(trimmed) {
            Ok(message) => {
                process_client_message(message, &tx, &state, &mut session).await;
            }
            Err(err) => {
                send_to_client(
                    &tx,
                    ServerMessage::Error {
                        text: format!("Invalid message format: {}", err),
                    },
                );
            }
        }

        send_prompt(&tx, session.player_id.is_some());
    }

    disconnect_player(&state, &mut session).await;
    drop(tx);
    let _ = writer_task.await;

    Ok(())
}

async fn process_client_message(
    message: ClientMessage,
    tx: &ClientTx,
    state: &SharedState,
    session: &mut SessionContext,
) {
    match message {
        ClientMessage::Ping => {
            send_to_client(tx, ServerMessage::Pong);
        }
        ClientMessage::CreateCharacter { character } => {
            handle_create_character(character, tx, state, session).await;
        }
        ClientMessage::Login { name } => {
            handle_login(name, tx, state, session).await;
        }
        ClientMessage::CommandText { command } => {
            let Some(player_id) = session.player_id else {
                send_to_client(
                    tx,
                    ServerMessage::Error {
                        text: "Authenticate first with /create or /login.".to_owned(),
                    },
                );
                return;
            };
            run_command(command, player_id, tx, state).await;
        }
    }
}

async fn handle_create_character(
    character: CharacterDraft,
    tx: &ClientTx,
    state: &SharedState,
    session: &mut SessionContext,
) {
    if session.player_id.is_some() {
        send_to_client(
            tx,
            ServerMessage::Error {
                text: "Already authenticated for this connection.".to_owned(),
            },
        );
        return;
    }

    let character = match normalize_character(character) {
        Ok(character) => character,
        Err(error_text) => {
            send_to_client(tx, ServerMessage::Error { text: error_text });
            return;
        }
    };

    let name_key = name_key(&character.name);
    let mut join_recipients: Vec<ClientTx> = Vec::new();
    let player_id: u64;
    let room_id: String;
    let player_name: String;
    let room_state: Option<ServerMessage>;

    {
        let mut locked = state.lock().await;

        if locked.player_name_index.contains_key(&name_key) {
            send_to_client(
                tx,
                ServerMessage::Error {
                    text: "Character already exists. Use /login <name>.".to_owned(),
                },
            );
            return;
        }

        player_id = locked.next_player_id;
        locked.next_player_id += 1;

        room_id = "town_square".to_owned();
        player_name = character.name.clone();

        let player = Player {
            id: player_id,
            character: character.clone(),
            room_id: room_id.clone(),
        };

        locked.players.insert(player_id, player);
        locked.player_name_index.insert(name_key, player_id);
        locked.sessions.insert(player_id, tx.clone());

        room_state = room_state_for_player_locked(&locked, player_id);

        join_recipients.extend(
            players_in_room_senders_locked(&locked, &room_id, Some(player_id))
                .into_iter()
                .collect::<Vec<_>>(),
        );
    }

    session.player_id = Some(player_id);

    send_to_client(
        tx,
        ServerMessage::AuthOk {
            player_id,
            name: player_name.clone(),
        },
    );

    if let Some(state_message) = room_state {
        send_to_client(tx, state_message);
    }

    let join_text = format!("{} enters the area.", player_name);
    for recipient in join_recipients {
        send_to_client(
            &recipient,
            ServerMessage::ChatMsg {
                from: "system".to_owned(),
                channel: "room".to_owned(),
                text: join_text.clone(),
            },
        );
    }
}

async fn handle_login(
    name: String,
    tx: &ClientTx,
    state: &SharedState,
    session: &mut SessionContext,
) {
    if session.player_id.is_some() {
        send_to_client(
            tx,
            ServerMessage::Error {
                text: "Already authenticated for this connection.".to_owned(),
            },
        );
        return;
    }

    let clean_name = name.trim();
    let lookup_key = name_key(clean_name);
    let mut join_recipients: Vec<ClientTx> = Vec::new();
    let player_id: u64;
    let room_id: String;
    let player_name: String;
    let room_state: Option<ServerMessage>;

    {
        let mut locked = state.lock().await;
        let Some(found_id) = locked.player_name_index.get(&lookup_key).copied() else {
            send_to_client(
                tx,
                ServerMessage::Error {
                    text: "Character does not exist. Use /create to build one.".to_owned(),
                },
            );
            return;
        };

        if locked.sessions.contains_key(&found_id) {
            send_to_client(
                tx,
                ServerMessage::Error {
                    text: "Character is already logged in.".to_owned(),
                },
            );
            return;
        }

        player_id = found_id;
        locked.sessions.insert(player_id, tx.clone());

        let Some(player) = locked.players.get(&player_id).cloned() else {
            send_to_client(
                tx,
                ServerMessage::Error {
                    text: "Character data is missing.".to_owned(),
                },
            );
            locked.sessions.remove(&player_id);
            return;
        };

        room_id = player.room_id.clone();
        player_name = player.display_name().to_owned();

        room_state = room_state_for_player_locked(&locked, player_id);
        join_recipients.extend(
            players_in_room_senders_locked(&locked, &room_id, Some(player_id))
                .into_iter()
                .collect::<Vec<_>>(),
        );
    }

    session.player_id = Some(player_id);

    send_to_client(
        tx,
        ServerMessage::AuthOk {
            player_id,
            name: player_name.clone(),
        },
    );

    if let Some(state_message) = room_state {
        send_to_client(tx, state_message);
    }

    let join_text = format!("{} reconnects.", player_name);
    for recipient in join_recipients {
        send_to_client(
            &recipient,
            ServerMessage::ChatMsg {
                from: "system".to_owned(),
                channel: "room".to_owned(),
                text: join_text.clone(),
            },
        );
    }
}

async fn run_command(command: String, player_id: u64, tx: &ClientTx, state: &SharedState) {
    let trimmed = command.trim();
    if trimmed.is_empty() {
        return;
    }

    let mut parts = trimmed.split_whitespace();
    let verb = parts.next().unwrap_or_default().to_lowercase();

    match verb.as_str() {
        "look" => {
            show_room(player_id, tx, state).await;
        }
        "go" => {
            let Some(direction) = parts.next() else {
                send_to_client(
                    tx,
                    ServerMessage::Error {
                        text: "Usage: go <north|south|east|west>".to_owned(),
                    },
                );
                return;
            };
            move_player(player_id, direction, tx, state).await;
        }
        "say" => {
            let msg = trimmed
                .strip_prefix("say")
                .unwrap_or_default()
                .trim()
                .to_owned();

            if msg.is_empty() {
                send_to_client(
                    tx,
                    ServerMessage::Error {
                        text: "Usage: say <message>".to_owned(),
                    },
                );
                return;
            }

            say_to_room(player_id, msg, state).await;
        }
        "who" => {
            show_who(tx, state).await;
        }
        "sheet" => {
            show_sheet(player_id, tx, state).await;
        }
        "help" => {
            send_to_client(
                tx,
                ServerMessage::Info {
                    text: "Commands: look, go <dir>, say <msg>, who, sheet, help".to_owned(),
                },
            );
        }
        _ => {
            send_to_client(
                tx,
                ServerMessage::Error {
                    text: format!("Unknown command: {}", verb),
                },
            );
        }
    }
}

async fn show_room(player_id: u64, tx: &ClientTx, state: &SharedState) {
    let view = {
        let locked = state.lock().await;
        room_state_for_player_locked(&locked, player_id)
    };

    if let Some(message) = view {
        send_to_client(tx, message);
    } else {
        send_to_client(
            tx,
            ServerMessage::Error {
                text: "Unable to load room state.".to_owned(),
            },
        );
    }
}

async fn show_who(tx: &ClientTx, state: &SharedState) {
    let mut online_names = {
        let locked = state.lock().await;
        locked
            .sessions
            .keys()
            .filter_map(|id| {
                locked
                    .players
                    .get(id)
                    .map(|player| player.display_name().to_owned())
            })
            .collect::<Vec<_>>()
    };

    online_names.sort();
    send_to_client(
        tx,
        ServerMessage::WhoList {
            players: online_names,
        },
    );
}

async fn show_sheet(player_id: u64, tx: &ClientTx, state: &SharedState) {
    let text = {
        let locked = state.lock().await;
        if let Some(player) = locked.players.get(&player_id) {
            format!(
                "Character Sheet\nName: {}\nAncestry: {}\nClass: {}\nBackground: {}\nPronouns: {}\nMotto: {}",
                player.character.name.as_str(),
                player.character.ancestry.as_str(),
                player.character.class_name.as_str(),
                player.character.background.as_str(),
                player.character.pronouns.as_str(),
                player.character.motto.as_str()
            )
        } else {
            "Unable to load character sheet.".to_owned()
        }
    };

    send_to_client(tx, ServerMessage::Info { text });
}

async fn say_to_room(player_id: u64, text: String, state: &SharedState) {
    let (speaker_name, room_id, recipients) = {
        let locked = state.lock().await;
        let Some(player) = locked.players.get(&player_id) else {
            return;
        };

        let speaker_name = player.display_name().to_owned();
        let room_id = player.room_id.clone();
        let recipients = players_in_room_senders_locked(&locked, &room_id, None);

        (speaker_name, room_id, recipients)
    };

    let message = ServerMessage::ChatMsg {
        from: speaker_name,
        channel: format!("room:{}", room_id),
        text,
    };

    for recipient in recipients {
        send_to_client(&recipient, message.clone());
    }
}

async fn move_player(player_id: u64, direction: &str, tx: &ClientTx, state: &SharedState) {
    let direction_key = direction.trim().to_lowercase();

    let (player_name, old_room, new_room, leave_recipients, join_recipients, room_view) = {
        let mut locked = state.lock().await;

        let Some(player_snapshot) = locked.players.get(&player_id).cloned() else {
            send_to_client(
                tx,
                ServerMessage::Error {
                    text: "Unknown player session.".to_owned(),
                },
            );
            return;
        };

        let old_room = player_snapshot.room_id.clone();

        let Some(current_room) = locked.rooms.get(&old_room) else {
            send_to_client(
                tx,
                ServerMessage::Error {
                    text: "Current room is missing.".to_owned(),
                },
            );
            return;
        };

        let Some(new_room) = current_room.exits.get(&direction_key).cloned() else {
            send_to_client(
                tx,
                ServerMessage::Error {
                    text: format!("No exit '{}' from here.", direction_key),
                },
            );
            return;
        };

        if let Some(player_mut) = locked.players.get_mut(&player_id) {
            player_mut.room_id = new_room.clone();
        }

        let leave_recipients = players_in_room_senders_locked(&locked, &old_room, Some(player_id));
        let join_recipients = players_in_room_senders_locked(&locked, &new_room, Some(player_id));
        let room_view = room_state_for_player_locked(&locked, player_id);

        (
            player_snapshot.display_name().to_owned(),
            old_room,
            new_room,
            leave_recipients,
            join_recipients,
            room_view,
        )
    };

    let leave_message = ServerMessage::ChatMsg {
        from: "system".to_owned(),
        channel: format!("room:{}", old_room),
        text: format!("{} leaves to the {}.", player_name, direction_key),
    };

    for recipient in leave_recipients {
        send_to_client(&recipient, leave_message.clone());
    }

    let join_message = ServerMessage::ChatMsg {
        from: "system".to_owned(),
        channel: format!("room:{}", new_room),
        text: format!(
            "{} arrives from the {}.",
            player_name,
            opposite_direction(&direction_key)
        ),
    };

    for recipient in join_recipients {
        send_to_client(&recipient, join_message.clone());
    }

    if let Some(view) = room_view {
        send_to_client(tx, view);
    }
}

async fn disconnect_player(state: &SharedState, session: &mut SessionContext) {
    let Some(player_id) = session.player_id.take() else {
        return;
    };

    let (name, room_id, recipients) = {
        let mut locked = state.lock().await;
        locked.sessions.remove(&player_id);

        let Some(player) = locked.players.get(&player_id) else {
            return;
        };

        let name = player.display_name().to_owned();
        let room_id = player.room_id.clone();
        let recipients = players_in_room_senders_locked(&locked, &room_id, Some(player_id));

        (name, room_id, recipients)
    };

    let message = ServerMessage::ChatMsg {
        from: "system".to_owned(),
        channel: format!("room:{}", room_id),
        text: format!("{} disconnects.", name),
    };

    for recipient in recipients {
        send_to_client(&recipient, message.clone());
    }
}

fn room_state_for_player_locked(state: &ServerState, player_id: u64) -> Option<ServerMessage> {
    let player = state.players.get(&player_id)?;
    let room = state.rooms.get(&player.room_id)?;

    let mut exits = room.exits.keys().cloned().collect::<Vec<_>>();
    exits.sort();

    let mut players = state
        .players
        .values()
        .filter(|p| p.room_id == room.id && state.sessions.contains_key(&p.id))
        .map(|p| p.display_name().to_owned())
        .collect::<Vec<_>>();
    players.sort();

    Some(ServerMessage::RoomState {
        room_id: room.id.clone(),
        room_name: room.name.clone(),
        description: room.description.clone(),
        exits,
        players,
    })
}

fn players_in_room_senders_locked(
    state: &ServerState,
    room_id: &str,
    except_player_id: Option<u64>,
) -> Vec<ClientTx> {
    state
        .players
        .values()
        .filter(|player| player.room_id == room_id)
        .filter(|player| Some(player.id) != except_player_id)
        .filter_map(|player| state.sessions.get(&player.id).cloned())
        .collect()
}

fn send_to_client(tx: &ClientTx, message: ServerMessage) {
    let _ = tx.send(message);
}

fn send_prompt(tx: &ClientTx, authenticated: bool) {
    let prompt = if authenticated { "cmd>" } else { "auth>" };

    send_to_client(
        tx,
        ServerMessage::Prompt {
            text: prompt.to_owned(),
        },
    );
}

fn build_world() -> HashMap<String, Room> {
    let mut rooms = HashMap::new();

    let mut town_square_exits = HashMap::new();
    town_square_exits.insert("north".to_owned(), "tavern".to_owned());
    town_square_exits.insert("east".to_owned(), "forest_edge".to_owned());

    rooms.insert(
        "town_square".to_owned(),
        Room {
            id: "town_square".to_owned(),
            name: "Town Square".to_owned(),
            description: "A busy square with a fountain and a quest board.".to_owned(),
            exits: town_square_exits,
        },
    );

    let mut tavern_exits = HashMap::new();
    tavern_exits.insert("south".to_owned(), "town_square".to_owned());

    rooms.insert(
        "tavern".to_owned(),
        Room {
            id: "tavern".to_owned(),
            name: "Copper Cup Tavern".to_owned(),
            description: "Warm light, noisy patrons, and rumors at every table.".to_owned(),
            exits: tavern_exits,
        },
    );

    let mut forest_edge_exits = HashMap::new();
    forest_edge_exits.insert("west".to_owned(), "town_square".to_owned());

    rooms.insert(
        "forest_edge".to_owned(),
        Room {
            id: "forest_edge".to_owned(),
            name: "Forest Edge".to_owned(),
            description: "Dark trees sway at the boundary of civilization.".to_owned(),
            exits: forest_edge_exits,
        },
    );

    rooms
}

fn normalize_character(character: CharacterDraft) -> Result<CharacterDraft, String> {
    let name = character.name.trim().to_owned();
    let ancestry = character.ancestry.trim().to_owned();
    let class_name = character.class_name.trim().to_owned();
    let background = character.background.trim().to_owned();
    let pronouns = character.pronouns.trim().to_owned();
    let motto = character.motto.trim().to_owned();

    if !is_valid_name(&name) {
        return Err("Name must be 3-20 chars and use letters, numbers, _ or -.".to_owned());
    }

    ensure_value_len("Ancestry", &ancestry, 2, 32)?;
    ensure_value_len("Class", &class_name, 2, 32)?;
    ensure_value_len("Background", &background, 2, 48)?;
    ensure_value_len("Pronouns", &pronouns, 2, 24)?;
    ensure_value_len("Motto", &motto, 2, 120)?;

    Ok(CharacterDraft {
        name,
        ancestry,
        class_name,
        background,
        pronouns,
        motto,
    })
}

fn ensure_value_len(label: &str, value: &str, min: usize, max: usize) -> Result<(), String> {
    let count = value.chars().count();
    if !(min..=max).contains(&count) {
        return Err(format!(
            "{} must be between {} and {} characters.",
            label, min, max
        ));
    }

    Ok(())
}

fn is_valid_name(name: &str) -> bool {
    let len = name.chars().count();
    if !(3..=20).contains(&len) {
        return false;
    }

    name.chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
}

fn name_key(name: &str) -> String {
    name.trim().to_lowercase()
}

fn opposite_direction(direction: &str) -> &'static str {
    match direction {
        "north" => "south",
        "south" => "north",
        "east" => "west",
        "west" => "east",
        _ => "unknown direction",
    }
}
