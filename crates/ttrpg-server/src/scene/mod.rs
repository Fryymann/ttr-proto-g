use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TileType {
    Floor,
    Wall,
    DifficultTerrain,
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub tile_type: TileType,
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub tiles: HashMap<Position, Tile>,
}

impl Grid {
    pub fn is_within_bounds(&self, pos: Position) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32
    }

    pub fn get_tile(&self, pos: Position) -> Option<&Tile> {
        self.tiles.get(&pos)
    }
}

#[derive(Debug, Clone)]
pub struct Scene {
    pub id: String,
    pub grid: Grid,
    pub occupants: HashMap<Position, String>, // Position -> ActorID (Character/NPC ID)
}

impl Scene {
    /// Checks if an actor can occupy a specific tile.
    /// V1 Rules:
    /// - Must be within bounds.
    /// - Tile must be passable (not a Wall).
    /// - Tile must not be occupied by another actor.
    pub fn can_occupy(&self, pos: Position, _actor_id: &str) -> bool {
        if !self.grid.is_within_bounds(pos) {
            return false;
        }

        if let Some(tile) = self.grid.get_tile(pos) {
            if tile.tile_type == TileType::Wall {
                return false;
            }
        } else {
            // If no tile defined, assume it's out of bounds or empty space
            // For V1, we might treat undefined tiles as impassable or default floor.
            // Let's assume defined tiles are the only playable space.
            return false;
        }

        if self.occupants.contains_key(&pos) {
            // For V1, no two actors can occupy the same square.
            return false;
        }

        true
    }

    /// Moves an actor to a new position if valid.
    pub fn move_actor(&mut self, actor_id: &str, new_pos: Position) -> Result<(), String> {
        if !self.can_occupy(new_pos, actor_id) {
            return Err("Cannot occupy target position".to_owned());
        }

        // Find current position of actor
        let old_pos = self
            .occupants
            .iter()
            .find(|(_, id)| *id == actor_id)
            .map(|(pos, _)| *pos);

        if let Some(old) = old_pos {
            self.occupants.remove(&old);
        }

        self.occupants.insert(new_pos, actor_id.to_owned());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_basic_scene() -> Scene {
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
        // Add a wall
        tiles.insert(
            Position { x: 2, y: 2 },
            Tile {
                tile_type: TileType::Wall,
            },
        );

        Scene {
            id: "test-scene".to_owned(),
            grid: Grid {
                width: 5,
                height: 5,
                tiles,
            },
            occupants: HashMap::new(),
        }
    }

    #[test]
    fn test_within_bounds() {
        let scene = setup_basic_scene();
        assert!(scene.grid.is_within_bounds(Position { x: 0, y: 0 }));
        assert!(scene.grid.is_within_bounds(Position { x: 4, y: 4 }));
        assert!(!scene.grid.is_within_bounds(Position { x: 5, y: 0 }));
        assert!(!scene.grid.is_within_bounds(Position { x: 0, y: -1 }));
    }

    #[test]
    fn test_occupancy_rules() {
        let mut scene = setup_basic_scene();
        let actor1 = "actor1";
        let actor2 = "actor2";

        // Space is free
        assert!(scene.can_occupy(Position { x: 1, y: 1 }, actor1));

        // Wall is impassable
        assert!(!scene.can_occupy(Position { x: 2, y: 2 }, actor1));

        // Occupy space
        scene.move_actor(actor1, Position { x: 1, y: 1 }).unwrap();

        // Cannot occupy same space
        assert!(!scene.can_occupy(Position { x: 1, y: 1 }, actor2));

        // Can occupy adjacent free space
        assert!(scene.can_occupy(Position { x: 1, y: 2 }, actor2));
    }

    #[test]
    fn test_movement() {
        let mut scene = setup_basic_scene();
        let actor = "hero";
        let pos1 = Position { x: 0, y: 0 };
        let pos2 = Position { x: 0, y: 1 };

        scene.move_actor(actor, pos1).unwrap();
        assert_eq!(scene.occupants.get(&pos1), Some(&actor.to_owned()));

        scene.move_actor(actor, pos2).unwrap();
        assert_eq!(scene.occupants.get(&pos2), Some(&actor.to_owned()));
        assert!(scene.occupants.get(&pos1).is_none());
    }
}
