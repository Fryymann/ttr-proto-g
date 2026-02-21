use crate::scene::{Position, Scene, TileType};
use crate::scene::queue::{SceneCommandQueue};
use ttrpg_protocol::{SceneCommand, SceneDelta, ScenePosition, SceneSnapshot, SceneTileType};

#[derive(Clone)]
pub struct SceneRuntime {
    pub scene: Scene,
    pub queue: SceneCommandQueue,
}

impl SceneRuntime {
    pub fn new(scene: Scene) -> Self {
        Self {
            scene,
            queue: SceneCommandQueue::new(),
        }
    }

    /// Processes all queued commands deterministically and returns the resulting deltas.
    pub fn tick(&mut self) -> Vec<SceneDelta> {
        let envelopes = self.queue.drain_deterministic();
        let mut deltas = Vec::new();

        for env in envelopes {
            match env.command {
                SceneCommand::Move { target_pos } => {
                    let actor_id = env.actor_id;
                    let new_pos = Position {
                        x: target_pos.x,
                        y: target_pos.y,
                    };

                    // Find current position before moving
                    let old_pos = self
                        .scene
                        .occupants
                        .iter()
                        .find(|(_, id)| **id == actor_id)
                        .map(|(pos, _)| *pos);

                    if self.scene.move_actor(&actor_id, new_pos).is_ok() {
                        deltas.push(SceneDelta::ActorMoved {
                            actor_id,
                            from: old_pos.map(|p| ScenePosition { x: p.x, y: p.y }),
                            to: target_pos,
                        });
                    }
                }
            }
        }

        deltas
    }

    /// Generates a full snapshot of the current scene state.
    pub fn generate_snapshot(&self) -> SceneSnapshot {
        let mut tiles: Vec<(ScenePosition, SceneTileType)> = self
            .scene
            .grid
            .tiles
            .iter()
            .map(|(pos, tile)| {
                (
                    ScenePosition { x: pos.x, y: pos.y },
                    match tile.tile_type {
                        TileType::Floor => SceneTileType::Floor,
                        TileType::Wall => SceneTileType::Wall,
                        TileType::DifficultTerrain => SceneTileType::DifficultTerrain,
                    },
                )
            })
            .collect();

        // Sort tiles by position for stable serialization
        tiles.sort_by_key(|(pos, _)| (pos.x, pos.y));

        let mut occupants: Vec<(ScenePosition, String)> = self
            .scene
            .occupants
            .iter()
            .map(|(pos, id)| (ScenePosition { x: pos.x, y: pos.y }, id.clone()))
            .collect();

        // Sort occupants by position for stable serialization
        occupants.sort_by_key(|(pos, _)| (pos.x, pos.y));

        SceneSnapshot {
            scene_id: self.scene.id.clone(),
            width: self.scene.grid.width,
            height: self.scene.grid.height,
            tiles,
            occupants,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::{Grid, Tile};
    use crate::scene::queue::CommandEnvelope;
    use std::collections::HashMap;

    fn setup_test_runtime() -> SceneRuntime {
        let mut tiles = HashMap::new();
        for x in 0..5 {
            for y in 0..5 {
                tiles.insert(
                    Position { x, y },
                    Tile {
                        tile_type: TileType::Floor,
                    },
                );
            }
        }
        let scene = Scene {
            id: "test-runtime".to_owned(),
            grid: Grid {
                width: 5,
                height: 5,
                tiles,
            },
            occupants: HashMap::new(),
        };
        SceneRuntime::new(scene)
    }

    #[test]
    fn test_tick_processing() {
        let mut runtime = setup_test_runtime();
        let actor = "hero";
        
        runtime.queue.push(CommandEnvelope {
            actor_id: actor.to_owned(),
            command: SceneCommand::Move { target_pos: ScenePosition { x: 1, y: 1 } },
            timestamp_ms: 100,
            sequence_id: 0,
        });

        let deltas = runtime.tick();
        assert_eq!(deltas.len(), 1);
        
        if let Some(SceneDelta::ActorMoved { actor_id, from, to }) = deltas.get(0) {
            assert_eq!(actor_id, actor);
            assert_eq!(from, &None);
            assert_eq!(to.x, 1);
            assert_eq!(to.y, 1);
        } else {
            panic!("Expected ActorMoved delta");
        }

        assert_eq!(runtime.scene.occupants.get(&Position { x: 1, y: 1 }), Some(&actor.to_owned()));
    }

    #[test]
    fn test_multi_client_consistency() {
        // This test addresses finding #5: ensuring two clients reach identical states
        // regardless of the order they receive/queue commands before a tick.
        let mut client1 = setup_test_runtime();
        let mut client2 = setup_test_runtime();

        let actor_a = "ActorA";
        let actor_b = "ActorB";

        // Both actors want to move to the same spot (2,2)
        let cmd_a = CommandEnvelope {
            actor_id: actor_a.to_owned(),
            command: SceneCommand::Move { target_pos: ScenePosition { x: 2, y: 2 } },
            timestamp_ms: 500,
            sequence_id: 0,
        };
        let cmd_b = CommandEnvelope {
            actor_id: actor_b.to_owned(),
            command: SceneCommand::Move { target_pos: ScenePosition { x: 2, y: 2 } },
            timestamp_ms: 500,
            sequence_id: 0,
        };

        // Client 1 receives A then B
        client1.queue.push(cmd_a.clone());
        client1.queue.push(cmd_b.clone());

        // Client 2 receives B then A
        client2.queue.push(cmd_b.clone());
        client2.queue.push(cmd_a.clone());

        let deltas1 = client1.tick();
        let deltas2 = client2.tick();

        // Determinism: ActorA (Tie-breaker) should win in both
        assert_eq!(deltas1.len(), 1);
        assert_eq!(deltas2.len(), 1);

        let snap1 = client1.generate_snapshot();
        let snap2 = client2.generate_snapshot();

        // Snapshots must be bit-identical (stable serialization)
        assert_eq!(serde_json::to_string(&snap1).unwrap(), serde_json::to_string(&snap2).unwrap());
        
        // Final state: ActorA is at (2,2)
        assert_eq!(client1.scene.occupants.get(&Position { x: 2, y: 2 }), Some(&actor_a.to_owned()));
        assert_eq!(client2.scene.occupants.get(&Position { x: 2, y: 2 }), Some(&actor_a.to_owned()));
    }
}
