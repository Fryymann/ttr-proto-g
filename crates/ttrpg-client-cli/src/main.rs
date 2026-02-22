use std::env;
use std::io::Write;

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;

mod render_scene;

use render_scene::SceneRenderer;
use ttrpg_protocol::{to_json_line, CharacterDraft, ClientMessage, ServerMessage};

type StdinLines = tokio::io::Lines<BufReader<io::Stdin>>;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = env::args()
        .nth(1)
        .or_else(|| env::var("TTRPG_SERVER_ADDR").ok())
        .unwrap_or_else(|| "127.0.0.1:7000".to_owned());

    let stream = TcpStream::connect(&addr).await?;
    let (reader, mut writer) = stream.into_split();

    render_welcome_screen(&addr);

    let read_task = tokio::spawn(async move {
        let mut scene_renderer = SceneRenderer::new();
        let mut lines = BufReader::new(reader).lines();

        while let Ok(Some(line)) = lines.next_line().await {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            match serde_json::from_str::<ServerMessage>(line) {
                Ok(message) => render_server_message(message, &mut scene_renderer),
                Err(_) => println!("[raw] {}", line),
            }
        }
    });

    let stdin = io::stdin();
    let mut stdin_lines = BufReader::new(stdin).lines();

    while let Some(input) = stdin_lines.next_line().await? {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed == "/create" {
            run_character_editor(&mut stdin_lines, &mut writer).await?;
            continue;
        }

        let outbound = if let Some(name) = trimmed.strip_prefix("/create ") {
            ClientMessage::CreateCharacter {
                character: quick_character_from_name(name),
            }
        } else if let Some(name) = trimmed.strip_prefix("/play ") {
            ClientMessage::SelectCharacter {
                name: name.trim().to_owned(),
            }
        } else if let Some(name) = trimmed.strip_prefix("/login ") {
            ClientMessage::Login {
                name: name.trim().to_owned(),
            }
        } else if trimmed == "/ping" {
            ClientMessage::Ping
        } else {
            ClientMessage::CommandText {
                command: trimmed.to_owned(),
            }
        };

        send_message(&mut writer, outbound).await?;
    }

    let _ = read_task.await;
    Ok(())
}

async fn run_character_editor(
    stdin_lines: &mut StdinLines,
    writer: &mut OwnedWriteHalf,
) -> io::Result<()> {
    println!("\n\x1b[1;34mCharacter Editor\x1b[0m");
    println!("Press Enter to keep defaults.");

    let name = prompt_with_default(stdin_lines, "Name", "Adventurer").await?;
    let ancestry = prompt_choice(
        stdin_lines,
        "Ancestry",
        &[
            "Human",
            "Elf",
            "Dwarf",
            "Halfling",
            "Tiefling",
            "Dragonborn",
        ],
        0,
    )
    .await?;
    let class_name = prompt_choice(
        stdin_lines,
        "Class",
        &["Fighter", "Rogue", "Wizard", "Cleric", "Bard", "Warlock"],
        0,
    )
    .await?;
    let background = prompt_choice(
        stdin_lines,
        "Background",
        &[
            "Soldier",
            "Acolyte",
            "Sage",
            "Criminal",
            "Artisan",
            "Outlander",
        ],
        0,
    )
    .await?;
    let pronouns = prompt_with_default(stdin_lines, "Pronouns", "they/them").await?;
    let motto = prompt_with_default(stdin_lines, "Motto", "Fortune favors the bold.").await?;

    let character = CharacterDraft {
        name,
        ancestry,
        class_name,
        background,
        pronouns,
        motto,
    };

    println!("\n\x1b[1;36mPreview\x1b[0m");
    println!("Name: {}", character.name);
    println!("Ancestry: {}", character.ancestry);
    println!("Class: {}", character.class_name);
    println!("Background: {}", character.background);
    println!("Pronouns: {}", character.pronouns);
    println!("Motto: {}", character.motto);

    if !prompt_yes_no(stdin_lines, "Create this character", true).await? {
        println!("Character creation canceled.");
        return Ok(());
    }

    send_message(writer, ClientMessage::CreateCharacter { character }).await?;
    Ok(())
}

async fn prompt_with_default(
    stdin_lines: &mut StdinLines,
    label: &str,
    default: &str,
) -> io::Result<String> {
    print!("{} [{}]: ", label, default);
    let _ = std::io::stdout().flush();

    let value = match stdin_lines.next_line().await? {
        Some(input) => {
            let trimmed = input.trim();
            if trimmed.is_empty() {
                default.to_owned()
            } else {
                trimmed.to_owned()
            }
        }
        None => default.to_owned(),
    };

    Ok(value)
}

async fn prompt_choice(
    stdin_lines: &mut StdinLines,
    label: &str,
    options: &[&str],
    default_idx: usize,
) -> io::Result<String> {
    println!("{}:", label);
    for (index, option) in options.iter().enumerate() {
        println!("  {}. {}", index + 1, option);
    }

    loop {
        print!("Select {} [{}]: ", label.to_lowercase(), default_idx + 1);
        let _ = std::io::stdout().flush();

        let Some(input) = stdin_lines.next_line().await? else {
            return Ok(options[default_idx].to_owned());
        };

        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Ok(options[default_idx].to_owned());
        }

        let Ok(parsed) = trimmed.parse::<usize>() else {
            println!("Please enter a number between 1 and {}.", options.len());
            continue;
        };

        if parsed == 0 || parsed > options.len() {
            println!("Please enter a number between 1 and {}.", options.len());
            continue;
        }

        return Ok(options[parsed - 1].to_owned());
    }
}

async fn prompt_yes_no(
    stdin_lines: &mut StdinLines,
    label: &str,
    default_yes: bool,
) -> io::Result<bool> {
    let default_hint = if default_yes { "Y/n" } else { "y/N" };

    loop {
        print!("{} [{}]: ", label, default_hint);
        let _ = std::io::stdout().flush();

        let Some(input) = stdin_lines.next_line().await? else {
            return Ok(default_yes);
        };

        let trimmed = input.trim().to_lowercase();
        if trimmed.is_empty() {
            return Ok(default_yes);
        }

        if trimmed == "y" || trimmed == "yes" {
            return Ok(true);
        }

        if trimmed == "n" || trimmed == "no" {
            return Ok(false);
        }

        println!("Please answer y or n.");
    }
}

fn quick_character_from_name(name: &str) -> CharacterDraft {
    CharacterDraft {
        name: name.trim().to_owned(),
        ancestry: "Human".to_owned(),
        class_name: "Fighter".to_owned(),
        background: "Soldier".to_owned(),
        pronouns: "they/them".to_owned(),
        motto: "Fortune favors the bold.".to_owned(),
    }
}

async fn send_message(writer: &mut OwnedWriteHalf, outbound: ClientMessage) -> io::Result<()> {
    let line = match to_json_line(&outbound) {
        Ok(line) => line,
        Err(err) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to encode outbound message: {}", err),
            ));
        }
    };

    writer.write_all(line.as_bytes()).await
}

fn render_welcome_screen(addr: &str) {
    println!("\x1b[2J\x1b[H");
    println!("\x1b[1;36m╔══════════════════════════════════════════════════════╗\x1b[0m");
    println!("\x1b[1;36m║\x1b[0m  \x1b[1;33m⚔️  TTRPG: Echoes of the Shattered Realm  🛡️\x1b[0m     \x1b[1;36m║\x1b[0m");
    println!("\x1b[1;36m╠══════════════════════════════════════════════════════╣\x1b[0m");
    println!(
        "\x1b[1;36m║\x1b[0m  Connected to: {:<36} \x1b[1;36m║\x1b[0m",
        addr
    );
    println!(
        "\x1b[1;36m║\x1b[0m  Create hero:  /create (opens editor)              \x1b[1;36m║\x1b[0m"
    );
    println!(
        "\x1b[1;36m║\x1b[0m  Quick create:  /create <name>                      \x1b[1;36m║\x1b[0m"
    );
    println!(
        "\x1b[1;36m║\x1b[0m  Account:      /login <account>                     \x1b[1;36m║\x1b[0m"
    );
    println!(
        "\x1b[1;36m║\x1b[0m  Play hero:    /play <name>                         \x1b[1;36m║\x1b[0m"
    );
    println!(
        "\x1b[1;36m║\x1b[0m  In game:      look, go <dir>, say <msg>, who       \x1b[1;36m║\x1b[0m"
    );
    println!("\x1b[1;36m╚══════════════════════════════════════════════════════╝\x1b[0m");
    println!();
}

fn render_server_message(message: ServerMessage, scene_renderer: &mut SceneRenderer) {
    match message {
        ServerMessage::AuthOk { player_id, name } => {
            scene_renderer.remember_local_player_name(&name);
            println!("✅ Logged in as {} (id {}).", name, player_id);
        }
        ServerMessage::RoomState {
            room_id,
            room_name,
            description,
            exits,
            players,
        } => {
            scene_renderer.update_room_players(&players);

            let exits_text = if exits.is_empty() {
                "none".to_owned()
            } else {
                exits.join(", ")
            };

            let players_text = if players.is_empty() {
                "nobody".to_owned()
            } else {
                players.join(", ")
            };

            println!("\n🧭 {} ({})", room_name, room_id);
            println!("{}", description);
            println!("Exits: {}", exits_text);
            println!("Players: {}", players_text);
        }
        ServerMessage::ChatMsg {
            from,
            channel,
            text,
        } => {
            println!("[{}] {}: {}", channel, from, text);
        }
        ServerMessage::WhoList { players } => {
            scene_renderer.update_room_players(&players);

            if players.is_empty() {
                println!("👥 No one is online.");
            } else {
                println!("👥 Online: {}", players.join(", "));
            }
        }
        ServerMessage::Prompt { text } => {
            print!("{} ", text);
            let _ = std::io::stdout().flush();
        }
        ServerMessage::Info { text } => {
            println!("ℹ️ {}", text);
        }
        ServerMessage::Error { text } => {
            eprintln!("⚠️ {}", text);
        }
        ServerMessage::Pong => {
            println!("🏓 pong");
        }
        ServerMessage::SceneSnapshot { snapshot } => {
            println!("{}", scene_renderer.apply_snapshot(snapshot));
        }
        ServerMessage::SceneDelta { deltas } => {
            println!("{}", scene_renderer.apply_deltas(deltas));
        }
    }
}
