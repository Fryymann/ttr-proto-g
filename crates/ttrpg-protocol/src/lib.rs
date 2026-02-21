use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterDraft {
    pub name: String,
    pub ancestry: String,
    pub class_name: String,
    pub background: String,
    pub pronouns: String,
    pub motto: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    Login { name: String },
    CreateCharacter { character: CharacterDraft },
    CommandText { command: String },
    Ping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    AuthOk {
        player_id: u64,
        name: String,
    },
    RoomState {
        room_id: String,
        room_name: String,
        description: String,
        exits: Vec<String>,
        players: Vec<String>,
    },
    ChatMsg {
        from: String,
        channel: String,
        text: String,
    },
    WhoList {
        players: Vec<String>,
    },
    Prompt {
        text: String,
    },
    Info {
        text: String,
    },
    Error {
        text: String,
    },
    Pong,
}

pub fn to_json_line<T: Serialize>(message: &T) -> Result<String, serde_json::Error> {
    let mut line = serde_json::to_string(message)?;
    line.push('\n');
    Ok(line)
}
