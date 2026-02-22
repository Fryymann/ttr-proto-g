#[derive(Default)]
pub struct TurnTracker {
    local_player_name: Option<String>,
    encounter_id: Option<String>,
    initiative_order: Vec<String>,
    round: Option<u32>,
    active_actor: Option<String>,
}

impl TurnTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn remember_local_player_name(&mut self, name: &str) {
        let clean = name.trim();
        if clean.is_empty() {
            return;
        }
        self.local_player_name = Some(clean.to_owned());
    }

    pub fn apply_info(&mut self, text: &str) -> Option<String> {
        if let Some(status) = text
            .strip_prefix("Encounter started: ")
            .and_then(parse_encounter_status_line)
        {
            self.apply_status(status);
            return Some(self.render_tracker_line());
        }

        if let Some(status) = text
            .strip_prefix("Encounter status: ")
            .and_then(parse_encounter_status_line)
        {
            self.apply_status(status);
            return Some(self.render_tracker_line());
        }

        if let Some((active_actor, round)) = parse_active_turn_update(text) {
            self.active_actor = Some(active_actor);
            self.round = Some(round);
            return Some(self.render_tracker_line());
        }

        if parse_encounter_end(text).is_some() {
            self.clear();
            return Some("[Turn] Encounter ended.".to_owned());
        }

        None
    }

    pub fn apply_error(&mut self, text: &str) -> Option<String> {
        let actor = text
            .strip_prefix("It is ")
            .and_then(|rest| rest.strip_suffix("'s turn."))?;

        let clean_actor = actor.trim();
        if clean_actor.is_empty() {
            return None;
        }

        self.active_actor = Some(clean_actor.to_owned());
        if self.encounter_id.is_some() {
            return Some(self.render_tracker_line());
        }

        None
    }

    fn apply_status(&mut self, status: EncounterStatus) {
        self.encounter_id = Some(status.encounter_id);
        self.initiative_order = status.initiative_order;
        self.round = Some(status.round);
        self.active_actor = Some(status.active_actor);
    }

    fn clear(&mut self) {
        self.encounter_id = None;
        self.initiative_order.clear();
        self.round = None;
        self.active_actor = None;
    }

    fn render_tracker_line(&self) -> String {
        let encounter_id = self
            .encounter_id
            .as_deref()
            .unwrap_or("unknown-encounter")
            .to_owned();
        let round = self.round.unwrap_or(0);
        let active_actor = self.active_actor.as_deref().unwrap_or("none");

        let order_text = if self.initiative_order.is_empty() {
            format!("[{}]", self.decorate_actor(active_actor))
        } else {
            self.initiative_order
                .iter()
                .map(|actor| self.decorate_actor(actor))
                .collect::<Vec<_>>()
                .join(" -> ")
        };

        format!(
            "[Turn] {} | round {} | active {} | order {}",
            encounter_id,
            round,
            self.decorate_actor(active_actor),
            order_text
        )
    }

    fn decorate_actor(&self, actor: &str) -> String {
        if self.active_actor.as_deref() == Some(actor) {
            format!("*{}*", self.label_actor(actor))
        } else {
            self.label_actor(actor)
        }
    }

    fn label_actor(&self, actor: &str) -> String {
        if self.local_player_name.as_deref() == Some(actor) {
            format!("{}(you)", actor)
        } else {
            actor.to_owned()
        }
    }
}

struct EncounterStatus {
    encounter_id: String,
    initiative_order: Vec<String>,
    round: u32,
    active_actor: String,
}

fn parse_encounter_status_line(line: &str) -> Option<EncounterStatus> {
    let mut parts = line.split(" | ");
    let encounter_id = parts.next()?.trim();
    if encounter_id.is_empty() {
        return None;
    }

    let participants_part = parts.next()?;
    let initiative_part = parts.next()?;
    let round_part = parts.next()?;
    let active_turn_part = parts.next()?;

    let _ = parse_bracket_list(participants_part, "participants: [")?;
    let initiative_order = parse_bracket_list(initiative_part, "initiative: [")?;
    let round = round_part
        .strip_prefix("round: ")?
        .trim()
        .parse::<u32>()
        .ok()?;
    let active_actor = active_turn_part
        .strip_prefix("active_turn: ")?
        .trim()
        .to_owned();

    if active_actor.is_empty() {
        return None;
    }

    Some(EncounterStatus {
        encounter_id: encounter_id.to_owned(),
        initiative_order,
        round,
        active_actor,
    })
}

fn parse_bracket_list(part: &str, prefix: &str) -> Option<Vec<String>> {
    let inner = part.strip_prefix(prefix)?.strip_suffix(']')?;
    let values = inner
        .split(',')
        .flat_map(|entry| entry.split(" -> "))
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    Some(values)
}

fn parse_active_turn_update(text: &str) -> Option<(String, u32)> {
    let marker = "Active turn: ";
    let start = text.find(marker)?;
    let active_section = &text[(start + marker.len())..];
    let split_at = active_section.find(" (round ")?;
    let actor = active_section[..split_at].trim();
    if actor.is_empty() {
        return None;
    }

    let round_section = &active_section[(split_at + " (round ".len())..];
    let round_str = round_section
        .trim_end_matches('.')
        .strip_suffix(')')
        .unwrap_or(round_section)
        .trim();
    let round = round_str.parse::<u32>().ok()?;
    Some((actor.to_owned(), round))
}

fn parse_encounter_end(text: &str) -> Option<&str> {
    text.strip_prefix("Encounter ")
        .and_then(|rest| rest.split(" ended by ").next())
}

#[cfg(test)]
mod tests {
    use super::TurnTracker;

    #[test]
    fn tracker_renders_from_encounter_start_status() {
        let mut tracker = TurnTracker::new();
        tracker.remember_local_player_name("Alpha");

        let output = tracker
            .apply_info("Encounter started: encounter-1 | participants: [Alpha, Bravo, npc:wolf] | initiative: [Alpha -> Bravo -> npc:wolf] | round: 1 | active_turn: Alpha")
            .expect("start status should render");

        assert_eq!(
            output,
            "[Turn] encounter-1 | round 1 | active *Alpha(you)* | order *Alpha(you)* -> Bravo -> npc:wolf"
        );
    }

    #[test]
    fn tracker_updates_active_actor_on_turn_advance() {
        let mut tracker = TurnTracker::new();
        tracker.remember_local_player_name("Alpha");

        tracker.apply_info("Encounter started: encounter-2 | participants: [Alpha, Bravo] | initiative: [Alpha -> Bravo] | round: 1 | active_turn: Alpha");
        let output = tracker
            .apply_info("Alpha ends turn. Active turn: Bravo (round 1).")
            .expect("turn update should render");

        assert_eq!(
            output,
            "[Turn] encounter-2 | round 1 | active *Bravo* | order Alpha(you) -> *Bravo*"
        );
    }

    #[test]
    fn tracker_updates_round_on_timeout_message() {
        let mut tracker = TurnTracker::new();
        tracker.apply_info("Encounter started: encounter-3 | participants: [Alpha, Bravo] | initiative: [Alpha -> Bravo] | round: 1 | active_turn: Bravo");
        let output = tracker
            .apply_info(
                "Turn timeout: Bravo auto-resolves with dodge. Active turn: Alpha (round 2).",
            )
            .expect("timeout update should render");

        assert_eq!(
            output,
            "[Turn] encounter-3 | round 2 | active *Alpha* | order *Alpha* -> Bravo"
        );
    }

    #[test]
    fn tracker_ignores_unrelated_info_lines() {
        let mut tracker = TurnTracker::new();
        assert!(tracker.apply_info("Welcome to the room.").is_none());
    }

    #[test]
    fn tracker_can_sync_from_out_of_turn_error() {
        let mut tracker = TurnTracker::new();
        tracker.apply_info("Encounter started: encounter-4 | participants: [Alpha, Bravo] | initiative: [Alpha -> Bravo] | round: 1 | active_turn: Alpha");
        let output = tracker
            .apply_error("It is Bravo's turn.")
            .expect("turn error should render");

        assert_eq!(
            output,
            "[Turn] encounter-4 | round 1 | active *Bravo* | order Alpha -> *Bravo*"
        );
    }

    #[test]
    fn tracker_clears_on_encounter_end() {
        let mut tracker = TurnTracker::new();
        tracker.apply_info("Encounter started: encounter-5 | participants: [Alpha, Bravo] | initiative: [Alpha -> Bravo] | round: 1 | active_turn: Alpha");
        let output = tracker
            .apply_info("Encounter encounter-5 ended by Alpha.")
            .expect("encounter end should render");

        assert_eq!(output, "[Turn] Encounter ended.");
    }
}
