use std::collections::{BTreeSet, HashMap};

use crate::party::Party;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncounterPhase {
    Active,
    Resolved,
    Ended,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncounterParticipantKind {
    PartyMember,
    Npc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncounterParticipant {
    pub actor_id: String,
    pub kind: EncounterParticipantKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitiativeScore {
    pub roll_total: i32,
    pub dex_mod: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncounterStartError {
    TriggeringActorNotInParty,
    EmptyParticipants,
    MissingInitiativeScore { actor_id: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncounterState {
    pub encounter_id: String,
    pub scene_id: String,
    pub party_id: String,
    pub phase: EncounterPhase,
    pub participants: Vec<EncounterParticipant>,
    pub initiative_order: Vec<String>,
    pub active_turn_index: usize,
    pub round: u32,
}

impl EncounterState {
    pub fn start(
        encounter_id: impl Into<String>,
        scene_id: impl Into<String>,
        triggering_actor_id: &str,
        party: &Party,
        relevant_npc_ids: impl IntoIterator<Item = String>,
        initiative_scores: HashMap<String, InitiativeScore>,
    ) -> Result<Self, EncounterStartError> {
        if !party.includes_member(triggering_actor_id) {
            return Err(EncounterStartError::TriggeringActorNotInParty);
        }

        let participants = capture_participants(party, relevant_npc_ids);
        if participants.is_empty() {
            return Err(EncounterStartError::EmptyParticipants);
        }

        let mut sortable = Vec::with_capacity(participants.len());
        for participant in &participants {
            let score = initiative_scores
                .get(&participant.actor_id)
                .ok_or_else(|| EncounterStartError::MissingInitiativeScore {
                    actor_id: participant.actor_id.clone(),
                })?;
            sortable.push((participant.actor_id.clone(), *score));
        }

        let initiative_order = sort_initiative_desc(sortable);

        Ok(Self {
            encounter_id: encounter_id.into(),
            scene_id: scene_id.into(),
            party_id: party.party_id.clone(),
            phase: EncounterPhase::Active,
            participants,
            initiative_order,
            active_turn_index: 0,
            round: 1,
        })
    }

    pub fn active_actor(&self) -> Option<&str> {
        if self.phase != EncounterPhase::Active {
            return None;
        }

        self.initiative_order
            .get(self.active_turn_index)
            .map(String::as_str)
    }

    pub fn is_actor_turn(&self, actor_id: &str) -> bool {
        matches!(self.active_actor(), Some(active) if active == actor_id)
    }

    pub fn is_participant(&self, actor_id: &str) -> bool {
        self.participants.iter().any(|p| p.actor_id == actor_id)
    }

    pub fn advance_turn(&mut self) -> Option<&str> {
        if self.phase != EncounterPhase::Active || self.initiative_order.is_empty() {
            return None;
        }

        self.active_turn_index = (self.active_turn_index + 1) % self.initiative_order.len();
        if self.active_turn_index == 0 {
            self.round += 1;
        }

        self.active_actor()
    }

    pub fn resolve(&mut self) {
        self.phase = EncounterPhase::Resolved;
    }

    pub fn end(&mut self) {
        self.phase = EncounterPhase::Ended;
    }
}

pub fn capture_participants(
    party: &Party,
    relevant_npc_ids: impl IntoIterator<Item = String>,
) -> Vec<EncounterParticipant> {
    let mut participants = party
        .members()
        .iter()
        .cloned()
        .map(|actor_id| EncounterParticipant {
            actor_id,
            kind: EncounterParticipantKind::PartyMember,
        })
        .collect::<Vec<_>>();

    let party_members = party.members().iter().collect::<BTreeSet<_>>();
    let npc_ids = relevant_npc_ids
        .into_iter()
        .filter(|actor_id| !party_members.contains(actor_id))
        .collect::<BTreeSet<_>>();

    participants.extend(npc_ids.into_iter().map(|actor_id| EncounterParticipant {
        actor_id,
        kind: EncounterParticipantKind::Npc,
    }));
    participants
}

pub fn sort_initiative_desc(entries: Vec<(String, InitiativeScore)>) -> Vec<String> {
    let mut sorted = entries;
    sorted.sort_by(|(left_actor, left_score), (right_actor, right_score)| {
        right_score
            .roll_total
            .cmp(&left_score.roll_total)
            .then_with(|| right_score.dex_mod.cmp(&left_score.dex_mod))
            .then_with(|| left_actor.cmp(right_actor))
    });

    sorted
        .into_iter()
        .map(|(actor_id, _)| actor_id)
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::party::Party;

    fn test_party() -> Party {
        Party::new(
            "party:alpha",
            "greenhollow",
            "Alpha",
            vec!["Bravo".to_owned(), "Alpha".to_owned()],
        )
        .expect("party should be valid")
    }

    #[test]
    fn participant_capture_is_party_scoped_plus_relevant_npcs() {
        let party = test_party();
        let participants = capture_participants(
            &party,
            vec![
                "npc:wolf".to_owned(),
                "npc:wolf".to_owned(),
                "npc:bandit".to_owned(),
            ],
        );

        assert_eq!(participants.len(), 4);
        assert_eq!(participants[0].actor_id, "Alpha");
        assert_eq!(participants[1].actor_id, "Bravo");
        assert_eq!(participants[2].actor_id, "npc:bandit");
        assert_eq!(participants[3].actor_id, "npc:wolf");
    }

    #[test]
    fn initiative_tie_breakers_are_deterministic() {
        let ordered = sort_initiative_desc(vec![
            (
                "bravo".to_owned(),
                InitiativeScore {
                    roll_total: 14,
                    dex_mod: 2,
                },
            ),
            (
                "alpha".to_owned(),
                InitiativeScore {
                    roll_total: 14,
                    dex_mod: 2,
                },
            ),
            (
                "charlie".to_owned(),
                InitiativeScore {
                    roll_total: 14,
                    dex_mod: 3,
                },
            ),
        ]);

        assert_eq!(
            ordered,
            vec!["charlie".to_owned(), "alpha".to_owned(), "bravo".to_owned()]
        );
    }

    #[test]
    fn turn_advance_wraps_and_increments_round() {
        let party = test_party();
        let mut initiative_scores = HashMap::new();
        initiative_scores.insert(
            "Alpha".to_owned(),
            InitiativeScore {
                roll_total: 15,
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

        let mut encounter = EncounterState::start(
            "encounter-1",
            "town_square_scene",
            "Alpha",
            &party,
            vec!["npc:wolf".to_owned()],
            initiative_scores,
        )
        .expect("encounter start should succeed");

        assert_eq!(encounter.round, 1);
        assert_eq!(encounter.active_actor(), Some("Alpha"));

        encounter.advance_turn();
        assert_eq!(encounter.active_actor(), Some("Bravo"));
        assert_eq!(encounter.round, 1);

        encounter.advance_turn();
        assert_eq!(encounter.active_actor(), Some("npc:wolf"));
        assert_eq!(encounter.round, 1);

        encounter.advance_turn();
        assert_eq!(encounter.active_actor(), Some("Alpha"));
        assert_eq!(encounter.round, 2);
    }

    #[test]
    fn encounter_can_resolve_and_end() {
        let party = test_party();
        let mut initiative_scores = HashMap::new();
        initiative_scores.insert(
            "Alpha".to_owned(),
            InitiativeScore {
                roll_total: 15,
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

        let mut encounter = EncounterState::start(
            "encounter-2",
            "town_square_scene",
            "Alpha",
            &party,
            Vec::new(),
            initiative_scores,
        )
        .expect("encounter start should succeed");

        encounter.resolve();
        assert_eq!(encounter.phase, EncounterPhase::Resolved);
        assert_eq!(encounter.active_actor(), None);

        encounter.end();
        assert_eq!(encounter.phase, EncounterPhase::Ended);
    }
}
