use std::collections::{HashMap, HashSet};

use ttrpg_protocol::{SceneDelta, ScenePosition, SceneSnapshot, SceneTileType};

#[derive(Default)]
pub struct SceneRenderer {
    local_player_name: Option<String>,
    known_player_names: HashSet<String>,
    state: Option<SceneState>,
}

#[derive(Clone)]
struct SceneState {
    scene_id: String,
    width: u32,
    height: u32,
    tiles: HashMap<PositionKey, SceneTileType>,
    occupants: HashMap<PositionKey, String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct PositionKey {
    x: i32,
    y: i32,
}

impl PositionKey {
    fn from_scene_pos(pos: &ScenePosition) -> Self {
        Self { x: pos.x, y: pos.y }
    }

    fn to_scene_pos(self) -> ScenePosition {
        ScenePosition {
            x: self.x,
            y: self.y,
        }
    }
}

impl SceneRenderer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn remember_local_player_name(&mut self, name: &str) {
        let clean = name.trim();
        if clean.is_empty() {
            return;
        }
        self.local_player_name = Some(clean.to_owned());
        self.known_player_names.insert(clean.to_owned());
    }

    pub fn update_room_players(&mut self, players: &[String]) {
        self.known_player_names = players
            .iter()
            .map(|name| name.trim())
            .filter(|name| !name.is_empty())
            .map(str::to_owned)
            .collect::<HashSet<_>>();

        if let Some(local_name) = &self.local_player_name {
            self.known_player_names.insert(local_name.clone());
        }
    }

    pub fn apply_snapshot(&mut self, snapshot: SceneSnapshot) -> String {
        let mut tiles = HashMap::new();
        for (pos, tile_type) in snapshot.tiles {
            tiles.insert(PositionKey::from_scene_pos(&pos), tile_type);
        }

        let mut occupants = HashMap::new();
        for (pos, actor_id) in snapshot.occupants {
            occupants.insert(PositionKey::from_scene_pos(&pos), actor_id);
        }

        let scene_id = snapshot.scene_id;
        self.state = Some(SceneState {
            scene_id: scene_id.clone(),
            width: snapshot.width,
            height: snapshot.height,
            tiles,
            occupants,
        });

        self.render_with_header(format!(
            "Scene snapshot loaded: {} ({}x{})",
            scene_id, snapshot.width, snapshot.height
        ))
    }

    pub fn apply_deltas(&mut self, deltas: Vec<SceneDelta>) -> String {
        let Some(state) = self.state.as_mut() else {
            return format!(
                "\n[Scene] Received {} update(s) before initial snapshot; map not rendered yet.",
                deltas.len()
            );
        };

        if deltas.is_empty() {
            return self.render_with_header("Scene update: no changes".to_owned());
        }

        let mut delta_lines = Vec::new();
        for delta in deltas {
            match delta {
                SceneDelta::ActorMoved { actor_id, from, to } => {
                    state.remove_actor(&actor_id);

                    if let Some(ref from_pos) = from {
                        state
                            .occupants
                            .remove(&PositionKey::from_scene_pos(from_pos));
                    }

                    state
                        .occupants
                        .insert(PositionKey::from_scene_pos(&to), actor_id.clone());

                    let from_text = from
                        .as_ref()
                        .map(position_text)
                        .unwrap_or_else(|| "spawn".to_owned());
                    delta_lines.push(format!(
                        "- {} moved {} -> {}",
                        actor_id,
                        from_text,
                        position_text(&to)
                    ));
                }
                SceneDelta::TileChanged { pos, new_type } => {
                    state
                        .tiles
                        .insert(PositionKey::from_scene_pos(&pos), new_type.clone());
                    delta_lines.push(format!(
                        "- tile {} changed to {}",
                        position_text(&pos),
                        tile_name(&new_type)
                    ));
                }
            }
        }

        self.render_with_header(format!("Scene update:\n{}", delta_lines.join("\n")))
    }

    fn render_with_header(&self, header: String) -> String {
        let Some(state) = &self.state else {
            return format!("\n[Scene] {}", header);
        };

        let mut lines = vec![format!("\n[Scene] {}", header)];
        lines.push(format!(
            "Legend: @=you P=player N=npc .=floor #=wall ~=difficult ?=unknown"
        ));
        lines.push(format!("Map: {}", state.scene_id));
        lines.extend(self.render_grid_lines(state));
        lines.push(self.render_actor_roster(state));
        lines.join("\n")
    }

    fn render_grid_lines(&self, state: &SceneState) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!("    {}", render_x_axis(state.width)));

        for y in 0..state.height {
            let mut row = String::new();
            for x in 0..state.width {
                let pos = PositionKey {
                    x: x as i32,
                    y: y as i32,
                };

                if let Some(actor_id) = state.occupants.get(&pos) {
                    row.push(self.actor_symbol(actor_id));
                } else {
                    row.push(tile_symbol(state.tiles.get(&pos)));
                }
            }

            lines.push(format!("{:>3} {}", y, row));
        }

        lines
    }

    fn render_actor_roster(&self, state: &SceneState) -> String {
        let mut actors = state
            .occupants
            .iter()
            .map(|(pos, actor_id)| {
                (
                    actor_id.clone(),
                    pos.to_scene_pos(),
                    self.actor_symbol(actor_id),
                    self.actor_kind_name(actor_id),
                )
            })
            .collect::<Vec<_>>();

        actors.sort_by_key(|(actor_id, pos, _, _)| (actor_id.clone(), pos.y, pos.x));

        if actors.is_empty() {
            return "Actors: none".to_owned();
        }

        let mut entries = Vec::new();
        for (actor_id, pos, symbol, kind) in actors {
            entries.push(format!(
                "{}:{} ({}) {}",
                symbol,
                actor_id,
                kind,
                position_text(&pos)
            ));
        }

        format!("Actors: {}", entries.join(", "))
    }

    fn actor_symbol(&self, actor_id: &str) -> char {
        if self
            .local_player_name
            .as_ref()
            .is_some_and(|name| name == actor_id)
        {
            '@'
        } else if self.known_player_names.contains(actor_id) {
            'P'
        } else {
            'N'
        }
    }

    fn actor_kind_name(&self, actor_id: &str) -> &'static str {
        if self
            .local_player_name
            .as_ref()
            .is_some_and(|name| name == actor_id)
        {
            "you"
        } else if self.known_player_names.contains(actor_id) {
            "player"
        } else {
            "npc"
        }
    }
}

impl SceneState {
    fn remove_actor(&mut self, actor_id: &str) {
        let actor_pos =
            self.occupants
                .iter()
                .find_map(|(pos, id)| if id == actor_id { Some(*pos) } else { None });

        if let Some(pos) = actor_pos {
            self.occupants.remove(&pos);
        }
    }
}

fn render_x_axis(width: u32) -> String {
    let mut labels = String::new();
    for x in 0..width {
        let x = x as usize;
        labels.push_str(&(x % 10).to_string());
    }
    labels
}

fn tile_symbol(tile: Option<&SceneTileType>) -> char {
    match tile {
        Some(SceneTileType::Floor) => '.',
        Some(SceneTileType::Wall) => '#',
        Some(SceneTileType::DifficultTerrain) => '~',
        None => '?',
    }
}

fn tile_name(tile: &SceneTileType) -> &'static str {
    match tile {
        SceneTileType::Floor => "floor",
        SceneTileType::Wall => "wall",
        SceneTileType::DifficultTerrain => "difficult_terrain",
    }
}

fn position_text(pos: &ScenePosition) -> String {
    format!("({}, {})", pos.x, pos.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_snapshot() -> SceneSnapshot {
        SceneSnapshot {
            scene_id: "test_scene".to_owned(),
            width: 3,
            height: 2,
            tiles: vec![
                (ScenePosition { x: 0, y: 0 }, SceneTileType::Floor),
                (ScenePosition { x: 1, y: 0 }, SceneTileType::Wall),
                (ScenePosition { x: 2, y: 0 }, SceneTileType::Floor),
                (ScenePosition { x: 0, y: 1 }, SceneTileType::Floor),
                (
                    ScenePosition { x: 1, y: 1 },
                    SceneTileType::DifficultTerrain,
                ),
                (ScenePosition { x: 2, y: 1 }, SceneTileType::Floor),
            ],
            occupants: vec![
                (ScenePosition { x: 0, y: 0 }, "Hero".to_owned()),
                (ScenePosition { x: 2, y: 1 }, "Goblin".to_owned()),
            ],
        }
    }

    #[test]
    fn snapshot_render_includes_legend_and_symbols() {
        let mut renderer = SceneRenderer::new();
        renderer.remember_local_player_name("Hero");
        renderer.update_room_players(&["Hero".to_owned(), "Ally".to_owned()]);

        let output = renderer.apply_snapshot(sample_snapshot());

        assert!(output.contains("Legend: @=you P=player N=npc"));
        assert!(output.contains("Map: test_scene"));
        assert!(output.contains("0 @#."));
        assert!(output.contains("1 .~N"));
        assert!(output.contains("@:Hero (you) (0, 0)"));
        assert!(output.contains("N:Goblin (npc) (2, 1)"));
    }

    #[test]
    fn actor_move_delta_updates_rendered_position() {
        let mut renderer = SceneRenderer::new();
        renderer.remember_local_player_name("Hero");
        renderer.update_room_players(&["Hero".to_owned(), "Ally".to_owned()]);
        renderer.apply_snapshot(sample_snapshot());

        let output = renderer.apply_deltas(vec![SceneDelta::ActorMoved {
            actor_id: "Hero".to_owned(),
            from: Some(ScenePosition { x: 0, y: 0 }),
            to: ScenePosition { x: 2, y: 0 },
        }]);

        assert!(output.contains("Hero moved (0, 0) -> (2, 0)"));
        assert!(output.contains("0 .#@"));
        assert!(output.contains("@:Hero (you) (2, 0)"));
    }

    #[test]
    fn known_players_render_as_player_symbol() {
        let mut renderer = SceneRenderer::new();
        renderer.remember_local_player_name("Hero");
        renderer.update_room_players(&["Hero".to_owned(), "Ally".to_owned()]);
        renderer.apply_snapshot(sample_snapshot());

        let output = renderer.apply_deltas(vec![SceneDelta::ActorMoved {
            actor_id: "Ally".to_owned(),
            from: None,
            to: ScenePosition { x: 0, y: 1 },
        }]);

        assert!(output.contains("Ally moved spawn -> (0, 1)"));
        assert!(output.contains("1 P~N"));
        assert!(output.contains("P:Ally (player) (0, 1)"));
    }
}
