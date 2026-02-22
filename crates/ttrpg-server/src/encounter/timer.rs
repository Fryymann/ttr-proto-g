use std::env;
use std::time::Duration;

use super::EncounterState;

const DEFAULT_TURN_TIMEOUT_MS: u64 = 30_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TurnTimerConfig {
    timeout: Duration,
}

impl TurnTimerConfig {
    pub fn from_millis(timeout_ms: u64) -> Self {
        Self {
            timeout: Duration::from_millis(timeout_ms.max(1)),
        }
    }

    pub fn from_env() -> Self {
        if let Ok(raw_ms) = env::var("TTRPG_ENCOUNTER_TURN_TIMEOUT_MS") {
            if let Ok(ms) = raw_ms.parse::<u64>() {
                return Self::from_millis(ms);
            }
        }

        if let Ok(raw_secs) = env::var("TTRPG_ENCOUNTER_TURN_TIMEOUT_SECS") {
            if let Ok(secs) = raw_secs.parse::<u64>() {
                return Self::from_millis(secs.saturating_mul(1_000));
            }
        }

        Self::default()
    }

    pub fn timeout(&self) -> Duration {
        self.timeout
    }
}

impl Default for TurnTimerConfig {
    fn default() -> Self {
        Self::from_millis(DEFAULT_TURN_TIMEOUT_MS)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TurnTimerMarker {
    pub room_id: String,
    pub encounter_id: String,
    pub actor_id: String,
    pub round: u32,
    pub turn_index: usize,
}

impl TurnTimerMarker {
    pub fn capture(room_id: impl Into<String>, encounter: &EncounterState) -> Option<Self> {
        let actor_id = encounter.active_actor()?.to_owned();
        Some(Self {
            room_id: room_id.into(),
            encounter_id: encounter.encounter_id.clone(),
            actor_id,
            round: encounter.round,
            turn_index: encounter.active_turn_index,
        })
    }

    pub fn is_current_for(&self, encounter: &EncounterState) -> bool {
        encounter.encounter_id == self.encounter_id
            && encounter.round == self.round
            && encounter.active_turn_index == self.turn_index
            && encounter.active_actor() == Some(self.actor_id.as_str())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::encounter::InitiativeScore;
    use crate::party::Party;

    fn test_encounter() -> EncounterState {
        let party = Party::new(
            "party:alpha",
            "campaign:test",
            "Alpha",
            vec!["Alpha".to_owned(), "Bravo".to_owned()],
        )
        .expect("party should be valid");
        let mut initiative_scores = HashMap::new();
        initiative_scores.insert(
            "Alpha".to_owned(),
            InitiativeScore {
                roll_total: 16,
                dex_mod: 2,
            },
        );
        initiative_scores.insert(
            "Bravo".to_owned(),
            InitiativeScore {
                roll_total: 10,
                dex_mod: 1,
            },
        );
        initiative_scores.insert(
            "npc:wolf".to_owned(),
            InitiativeScore {
                roll_total: 8,
                dex_mod: 0,
            },
        );

        EncounterState::start(
            "encounter:1",
            "scene:test",
            "Alpha",
            &party,
            vec!["npc:wolf".to_owned()],
            initiative_scores,
        )
        .expect("encounter should start")
    }

    #[test]
    fn marker_matches_when_turn_is_unchanged() {
        let encounter = test_encounter();
        let marker =
            TurnTimerMarker::capture("room:test", &encounter).expect("active turn should exist");
        assert!(marker.is_current_for(&encounter));
    }

    #[test]
    fn marker_turn_identity_becomes_stale_after_advance() {
        let mut encounter = test_encounter();
        let marker =
            TurnTimerMarker::capture("room:test", &encounter).expect("active turn should exist");

        encounter.advance_turn();

        assert!(!marker.is_current_for(&encounter));
    }
}
