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
    SelectCharacter { name: String },
    CreateCharacter { character: CharacterDraft },
    CommandText { command: String },
    Ping,
    SceneAction { command: SceneCommand },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScenePosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SceneTileType {
    Floor,
    Wall,
    DifficultTerrain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneSnapshot {
    pub scene_id: String,
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<(ScenePosition, SceneTileType)>,
    pub occupants: Vec<(ScenePosition, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SceneDelta {
    ActorMoved {
        actor_id: String,
        from: Option<ScenePosition>,
        to: ScenePosition,
    },
    TileChanged {
        pos: ScenePosition,
        new_type: SceneTileType,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SceneCommand {
    Move { target_pos: ScenePosition },
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
    SceneSnapshot {
        snapshot: SceneSnapshot,
    },
    SceneDelta {
        deltas: Vec<SceneDelta>,
    },
}

pub fn to_json_line<T: Serialize>(message: &T) -> Result<String, serde_json::Error> {
    let mut line = serde_json::to_string(message)?;
    line.push('\n');
    Ok(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scene_snapshot_round_trips_via_server_message() {
        let message = ServerMessage::SceneSnapshot {
            snapshot: SceneSnapshot {
                scene_id: "town_square_scene".to_owned(),
                width: 10,
                height: 10,
                tiles: vec![(ScenePosition { x: 0, y: 0 }, SceneTileType::Floor)],
                occupants: vec![(ScenePosition { x: 1, y: 1 }, "Alpha".to_owned())],
            },
        };

        let encoded = to_json_line(&message).expect("scene snapshot should serialize");
        let decoded: ServerMessage =
            serde_json::from_str(encoded.trim()).expect("scene snapshot should deserialize");

        match decoded {
            ServerMessage::SceneSnapshot { snapshot } => {
                assert_eq!(snapshot.scene_id, "town_square_scene");
                assert_eq!(snapshot.width, 10);
                assert_eq!(snapshot.height, 10);
                assert_eq!(snapshot.tiles.len(), 1);
                assert_eq!(snapshot.occupants.len(), 1);
            }
            _ => panic!("expected scene snapshot server message"),
        }
    }

    #[test]
    fn scene_delta_round_trips_via_server_message() {
        let message = ServerMessage::SceneDelta {
            deltas: vec![SceneDelta::ActorMoved {
                actor_id: "Alpha".to_owned(),
                from: Some(ScenePosition { x: 0, y: 0 }),
                to: ScenePosition { x: 0, y: 1 },
            }],
        };

        let encoded = to_json_line(&message).expect("scene delta should serialize");
        let decoded: ServerMessage =
            serde_json::from_str(encoded.trim()).expect("scene delta should deserialize");

        match decoded {
            ServerMessage::SceneDelta { deltas } => {
                assert_eq!(deltas.len(), 1);
                match &deltas[0] {
                    SceneDelta::ActorMoved { actor_id, from, to } => {
                        assert_eq!(actor_id, "Alpha");
                        assert_eq!(from.as_ref().map(|pos| (pos.x, pos.y)), Some((0, 0)));
                        assert_eq!((to.x, to.y), (0, 1));
                    }
                    _ => panic!("expected actor moved delta"),
                }
            }
            _ => panic!("expected scene delta server message"),
        }
    }
}
