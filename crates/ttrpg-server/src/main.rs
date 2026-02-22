mod account;
mod campaign;
mod encounter;
mod party;
mod persistence;
#[allow(dead_code)]
mod scene;

use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tracing::{error, info};

use campaign::{CampaignCatalog, CampaignManifestEntry};
use encounter::{
    capture_participants, deterministic_initiative_for_actor, resolve_timeout_fallback,
    EncounterState, TimeoutFallbackAction, TimeoutFallbackError, TurnTimerConfig, TurnTimerMarker,
};
use party::PartyRegistry;
#[cfg(test)]
use persistence::InMemoryPersistence;
use persistence::{
    CreateCharacterError, FilePersistence, JoinCampaignError, Persistence, SnapshotLoadStatus,
};
use scene::queue::CommandEnvelope;
use scene::runtime::SceneRuntime;
use scene::{Grid, Position, Scene, Tile, TileType};
use ttrpg_protocol::{to_json_line, CharacterDraft, ClientMessage, ServerMessage};

type ClientTx = mpsc::UnboundedSender<ServerMessage>;
type SharedState = Arc<Mutex<ServerState>>;

#[derive(Clone)]
struct Room {
    id: String,
    name: String,
    description: String,
    exits: HashMap<String, String>,
    scene_runtime: Option<SceneRuntime>,
}

#[derive(Clone)]
struct Player {
    id: u64,
    account_id: String,
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
    next_encounter_id: u64,
    players: HashMap<u64, Player>,
    player_name_index: HashMap<String, u64>,
    sessions: HashMap<u64, ClientTx>,
    party_registry: PartyRegistry,
    active_encounters: HashMap<String, EncounterState>, // room_id -> active encounter
    rooms: HashMap<String, Room>,
    persistence: Box<dyn Persistence>,
    active_campaign: CampaignManifestEntry,
    turn_timer_config: TurnTimerConfig,
    timeout_fallback_action: TimeoutFallbackAction,
}

impl ServerState {
    fn new(active_campaign: CampaignManifestEntry) -> io::Result<Self> {
        let (persistence, load_status) =
            FilePersistence::open(&active_campaign.campaign_id, &active_campaign.save_path)
                .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))?;

        match load_status {
            SnapshotLoadStatus::FreshStart => info!(
                "campaign save bootstrap created at {}",
                active_campaign.save_path
            ),
            SnapshotLoadStatus::LoadedPrimary => {
                info!(
                    "campaign save loaded from primary {}",
                    active_campaign.save_path
                )
            }
            SnapshotLoadStatus::RecoveredRollback => info!(
                "campaign save recovered from rollback snapshot for {}",
                active_campaign.save_path
            ),
        }

        Ok(Self::new_with_persistence(
            active_campaign,
            Box::new(persistence),
        ))
    }

    fn new_with_persistence(
        active_campaign: CampaignManifestEntry,
        persistence: Box<dyn Persistence>,
    ) -> Self {
        Self {
            next_player_id: 1,
            next_encounter_id: 1,
            players: HashMap::new(),
            player_name_index: HashMap::new(),
            sessions: HashMap::new(),
            party_registry: PartyRegistry::default(),
            active_encounters: HashMap::new(),
            rooms: build_world(),
            persistence,
            active_campaign,
            turn_timer_config: TurnTimerConfig::from_env(),
            timeout_fallback_action: TimeoutFallbackAction::from_env(),
        }
    }

    #[cfg(test)]
    fn new_for_tests(active_campaign: CampaignManifestEntry) -> Self {
        Self::new_with_persistence(active_campaign, Box::new(InMemoryPersistence::default()))
    }
}

#[derive(Default)]
struct SessionContext {
    account_handle: Option<String>,
    player_id: Option<u64>,
}

#[derive(Debug)]
struct StartupConfig {
    addr: String,
    manifest_path: PathBuf,
    campaign_id: Option<String>,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let startup_config = parse_startup_config()?;
    let campaign_catalog = CampaignCatalog::load_from_path(&startup_config.manifest_path)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))?;
    let active_campaign =
        campaign_catalog.resolve_active_campaign(startup_config.campaign_id.as_deref())?;

    let listener = TcpListener::bind(&startup_config.addr).await?;
    let state = Arc::new(Mutex::new(ServerState::new(active_campaign.clone())?));
    let (turn_timeout, fallback_action) = {
        let locked = state.lock().await;
        (
            locked.turn_timer_config.timeout(),
            locked.timeout_fallback_action.as_str(),
        )
    };

    info!(
        "active campaign: {} ({})",
        active_campaign.name, active_campaign.campaign_id
    );
    info!(
        "encounter timeout configured: {:?}, fallback action: {}",
        turn_timeout, fallback_action
    );
    info!("server listening on {}", startup_config.addr);

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

fn parse_startup_config() -> io::Result<StartupConfig> {
    let mut addr = env::var("TTRPG_SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:7000".to_owned());
    let mut manifest_path = env::var("TTRPG_CAMPAIGN_MANIFEST")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("campaigns/manifest.json"));
    let mut campaign_id = env::var("TTRPG_CAMPAIGN_ID").ok();

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--addr" => {
                addr = next_cli_value(&mut args, "--addr")?;
            }
            "--manifest" => {
                manifest_path = PathBuf::from(next_cli_value(&mut args, "--manifest")?);
            }
            "--campaign" => {
                campaign_id = Some(next_cli_value(&mut args, "--campaign")?);
            }
            unknown => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "unknown argument '{}'. Supported: --addr, --manifest, --campaign",
                        unknown
                    ),
                ));
            }
        }
    }

    Ok(StartupConfig {
        addr,
        manifest_path,
        campaign_id,
    })
}

fn next_cli_value(args: &mut impl Iterator<Item = String>, flag: &str) -> io::Result<String> {
    args.next().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("missing value for {}", flag),
        )
    })
}

async fn handle_connection(socket: TcpStream, state: SharedState) -> io::Result<()> {
    let (reader, mut writer) = socket.into_split();
    let (tx, mut rx) = mpsc::unbounded_channel::<ServerMessage>();
    let campaign_name = {
        let locked = state.lock().await;
        locked.active_campaign.name.clone()
    };

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
            text: format!(
                "Welcome to TTRPG M1. Active campaign: {}. First login with /login <account>, then use /create or /play <name>.",
                campaign_name
            ),
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
            handle_account_login(name, tx, state, session).await;
        }
        ClientMessage::SelectCharacter { name } => {
            handle_select_character(name, tx, state, session).await;
        }
        ClientMessage::CommandText { command } => {
            let Some(player_id) = session.player_id else {
                send_to_client(
                    tx,
                    ServerMessage::Error {
                        text: "Authenticate first with /login <account>, then /play <name>."
                            .to_owned(),
                    },
                );
                return;
            };
            run_command(command, player_id, tx, state).await;
        }
        ClientMessage::SceneAction { command } => {
            let player_id = match session.player_id {
                Some(id) => id,
                None => {
                    send_to_client(
                        tx,
                        ServerMessage::Error {
                            text: "Authenticate first.".to_owned(),
                        },
                    );
                    return;
                }
            };

            let mut locked = state.lock().await;
            let (room_id, actor_id) = if let Some(player) = locked.players.get(&player_id) {
                (player.room_id.clone(), player.display_name().to_owned())
            } else {
                return;
            };

            if let Some(encounter) = locked.active_encounters.get(&room_id) {
                if !encounter.is_participant(&actor_id) {
                    send_to_client(
                        tx,
                        ServerMessage::Error {
                            text: "You are not a participant in this encounter.".to_owned(),
                        },
                    );
                    return;
                }

                if !encounter.is_actor_turn(&actor_id) {
                    let active_actor = encounter.active_actor().unwrap_or("unknown");
                    send_to_client(
                        tx,
                        ServerMessage::Error {
                            text: format!("It is {}'s turn.", active_actor),
                        },
                    );
                    return;
                }
            }

            if let Some(room) = locked.rooms.get_mut(&room_id) {
                if let Some(runtime) = &mut room.scene_runtime {
                    runtime.queue.push(CommandEnvelope {
                        actor_id: actor_id.clone(),
                        command,
                        // Deterministic tie-breaking is actor_id + queue sequence for S2.
                        timestamp_ms: 0,
                        sequence_id: 0, // Assigned by push
                    });

                    // Immediate tick for S2 integration (Finding #1)
                    let deltas = runtime.tick();

                    if !deltas.is_empty() {
                        let broadcast_msg = ServerMessage::SceneDelta { deltas };
                        let recipients = players_in_room_senders_locked(&locked, &room_id, None);
                        for recipient in recipients {
                            send_to_client(&recipient, broadcast_msg.clone());
                        }
                    }

                    send_to_client(
                        tx,
                        ServerMessage::Info {
                            text: "Scene action processed.".to_owned(),
                        },
                    );
                } else {
                    send_to_client(
                        tx,
                        ServerMessage::Error {
                            text: "Current room has no scene.".to_owned(),
                        },
                    );
                }
            }
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

    let Some(account_handle) = session.account_handle.as_deref() else {
        send_to_client(
            tx,
            ServerMessage::Error {
                text: "Login to an account first with /login <account>.".to_owned(),
            },
        );
        return;
    };

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
    let snapshot: Option<ttrpg_protocol::SceneSnapshot>;

    {
        let mut locked = state.lock().await;
        let active_campaign_id = locked.active_campaign.campaign_id.clone();
        let entry_scene_id = locked.active_campaign.entry_scene_id.clone();

        let persisted = match locked.persistence.create_character(
            account_handle,
            character,
            &name_key,
            &active_campaign_id,
        ) {
            Ok(record) => record,
            Err(CreateCharacterError::CharacterNameExists) => {
                send_to_client(
                    tx,
                    ServerMessage::Error {
                        text: "Character already exists. Use /play <name>.".to_owned(),
                    },
                );
                return;
            }
            Err(CreateCharacterError::PersistFailed(details)) => {
                send_to_client(
                    tx,
                    ServerMessage::Error {
                        text: format!("Character could not be saved safely: {}", details),
                    },
                );
                return;
            }
        };

        player_id = locked.next_player_id;
        locked.next_player_id += 1;

        room_id = resolve_entry_room_id(&locked.rooms, &entry_scene_id);
        player_name = persisted.character.name.clone();

        let player = Player {
            id: player_id,
            account_id: persisted.account_id,
            character: persisted.character,
            room_id: room_id.clone(),
        };

        locked.players.insert(player_id, player);
        locked.player_name_index.insert(name_key.clone(), player_id);
        locked.sessions.insert(player_id, tx.clone());
        locked
            .party_registry
            .ensure_solo_party_for_actor(&player_name, &active_campaign_id);

        // Scene Occupancy Join
        if let Some(room) = locked.rooms.get_mut(&room_id) {
            if let Some(runtime) = &mut room.scene_runtime {
                let mut join_pos = crate::scene::Position { x: 0, y: 0 };
                for (pos, tile) in &runtime.scene.grid.tiles {
                    if tile.tile_type == crate::scene::TileType::Floor {
                        join_pos = *pos;
                        break;
                    }
                }
                let actor_id = player_name.clone();
                let _ = runtime.scene.move_actor(&actor_id, join_pos);
                snapshot = Some(runtime.generate_snapshot());
            } else {
                snapshot = None;
            }
        } else {
            snapshot = None;
        }

        room_state = room_state_for_player_locked(&locked, player_id);

        join_recipients.extend(
            players_in_room_senders_locked(&locked, &room_id, Some(player_id))
                .into_iter()
                .collect::<Vec<_>>(),
        );
    };

    if let Some(snap) = snapshot {
        send_to_client(tx, ServerMessage::SceneSnapshot { snapshot: snap });
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

async fn handle_account_login(
    account_name: String,
    tx: &ClientTx,
    state: &SharedState,
    session: &mut SessionContext,
) {
    if session.account_handle.is_some() {
        send_to_client(
            tx,
            ServerMessage::Error {
                text: "Account already authenticated for this connection.".to_owned(),
            },
        );
        return;
    }

    let clean_name = account_name.trim();
    if clean_name.is_empty() {
        send_to_client(
            tx,
            ServerMessage::Error {
                text: "Usage: /login <account>".to_owned(),
            },
        );
        return;
    }

    if !is_valid_account_name(clean_name) {
        send_to_client(
            tx,
            ServerMessage::Error {
                text: "Account must be 3-20 chars and use letters, numbers, _ or -.".to_owned(),
            },
        );
        return;
    }

    let account_handle = account_handle(clean_name);
    {
        let mut locked = state.lock().await;
        let _ = locked.persistence.ensure_account_session(&account_handle);
    }

    session.account_handle = Some(account_handle);
    send_to_client(
        tx,
        ServerMessage::Info {
            text: "Account authenticated. Use /create to make a character or /play <name>."
                .to_owned(),
        },
    );
}

async fn handle_select_character(
    name: String,
    tx: &ClientTx,
    state: &SharedState,
    session: &mut SessionContext,
) {
    if session.player_id.is_some() {
        send_to_client(
            tx,
            ServerMessage::Error {
                text: "Character already authenticated for this connection.".to_owned(),
            },
        );
        return;
    }

    let Some(account_handle) = session.account_handle.clone() else {
        send_to_client(
            tx,
            ServerMessage::Error {
                text: "Login to an account first with /login <account>.".to_owned(),
            },
        );
        return;
    };

    let clean_name = name.trim();
    if clean_name.is_empty() {
        send_to_client(
            tx,
            ServerMessage::Error {
                text: "Usage: /play <name>".to_owned(),
            },
        );
        return;
    }

    let lookup_key = name_key(clean_name);
    let mut join_recipients: Vec<ClientTx> = Vec::new();
    let player_id: u64;
    let room_id: String;
    let player_name: String;
    let room_state: Option<ServerMessage>;
    let snapshot: Option<ttrpg_protocol::SceneSnapshot>;

    {
        let mut locked = state.lock().await;
        let active_campaign_id = locked.active_campaign.campaign_id.clone();
        let entry_scene_id = locked.active_campaign.entry_scene_id.clone();

        let persisted = match locked.persistence.validate_character_join(
            &account_handle,
            &lookup_key,
            &active_campaign_id,
        ) {
            Ok(record) => record,
            Err(JoinCampaignError::CharacterNotFound) => {
                send_to_client(
                    tx,
                    ServerMessage::Error {
                        text:
                            "Character does not exist for this account. Use /create to build one."
                                .to_owned(),
                    },
                );
                return;
            }
            Err(JoinCampaignError::AccountNotFound { .. }) => {
                send_to_client(
                    tx,
                    ServerMessage::Error {
                        text: "Account session is invalid. Re-login with /login <account>."
                            .to_owned(),
                    },
                );
                return;
            }
            Err(JoinCampaignError::OwnershipMismatch { .. }) => {
                send_to_client(
                    tx,
                    ServerMessage::Error {
                        text: "Character does not belong to the authenticated account.".to_owned(),
                    },
                );
                return;
            }
            Err(JoinCampaignError::LockedToOtherCampaign { locked_campaign_id }) => {
                send_to_client(
                    tx,
                    ServerMessage::Error {
                        text: format!(
                            "Character is locked to campaign '{}'. Ask an admin to unlock.",
                            locked_campaign_id
                        ),
                    },
                );
                return;
            }
            Err(JoinCampaignError::PersistFailed(details)) => {
                send_to_client(
                    tx,
                    ServerMessage::Error {
                        text: format!("Campaign join could not be persisted safely: {}", details),
                    },
                );
                return;
            }
        };

        let found_id = match locked.player_name_index.get(&lookup_key).copied() {
            Some(existing_id) => existing_id,
            None => {
                let new_player_id = locked.next_player_id;
                locked.next_player_id += 1;
                locked
                    .player_name_index
                    .insert(lookup_key.clone(), new_player_id);
                new_player_id
            }
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
        let default_room_id = resolve_entry_room_id(&locked.rooms, &entry_scene_id);
        let player = locked.players.entry(player_id).or_insert_with(|| Player {
            id: player_id,
            account_id: persisted.account_id.clone(),
            character: persisted.character.clone(),
            room_id: default_room_id.clone(),
        });

        room_id = player.room_id.clone();
        player_name = player.display_name().to_owned();
        locked
            .party_registry
            .ensure_solo_party_for_actor(&player_name, &active_campaign_id);

        // Scene Occupancy Join
        if let Some(room) = locked.rooms.get_mut(&room_id) {
            if let Some(runtime) = &mut room.scene_runtime {
                let mut join_pos = crate::scene::Position { x: 0, y: 0 };
                for (pos, tile) in &runtime.scene.grid.tiles {
                    if tile.tile_type == crate::scene::TileType::Floor {
                        join_pos = *pos;
                        break;
                    }
                }
                let actor_id = player_name.clone();
                let _ = runtime.scene.move_actor(&actor_id, join_pos);
                snapshot = Some(runtime.generate_snapshot());
            } else {
                snapshot = None;
            }
        } else {
            snapshot = None;
        }

        room_state = room_state_for_player_locked(&locked, player_id);
        join_recipients.extend(
            players_in_room_senders_locked(&locked, &room_id, Some(player_id))
                .into_iter()
                .collect::<Vec<_>>(),
        );
    };

    if let Some(snap) = snapshot {
        send_to_client(tx, ServerMessage::SceneSnapshot { snapshot: snap });
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
        "encounter" => {
            let Some(action) = parts.next() else {
                send_to_client(
                    tx,
                    ServerMessage::Error {
                        text: "Usage: encounter <start|end|status>".to_owned(),
                    },
                );
                return;
            };
            let action = action.to_lowercase();

            match action.as_str() {
                "start" => start_encounter(player_id, tx, state).await,
                "end" => end_encounter(player_id, tx, state).await,
                "status" => show_encounter_status(player_id, tx, state).await,
                _ => {
                    send_to_client(
                        tx,
                        ServerMessage::Error {
                            text: "Usage: encounter <start|end|status>".to_owned(),
                        },
                    );
                }
            }
        }
        "end_turn" => {
            end_encounter_turn(player_id, tx, state).await;
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
                    text: "Commands: look, encounter <start|end|status>, end_turn, go <dir>, say <msg>, who, sheet, help".to_owned(),
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

async fn start_encounter(player_id: u64, tx: &ClientTx, state: &SharedState) {
    let (recipients, message, timeout_schedule) = {
        let mut locked = state.lock().await;
        let Some(player) = locked.players.get(&player_id).cloned() else {
            send_to_client(
                tx,
                ServerMessage::Error {
                    text: "Unknown player session.".to_owned(),
                },
            );
            return;
        };

        let room_id = player.room_id.clone();
        if locked.active_encounters.contains_key(&room_id) {
            send_to_client(
                tx,
                ServerMessage::Error {
                    text: "An encounter is already active in this room.".to_owned(),
                },
            );
            return;
        }

        let actor_id = player.display_name().to_owned();
        let campaign_id = locked.active_campaign.campaign_id.clone();
        locked
            .party_registry
            .ensure_solo_party_for_actor(&actor_id, &campaign_id);
        let Some(party) = locked.party_registry.party_for_actor(&actor_id).cloned() else {
            send_to_client(
                tx,
                ServerMessage::Error {
                    text: "Unable to resolve party membership for encounter start.".to_owned(),
                },
            );
            return;
        };

        let scene_id = locked
            .rooms
            .get(&room_id)
            .and_then(|room| room.scene_runtime.as_ref())
            .map(|runtime| runtime.scene.id.clone())
            .unwrap_or_else(|| room_id.clone());
        let relevant_npcs = locked
            .rooms
            .get(&room_id)
            .map(relevant_npc_ids_for_room)
            .unwrap_or_default();
        let participants = capture_participants(&party, relevant_npcs.clone());
        let initiative_scores = participants
            .iter()
            .map(|participant| {
                (
                    participant.actor_id.clone(),
                    deterministic_initiative_for_actor(&participant.actor_id),
                )
            })
            .collect::<HashMap<_, _>>();

        let encounter_id = format!("encounter-{}", locked.next_encounter_id);
        locked.next_encounter_id += 1;

        let encounter = match EncounterState::start(
            encounter_id.clone(),
            scene_id,
            &actor_id,
            &party,
            relevant_npcs,
            initiative_scores,
        ) {
            Ok(encounter) => encounter,
            Err(error) => {
                send_to_client(
                    tx,
                    ServerMessage::Error {
                        text: format!("Failed to start encounter: {:?}", error),
                    },
                );
                return;
            }
        };

        let message = format!(
            "Encounter started: {}",
            format_encounter_status_line(&encounter)
        );
        let timeout_schedule =
            TurnTimerMarker::capture(room_id.clone(), &encounter).map(|marker| {
                (
                    marker,
                    locked.turn_timer_config.timeout(),
                    locked.timeout_fallback_action,
                )
            });
        locked.active_encounters.insert(room_id.clone(), encounter);
        let recipients = players_in_room_senders_locked(&locked, &room_id, None);
        (recipients, message, timeout_schedule)
    };

    for recipient in recipients {
        send_to_client(
            &recipient,
            ServerMessage::Info {
                text: message.clone(),
            },
        );
    }

    if let Some((marker, timeout, fallback_action)) = timeout_schedule {
        spawn_turn_timeout_task(Arc::clone(state), marker, timeout, fallback_action);
    }
}

async fn end_encounter(player_id: u64, tx: &ClientTx, state: &SharedState) {
    let (recipients, message) = {
        let mut locked = state.lock().await;
        let Some(player) = locked.players.get(&player_id) else {
            send_to_client(
                tx,
                ServerMessage::Error {
                    text: "Unknown player session.".to_owned(),
                },
            );
            return;
        };

        let room_id = player.room_id.clone();
        let actor_id = player.display_name().to_owned();
        let Some(mut encounter) = locked.active_encounters.remove(&room_id) else {
            send_to_client(
                tx,
                ServerMessage::Error {
                    text: "No active encounter in this room.".to_owned(),
                },
            );
            return;
        };

        if !encounter.is_participant(&actor_id) {
            locked.active_encounters.insert(room_id.clone(), encounter);
            send_to_client(
                tx,
                ServerMessage::Error {
                    text: "Only encounter participants can end the encounter.".to_owned(),
                },
            );
            return;
        }

        encounter.resolve();
        encounter.end();

        let recipients = players_in_room_senders_locked(&locked, &room_id, None);
        (
            recipients,
            format!(
                "Encounter {} ended by {}.",
                encounter.encounter_id, actor_id
            ),
        )
    };

    for recipient in recipients {
        send_to_client(
            &recipient,
            ServerMessage::Info {
                text: message.clone(),
            },
        );
    }
}

async fn end_encounter_turn(player_id: u64, tx: &ClientTx, state: &SharedState) {
    let (recipients, message, timeout_schedule) = {
        let mut locked = state.lock().await;
        let Some(player) = locked.players.get(&player_id) else {
            send_to_client(
                tx,
                ServerMessage::Error {
                    text: "Unknown player session.".to_owned(),
                },
            );
            return;
        };

        let room_id = player.room_id.clone();
        let actor_id = player.display_name().to_owned();
        let turn_timeout = locked.turn_timer_config.timeout();
        let fallback_action = locked.timeout_fallback_action;
        let (turn_message, timeout_schedule) = {
            let Some(encounter) = locked.active_encounters.get_mut(&room_id) else {
                send_to_client(
                    tx,
                    ServerMessage::Error {
                        text: "No active encounter in this room.".to_owned(),
                    },
                );
                return;
            };

            if !encounter.is_participant(&actor_id) {
                send_to_client(
                    tx,
                    ServerMessage::Error {
                        text: "You are not a participant in this encounter.".to_owned(),
                    },
                );
                return;
            }

            if !encounter.is_actor_turn(&actor_id) {
                let active_actor = encounter.active_actor().unwrap_or("unknown");
                send_to_client(
                    tx,
                    ServerMessage::Error {
                        text: format!("It is {}'s turn.", active_actor),
                    },
                );
                return;
            }

            let next_actor = encounter
                .advance_turn()
                .map(str::to_owned)
                .unwrap_or_else(|| "unknown".to_owned());
            let round = encounter.round;
            let timeout_schedule = TurnTimerMarker::capture(room_id.clone(), encounter)
                .map(|marker| (marker, turn_timeout, fallback_action));
            (
                format!(
                    "{} ends turn. Active turn: {} (round {}).",
                    actor_id, next_actor, round
                ),
                timeout_schedule,
            )
        };

        let recipients = players_in_room_senders_locked(&locked, &room_id, None);
        (recipients, turn_message, timeout_schedule)
    };

    for recipient in recipients {
        send_to_client(
            &recipient,
            ServerMessage::Info {
                text: message.clone(),
            },
        );
    }

    if let Some((marker, timeout, fallback_action)) = timeout_schedule {
        spawn_turn_timeout_task(Arc::clone(state), marker, timeout, fallback_action);
    }
}

fn spawn_turn_timeout_task(
    state: SharedState,
    marker: TurnTimerMarker,
    timeout: Duration,
    fallback_action: TimeoutFallbackAction,
) {
    tokio::spawn(async move {
        tokio::time::sleep(timeout).await;
        apply_turn_timeout_fallback(state, marker, fallback_action).await;
    });
}

async fn apply_turn_timeout_fallback(
    state: SharedState,
    marker: TurnTimerMarker,
    fallback_action: TimeoutFallbackAction,
) {
    let (recipients, message, next_timer) = {
        let mut locked = state.lock().await;
        let turn_timeout = locked.turn_timer_config.timeout();
        let timeout_fallback_action = locked.timeout_fallback_action;
        let (event, next_timer) = {
            let Some(encounter) = locked.active_encounters.get_mut(&marker.room_id) else {
                return;
            };

            let event = match resolve_timeout_fallback(encounter, &marker, fallback_action) {
                Ok(event) => event,
                Err(
                    TimeoutFallbackError::StaleTurnMarker
                    | TimeoutFallbackError::EncounterNotActive,
                ) => {
                    return;
                }
                Err(TimeoutFallbackError::MissingActiveActor) => {
                    error!(
                        "encounter timeout fallback failed: missing active actor for encounter {}",
                        encounter.encounter_id
                    );
                    return;
                }
            };
            let next_timer = TurnTimerMarker::capture(marker.room_id.clone(), encounter)
                .map(|next_marker| (next_marker, turn_timeout, timeout_fallback_action));
            (event, next_timer)
        };

        info!(
            "encounter timeout fallback applied: encounter={} room={} actor={} action={} prior_round={} prior_turn_index={} resulting_round={} next_actor={}",
            event.encounter_id,
            marker.room_id,
            event.actor_id,
            event.action.as_str(),
            event.prior_round,
            event.prior_turn_index,
            event.resulting_round,
            event.next_actor_id.as_deref().unwrap_or("none"),
        );

        let recipients = players_in_room_senders_locked(&locked, &marker.room_id, None);
        let message = format!(
            "Turn timeout: {} auto-resolves with {}. Active turn: {} (round {}).",
            event.actor_id,
            event.action.as_str(),
            event.next_actor_id.as_deref().unwrap_or("none"),
            event.resulting_round,
        );
        (recipients, message, next_timer)
    };

    for recipient in recipients {
        send_to_client(
            &recipient,
            ServerMessage::Info {
                text: message.clone(),
            },
        );
    }

    if let Some((next_marker, timeout, next_fallback_action)) = next_timer {
        spawn_turn_timeout_task(state, next_marker, timeout, next_fallback_action);
    }
}

async fn show_encounter_status(player_id: u64, tx: &ClientTx, state: &SharedState) {
    let text = {
        let locked = state.lock().await;
        if let Some(player) = locked.players.get(&player_id) {
            match locked.active_encounters.get(&player.room_id) {
                Some(encounter) => {
                    format!(
                        "Encounter status: {}",
                        format_encounter_status_line(encounter)
                    )
                }
                None => "No active encounter in this room.".to_owned(),
            }
        } else {
            "Unknown player session.".to_owned()
        }
    };

    send_to_client(tx, ServerMessage::Info { text });
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
                "Character Sheet\nAccount: {}\nName: {}\nAncestry: {}\nClass: {}\nBackground: {}\nPronouns: {}\nMotto: {}",
                player.account_id.as_str(),
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

    let (player_name, old_room, new_room, leave_recipients, join_recipients, room_view, snapshot) = {
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

        if let Some(encounter) = locked.active_encounters.get(&old_room) {
            if encounter.is_participant(player_snapshot.display_name()) {
                send_to_client(
                    tx,
                    ServerMessage::Error {
                        text: "Cannot leave room during an active encounter.".to_owned(),
                    },
                );
                return;
            }
        }

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

        // Scene Occupancy Cleanup (Old Room)
        if let Some(room) = locked.rooms.get_mut(&old_room) {
            if let Some(runtime) = &mut room.scene_runtime {
                runtime.scene.remove_actor(&player_snapshot.display_name());
            }
        }

        // Scene Occupancy Join (New Room)
        let mut snapshot = None;
        if let Some(room) = locked.rooms.get_mut(&new_room) {
            if let Some(runtime) = &mut room.scene_runtime {
                // For V1, we join at (0,0) or nearest floor.
                // Let's find first floor tile.
                let mut join_pos = crate::scene::Position { x: 0, y: 0 };
                for (pos, tile) in &runtime.scene.grid.tiles {
                    if tile.tile_type == crate::scene::TileType::Floor {
                        join_pos = *pos;
                        break;
                    }
                }
                let actor_id = player_snapshot.display_name().to_owned();
                let _ = runtime.scene.move_actor(&actor_id, join_pos);
                snapshot = Some(runtime.generate_snapshot());
            }
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
            snapshot,
        )
    };

    if let Some(snap) = snapshot {
        send_to_client(tx, ServerMessage::SceneSnapshot { snapshot: snap });
    }

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
    session.account_handle.take();

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

fn format_encounter_status_line(encounter: &EncounterState) -> String {
    let participants = encounter
        .participants
        .iter()
        .map(|participant| participant.actor_id.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let initiative_order = encounter.initiative_order.join(" -> ");
    let active_turn = encounter.active_actor().unwrap_or("none");

    format!(
        "{} | participants: [{}] | initiative: [{}] | round: {} | active_turn: {}",
        encounter.encounter_id, participants, initiative_order, encounter.round, active_turn
    )
}

fn relevant_npc_ids_for_room(room: &Room) -> Vec<String> {
    let mut actor_ids = room
        .scene_runtime
        .as_ref()
        .map(|runtime| {
            runtime
                .scene
                .occupants
                .values()
                .filter(|actor_id| is_npc_actor_id(actor_id))
                .cloned()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    actor_ids.sort();
    actor_ids.dedup();
    actor_ids
}

fn is_npc_actor_id(actor_id: &str) -> bool {
    actor_id.starts_with("npc:")
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

    let mut town_square_tiles = HashMap::new();
    for x in 0..10 {
        for y in 0..10 {
            town_square_tiles.insert(
                Position { x, y },
                Tile {
                    tile_type: if (x == 5 && y == 5) || (x == 4 && y == 5) {
                        TileType::Wall
                    } else {
                        TileType::Floor
                    },
                },
            );
        }
    }

    let town_square_scene = Scene {
        id: "town_square_scene".to_owned(),
        grid: Grid {
            width: 10,
            height: 10,
            tiles: town_square_tiles,
        },
        occupants: HashMap::new(),
    };

    rooms.insert(
        "town_square".to_owned(),
        Room {
            id: "town_square".to_owned(),
            name: "Town Square".to_owned(),
            description: "A busy square with a fountain and a quest board.".to_owned(),
            exits: town_square_exits,
            scene_runtime: Some(SceneRuntime::new(town_square_scene)),
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
            scene_runtime: None,
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
            scene_runtime: None,
        },
    );

    rooms
}

fn resolve_entry_room_id(rooms: &HashMap<String, Room>, preferred_room_id: &str) -> String {
    if rooms.contains_key(preferred_room_id) {
        return preferred_room_id.to_owned();
    }

    "town_square".to_owned()
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

fn is_valid_account_name(name: &str) -> bool {
    is_valid_name(name)
}

fn account_handle(name: &str) -> String {
    format!("acct:{}", name_key(name))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::party::Party;
    use tokio::sync::mpsc;
    use ttrpg_protocol::{SceneCommand, SceneDelta, ScenePosition};

    fn test_campaign() -> CampaignManifestEntry {
        CampaignManifestEntry {
            campaign_id: "test-campaign".to_owned(),
            name: "Test Campaign".to_owned(),
            content_version: "v1".to_owned(),
            entry_scene_id: "town_square".to_owned(),
            save_path: "saves/test.json".to_owned(),
        }
    }

    fn test_character(name: &str) -> CharacterDraft {
        CharacterDraft {
            name: name.to_owned(),
            ancestry: "Human".to_owned(),
            class_name: "Fighter".to_owned(),
            background: "Soldier".to_owned(),
            pronouns: "they/them".to_owned(),
            motto: "Hold the line".to_owned(),
        }
    }

    fn collect_messages(rx: &mut mpsc::UnboundedReceiver<ServerMessage>) -> Vec<ServerMessage> {
        let mut out = Vec::new();
        while let Ok(message) = rx.try_recv() {
            out.push(message);
        }
        out
    }

    fn has_actor_moved_delta(
        messages: &[ServerMessage],
        actor_id: &str,
        from: Option<(i32, i32)>,
        to: (i32, i32),
    ) -> bool {
        messages.iter().any(|message| {
            let ServerMessage::SceneDelta { deltas } = message else {
                return false;
            };

            deltas.iter().any(|delta| {
                let SceneDelta::ActorMoved {
                    actor_id: delta_actor_id,
                    from: delta_from,
                    to: delta_to,
                } = delta
                else {
                    return false;
                };

                delta_actor_id == actor_id
                    && delta_from.as_ref().map(|p| (p.x, p.y)) == from
                    && (delta_to.x, delta_to.y) == to
            })
        })
    }

    fn has_error_message_containing(messages: &[ServerMessage], needle: &str) -> bool {
        messages.iter().any(|message| match message {
            ServerMessage::Error { text } => text.contains(needle),
            _ => false,
        })
    }

    fn has_info_message_containing(messages: &[ServerMessage], needle: &str) -> bool {
        messages.iter().any(|message| match message {
            ServerMessage::Info { text } => text.contains(needle),
            _ => false,
        })
    }

    async fn seed_player(
        state: &SharedState,
        player_id: u64,
        account_id: &str,
        name: &str,
        room_id: &str,
        tx: &ClientTx,
        pos: Position,
    ) {
        let mut locked = state.lock().await;
        locked.players.insert(
            player_id,
            Player {
                id: player_id,
                account_id: account_id.to_owned(),
                character: test_character(name),
                room_id: room_id.to_owned(),
            },
        );
        locked.sessions.insert(player_id, tx.clone());

        if let Some(room) = locked.rooms.get_mut(room_id) {
            if let Some(runtime) = room.scene_runtime.as_mut() {
                runtime
                    .scene
                    .move_actor(name, pos)
                    .expect("actor placement should succeed");
            }
        }
    }

    #[tokio::test]
    async fn scene_action_emits_delta_to_players_in_room() {
        let state = Arc::new(Mutex::new(ServerState::new_for_tests(test_campaign())));
        let (tx_actor, mut rx_actor) = mpsc::unbounded_channel::<ServerMessage>();
        let (tx_peer, mut rx_peer) = mpsc::unbounded_channel::<ServerMessage>();

        {
            let mut locked = state.lock().await;
            locked.players.insert(
                1,
                Player {
                    id: 1,
                    account_id: "acct:alpha".to_owned(),
                    character: test_character("Alpha"),
                    room_id: "town_square".to_owned(),
                },
            );
            locked.players.insert(
                2,
                Player {
                    id: 2,
                    account_id: "acct:bravo".to_owned(),
                    character: test_character("Bravo"),
                    room_id: "town_square".to_owned(),
                },
            );
            locked.sessions.insert(1, tx_actor.clone());
            locked.sessions.insert(2, tx_peer.clone());

            let room = locked
                .rooms
                .get_mut("town_square")
                .expect("town_square room should exist");
            let runtime = room
                .scene_runtime
                .as_mut()
                .expect("town_square should have scene runtime");
            runtime
                .scene
                .move_actor("Alpha", Position { x: 0, y: 0 })
                .expect("initial actor placement should succeed");
            runtime
                .scene
                .move_actor("Bravo", Position { x: 1, y: 0 })
                .expect("initial peer placement should succeed");
        }

        let mut session = SessionContext {
            account_handle: Some("acct:alpha".to_owned()),
            player_id: Some(1),
        };

        process_client_message(
            ClientMessage::SceneAction {
                command: SceneCommand::Move {
                    target_pos: ScenePosition { x: 0, y: 1 },
                },
            },
            &tx_actor,
            &state,
            &mut session,
        )
        .await;

        let actor_messages = collect_messages(&mut rx_actor);
        let peer_messages = collect_messages(&mut rx_peer);

        assert!(has_actor_moved_delta(
            &actor_messages,
            "Alpha",
            Some((0, 0)),
            (0, 1)
        ));
        assert!(has_actor_moved_delta(
            &peer_messages,
            "Alpha",
            Some((0, 0)),
            (0, 1)
        ));
        assert!(actor_messages.iter().any(|message| {
            matches!(
                message,
                ServerMessage::Info { text } if text == "Scene action processed."
            )
        }));
    }

    #[tokio::test]
    async fn encounter_start_captures_only_party_members_plus_relevant_npcs() {
        let state = Arc::new(Mutex::new(ServerState::new_for_tests(test_campaign())));
        let (tx_alpha, mut rx_alpha) = mpsc::unbounded_channel::<ServerMessage>();
        let (tx_bravo, _rx_bravo) = mpsc::unbounded_channel::<ServerMessage>();
        let (tx_charlie, _rx_charlie) = mpsc::unbounded_channel::<ServerMessage>();

        seed_player(
            &state,
            1,
            "acct:alpha",
            "Alpha",
            "town_square",
            &tx_alpha,
            Position { x: 0, y: 0 },
        )
        .await;
        seed_player(
            &state,
            2,
            "acct:bravo",
            "Bravo",
            "town_square",
            &tx_bravo,
            Position { x: 1, y: 0 },
        )
        .await;
        seed_player(
            &state,
            3,
            "acct:charlie",
            "Charlie",
            "town_square",
            &tx_charlie,
            Position { x: 2, y: 0 },
        )
        .await;

        {
            let mut locked = state.lock().await;
            let party = Party::new(
                "party:alpha",
                "test-campaign",
                "Alpha",
                vec!["Bravo".to_owned(), "Alpha".to_owned()],
            )
            .expect("party should be valid");
            locked.party_registry.upsert_party(party);
            locked
                .party_registry
                .ensure_solo_party_for_actor("Charlie", "test-campaign");

            let room = locked
                .rooms
                .get_mut("town_square")
                .expect("town_square should exist");
            let runtime = room
                .scene_runtime
                .as_mut()
                .expect("town_square should have scene runtime");
            runtime
                .scene
                .move_actor("npc:wolf", Position { x: 3, y: 0 })
                .expect("npc placement should succeed");
        }

        start_encounter(1, &tx_alpha, &state).await;

        let participants = {
            let locked = state.lock().await;
            locked
                .active_encounters
                .get("town_square")
                .expect("encounter should be active")
                .participants
                .iter()
                .map(|participant| participant.actor_id.clone())
                .collect::<Vec<_>>()
        };

        assert_eq!(
            participants,
            vec![
                "Alpha".to_owned(),
                "Bravo".to_owned(),
                "npc:wolf".to_owned()
            ]
        );
        assert!(!participants.iter().any(|actor_id| actor_id == "Charlie"));

        let actor_messages = collect_messages(&mut rx_alpha);
        assert!(actor_messages.iter().any(|message| {
            matches!(message, ServerMessage::Info { text } if text.contains("Encounter started:"))
        }));
    }

    #[tokio::test]
    async fn scene_action_is_rejected_when_actor_is_out_of_turn() {
        let state = Arc::new(Mutex::new(ServerState::new_for_tests(test_campaign())));
        let (tx_alpha, mut rx_alpha) = mpsc::unbounded_channel::<ServerMessage>();
        let (tx_bravo, mut rx_bravo) = mpsc::unbounded_channel::<ServerMessage>();

        seed_player(
            &state,
            1,
            "acct:alpha",
            "Alpha",
            "town_square",
            &tx_alpha,
            Position { x: 0, y: 0 },
        )
        .await;
        seed_player(
            &state,
            2,
            "acct:bravo",
            "Bravo",
            "town_square",
            &tx_bravo,
            Position { x: 1, y: 0 },
        )
        .await;

        {
            let mut locked = state.lock().await;
            let party = Party::new(
                "party:duo",
                "test-campaign",
                "Alpha",
                vec!["Alpha".to_owned(), "Bravo".to_owned()],
            )
            .expect("party should be valid");
            locked.party_registry.upsert_party(party);
        }

        start_encounter(1, &tx_alpha, &state).await;

        // Drop start encounter broadcast chatter to make assertions focused.
        let _ = collect_messages(&mut rx_alpha);
        let _ = collect_messages(&mut rx_bravo);

        let active_actor = {
            let locked = state.lock().await;
            locked
                .active_encounters
                .get("town_square")
                .expect("encounter should be active")
                .active_actor()
                .expect("active actor should exist")
                .to_owned()
        };

        let (out_of_turn_player_id, out_of_turn_tx, out_of_turn_rx) = if active_actor == "Alpha" {
            (2_u64, tx_bravo.clone(), &mut rx_bravo)
        } else {
            (1_u64, tx_alpha.clone(), &mut rx_alpha)
        };

        let mut session = SessionContext {
            account_handle: Some("acct:test".to_owned()),
            player_id: Some(out_of_turn_player_id),
        };

        process_client_message(
            ClientMessage::SceneAction {
                command: SceneCommand::Move {
                    target_pos: ScenePosition { x: 0, y: 1 },
                },
            },
            &out_of_turn_tx,
            &state,
            &mut session,
        )
        .await;

        let out_of_turn_messages = collect_messages(out_of_turn_rx);
        assert!(has_error_message_containing(&out_of_turn_messages, "It is"));
    }

    #[tokio::test]
    async fn end_turn_advances_to_next_actor() {
        let state = Arc::new(Mutex::new(ServerState::new_for_tests(test_campaign())));
        let (tx_alpha, mut rx_alpha) = mpsc::unbounded_channel::<ServerMessage>();
        let (tx_bravo, mut rx_bravo) = mpsc::unbounded_channel::<ServerMessage>();

        seed_player(
            &state,
            1,
            "acct:alpha",
            "Alpha",
            "town_square",
            &tx_alpha,
            Position { x: 0, y: 0 },
        )
        .await;
        seed_player(
            &state,
            2,
            "acct:bravo",
            "Bravo",
            "town_square",
            &tx_bravo,
            Position { x: 1, y: 0 },
        )
        .await;

        {
            let mut locked = state.lock().await;
            let party = Party::new(
                "party:duo",
                "test-campaign",
                "Alpha",
                vec!["Alpha".to_owned(), "Bravo".to_owned()],
            )
            .expect("party should be valid");
            locked.party_registry.upsert_party(party);
        }

        start_encounter(1, &tx_alpha, &state).await;
        let _ = collect_messages(&mut rx_alpha);
        let _ = collect_messages(&mut rx_bravo);

        let (active_actor_before, active_player_id, active_player_tx) = {
            let locked = state.lock().await;
            let encounter = locked
                .active_encounters
                .get("town_square")
                .expect("encounter should be active");
            let active_actor = encounter.active_actor().expect("active actor should exist");
            if active_actor == "Alpha" {
                (active_actor.to_owned(), 1_u64, tx_alpha.clone())
            } else {
                (active_actor.to_owned(), 2_u64, tx_bravo.clone())
            }
        };

        end_encounter_turn(active_player_id, &active_player_tx, &state).await;

        let active_actor_after = {
            let locked = state.lock().await;
            locked
                .active_encounters
                .get("town_square")
                .expect("encounter should stay active")
                .active_actor()
                .expect("active actor should exist")
                .to_owned()
        };

        assert_ne!(active_actor_before, active_actor_after);

        let actor_messages = if active_player_id == 1 {
            collect_messages(&mut rx_alpha)
        } else {
            collect_messages(&mut rx_bravo)
        };
        assert!(actor_messages.iter().any(|message| {
            matches!(message, ServerMessage::Info { text } if text.contains("ends turn"))
        }));
    }

    #[tokio::test]
    async fn timeout_fallback_advances_turn_when_actor_does_not_end_turn() {
        let state = Arc::new(Mutex::new(ServerState::new_for_tests(test_campaign())));
        let (tx_alpha, mut rx_alpha) = mpsc::unbounded_channel::<ServerMessage>();
        let (tx_bravo, mut rx_bravo) = mpsc::unbounded_channel::<ServerMessage>();

        seed_player(
            &state,
            1,
            "acct:alpha",
            "Alpha",
            "town_square",
            &tx_alpha,
            Position { x: 0, y: 0 },
        )
        .await;
        seed_player(
            &state,
            2,
            "acct:bravo",
            "Bravo",
            "town_square",
            &tx_bravo,
            Position { x: 1, y: 0 },
        )
        .await;

        {
            let mut locked = state.lock().await;
            let party = Party::new(
                "party:duo",
                "test-campaign",
                "Alpha",
                vec!["Alpha".to_owned(), "Bravo".to_owned()],
            )
            .expect("party should be valid");
            locked.party_registry.upsert_party(party);
            locked.turn_timer_config = TurnTimerConfig::from_millis(5);
        }

        start_encounter(1, &tx_alpha, &state).await;
        let _ = collect_messages(&mut rx_alpha);
        let _ = collect_messages(&mut rx_bravo);
        let active_actor_before_timeout = {
            let locked = state.lock().await;
            locked
                .active_encounters
                .get("town_square")
                .expect("encounter should still be active")
                .active_actor()
                .expect("active actor should exist")
                .to_owned()
        };

        let mut active_actor_after_timeout = active_actor_before_timeout.clone();
        for _ in 0..20 {
            tokio::time::sleep(Duration::from_millis(10)).await;
            active_actor_after_timeout = {
                let locked = state.lock().await;
                locked
                    .active_encounters
                    .get("town_square")
                    .expect("encounter should still be active")
                    .active_actor()
                    .expect("active actor should exist")
                    .to_owned()
            };
            if active_actor_after_timeout != active_actor_before_timeout {
                break;
            }
        }

        assert_ne!(active_actor_after_timeout, active_actor_before_timeout);

        let alpha_messages = collect_messages(&mut rx_alpha);
        let bravo_messages = collect_messages(&mut rx_bravo);
        assert!(has_info_message_containing(
            &alpha_messages,
            "Turn timeout:"
        ));
        assert!(has_info_message_containing(
            &bravo_messages,
            "Turn timeout:"
        ));
    }

    #[tokio::test]
    async fn timeout_fallback_does_not_double_advance_after_manual_end_turn() {
        let state = Arc::new(Mutex::new(ServerState::new_for_tests(test_campaign())));
        let (tx_alpha, mut rx_alpha) = mpsc::unbounded_channel::<ServerMessage>();
        let (tx_bravo, mut rx_bravo) = mpsc::unbounded_channel::<ServerMessage>();

        seed_player(
            &state,
            1,
            "acct:alpha",
            "Alpha",
            "town_square",
            &tx_alpha,
            Position { x: 0, y: 0 },
        )
        .await;
        seed_player(
            &state,
            2,
            "acct:bravo",
            "Bravo",
            "town_square",
            &tx_bravo,
            Position { x: 1, y: 0 },
        )
        .await;

        {
            let mut locked = state.lock().await;
            let party = Party::new(
                "party:duo",
                "test-campaign",
                "Alpha",
                vec!["Alpha".to_owned(), "Bravo".to_owned()],
            )
            .expect("party should be valid");
            locked.party_registry.upsert_party(party);
            locked.turn_timer_config = TurnTimerConfig::from_millis(25);
        }

        start_encounter(1, &tx_alpha, &state).await;
        let _ = collect_messages(&mut rx_alpha);
        let _ = collect_messages(&mut rx_bravo);

        let (active_player_id, active_player_tx, ended_actor_name) = {
            let locked = state.lock().await;
            let encounter = locked
                .active_encounters
                .get("town_square")
                .expect("encounter should be active");
            let active_actor = encounter.active_actor().expect("active actor should exist");
            if active_actor == "Alpha" {
                (1_u64, tx_alpha.clone(), "Alpha".to_owned())
            } else {
                (2_u64, tx_bravo.clone(), "Bravo".to_owned())
            }
        };

        end_encounter_turn(active_player_id, &active_player_tx, &state).await;
        let _ = collect_messages(&mut rx_alpha);
        let _ = collect_messages(&mut rx_bravo);

        let active_actor_after_manual_end = {
            let locked = state.lock().await;
            locked
                .active_encounters
                .get("town_square")
                .expect("encounter should still be active")
                .active_actor()
                .expect("active actor should exist")
                .to_owned()
        };
        assert_ne!(active_actor_after_manual_end, ended_actor_name);

        let mut active_actor_after_timer = active_actor_after_manual_end.clone();
        for _ in 0..20 {
            tokio::time::sleep(Duration::from_millis(10)).await;
            active_actor_after_timer = {
                let locked = state.lock().await;
                locked
                    .active_encounters
                    .get("town_square")
                    .expect("encounter should still be active")
                    .active_actor()
                    .expect("active actor should exist")
                    .to_owned()
            };
            if active_actor_after_timer != active_actor_after_manual_end {
                break;
            }
        }

        assert_ne!(active_actor_after_timer, active_actor_after_manual_end);
    }
}
