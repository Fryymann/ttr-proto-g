use std::env;

use super::state::EncounterPhase;
use super::timer::TurnTimerMarker;
use super::EncounterState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeoutFallbackAction {
    Dodge,
}

impl TimeoutFallbackAction {
    pub fn from_env() -> Self {
        match env::var("TTRPG_ENCOUNTER_TIMEOUT_FALLBACK_ACTION")
            .ok()
            .map(|raw| raw.to_ascii_lowercase())
            .as_deref()
        {
            Some("dodge") | Some("auto_dodge") => Self::Dodge,
            _ => Self::default(),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Dodge => "dodge",
        }
    }
}

impl Default for TimeoutFallbackAction {
    fn default() -> Self {
        Self::Dodge
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeoutFallbackEvent {
    pub encounter_id: String,
    pub actor_id: String,
    pub action: TimeoutFallbackAction,
    pub prior_round: u32,
    pub prior_turn_index: usize,
    pub resulting_round: u32,
    pub next_actor_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeoutFallbackError {
    EncounterNotActive,
    StaleTurnMarker,
    MissingActiveActor,
}

pub fn resolve_timeout_fallback(
    encounter: &mut EncounterState,
    marker: &TurnTimerMarker,
    action: TimeoutFallbackAction,
) -> Result<TimeoutFallbackEvent, TimeoutFallbackError> {
    if encounter.phase != EncounterPhase::Active {
        return Err(TimeoutFallbackError::EncounterNotActive);
    }

    if !marker.is_current_for(encounter) {
        return Err(TimeoutFallbackError::StaleTurnMarker);
    }

    let actor_id = encounter
        .active_actor()
        .map(str::to_owned)
        .ok_or(TimeoutFallbackError::MissingActiveActor)?;
    let prior_round = encounter.round;
    let prior_turn_index = encounter.active_turn_index;
    let next_actor_id = encounter.advance_turn().map(str::to_owned);

    Ok(TimeoutFallbackEvent {
        encounter_id: encounter.encounter_id.clone(),
        actor_id,
        action,
        prior_round,
        prior_turn_index,
        resulting_round: encounter.round,
        next_actor_id,
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::encounter::timer::TurnTimerMarker;
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

        EncounterState::start(
            "encounter:1",
            "scene:test",
            "Alpha",
            &party,
            Vec::new(),
            initiative_scores,
        )
        .expect("encounter should start")
    }

    #[test]
    fn timeout_fallback_advances_turn_deterministically() {
        let mut encounter = test_encounter();
        let marker =
            TurnTimerMarker::capture("room:test", &encounter).expect("active turn should exist");

        let event = resolve_timeout_fallback(&mut encounter, &marker, TimeoutFallbackAction::Dodge)
            .expect("fallback should advance");

        assert_eq!(event.actor_id, "Alpha");
        assert_eq!(event.action, TimeoutFallbackAction::Dodge);
        assert_eq!(event.prior_round, 1);
        assert_eq!(event.prior_turn_index, 0);
        assert_eq!(event.resulting_round, 1);
        assert_eq!(event.next_actor_id.as_deref(), Some("Bravo"));
        assert_eq!(encounter.active_actor(), Some("Bravo"));
    }

    #[test]
    fn stale_marker_is_rejected_without_advancing_turn() {
        let mut encounter = test_encounter();
        let stale_marker =
            TurnTimerMarker::capture("room:test", &encounter).expect("active turn should exist");
        encounter.advance_turn();
        let actor_after_manual_advance = encounter
            .active_actor()
            .map(str::to_owned)
            .expect("active actor should exist");

        let error =
            resolve_timeout_fallback(&mut encounter, &stale_marker, TimeoutFallbackAction::Dodge)
                .expect_err("stale marker should fail");

        assert_eq!(error, TimeoutFallbackError::StaleTurnMarker);
        assert_eq!(
            encounter.active_actor(),
            Some(actor_after_manual_advance.as_str())
        );
    }
}
