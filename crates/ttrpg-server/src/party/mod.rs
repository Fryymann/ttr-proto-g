use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Party {
    pub party_id: String,
    pub campaign_id: String,
    pub leader_actor_id: String,
    member_actor_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartyError {
    EmptyMembers,
    LeaderNotMember,
}

impl Party {
    pub fn new(
        party_id: impl Into<String>,
        campaign_id: impl Into<String>,
        leader_actor_id: impl Into<String>,
        members: impl IntoIterator<Item = String>,
    ) -> Result<Self, PartyError> {
        let party_id = party_id.into();
        let campaign_id = campaign_id.into();
        let leader_actor_id = leader_actor_id.into();

        let ordered_members = members.into_iter().collect::<BTreeSet<_>>();
        if ordered_members.is_empty() {
            return Err(PartyError::EmptyMembers);
        }
        if !ordered_members.contains(&leader_actor_id) {
            return Err(PartyError::LeaderNotMember);
        }

        Ok(Self {
            party_id,
            campaign_id,
            leader_actor_id,
            member_actor_ids: ordered_members.into_iter().collect(),
        })
    }

    pub fn members(&self) -> &[String] {
        &self.member_actor_ids
    }

    pub fn includes_member(&self, actor_id: &str) -> bool {
        self.member_actor_ids
            .binary_search_by(|member| member.as_str().cmp(actor_id))
            .is_ok()
    }
}

#[derive(Debug, Clone, Default)]
pub struct PartyRegistry {
    parties: HashMap<String, Party>,
    actor_to_party: HashMap<String, String>,
}

impl PartyRegistry {
    fn detach_actor_from_party(&mut self, party_id: &str, actor_id: &str) {
        let mut should_remove_party = false;

        if let Some(previous_party) = self.parties.get_mut(party_id) {
            previous_party.member_actor_ids.retain(|member| member != actor_id);

            if previous_party.member_actor_ids.is_empty() {
                should_remove_party = true;
            } else if !previous_party
                .member_actor_ids
                .contains(&previous_party.leader_actor_id)
            {
                previous_party.leader_actor_id = previous_party.member_actor_ids[0].clone();
            }
        }

        if should_remove_party {
            self.parties.remove(party_id);
        }
    }

    pub fn upsert_party(&mut self, party: Party) {
        let party_id = party.party_id.clone();
        let incoming_members = party.members().to_vec();

        if let Some(existing) = self.parties.get(&party_id) {
            for actor_id in existing.members() {
                self.actor_to_party.remove(actor_id);
            }
        }

        for actor_id in &incoming_members {
            if let Some(previous_party_id) = self.actor_to_party.get(actor_id).cloned() {
                if previous_party_id != party_id {
                    self.detach_actor_from_party(&previous_party_id, actor_id);
                }
            }
        }

        for actor_id in &incoming_members {
            self.actor_to_party
                .insert(actor_id.clone(), party_id.clone());
        }

        self.parties.insert(party_id, party);
    }

    pub fn party_for_actor(&self, actor_id: &str) -> Option<&Party> {
        let party_id = self.actor_to_party.get(actor_id)?;
        self.parties.get(party_id)
    }

    pub fn ensure_solo_party_for_actor(&mut self, actor_id: &str, campaign_id: &str) -> String {
        if let Some(existing) = self.actor_to_party.get(actor_id) {
            return existing.clone();
        }

        let party_id = format!(
            "solo:{}:{}",
            campaign_id.to_lowercase(),
            actor_id.to_lowercase()
        );
        let party = Party::new(
            party_id.clone(),
            campaign_id.to_owned(),
            actor_id.to_owned(),
            vec![actor_id.to_owned()],
        )
        .expect("solo party should always be valid");
        self.upsert_party(party);
        party_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn party_members_are_deduped_and_stably_sorted() {
        let party = Party::new(
            "party:1",
            "campaign-a",
            "Bravo",
            vec![
                "Charlie".to_owned(),
                "Bravo".to_owned(),
                "Alpha".to_owned(),
                "Bravo".to_owned(),
            ],
        )
        .expect("party should be valid");

        assert_eq!(
            party.members(),
            &vec!["Alpha".to_owned(), "Bravo".to_owned(), "Charlie".to_owned()]
        );
        assert!(party.includes_member("Alpha"));
        assert!(!party.includes_member("Delta"));
    }

    #[test]
    fn ensure_solo_party_is_idempotent_per_actor() {
        let mut registry = PartyRegistry::default();

        let first = registry.ensure_solo_party_for_actor("Alpha", "greenhollow");
        let second = registry.ensure_solo_party_for_actor("Alpha", "greenhollow");

        assert_eq!(first, second);
        assert_eq!(
            registry
                .party_for_actor("Alpha")
                .expect("party should exist")
                .members(),
            &vec!["Alpha".to_owned()]
        );
    }

    #[test]
    fn upsert_party_reassigns_actor_without_stale_membership() {
        let mut registry = PartyRegistry::default();
        registry.upsert_party(
            Party::new(
                "party:1",
                "campaign-a",
                "Alpha",
                vec!["Alpha".to_owned(), "Bravo".to_owned()],
            )
            .expect("party should be valid"),
        );

        registry.upsert_party(
            Party::new(
                "party:2",
                "campaign-a",
                "Charlie",
                vec![
                    "Charlie".to_owned(),
                    "Alpha".to_owned(),
                    "Charlie".to_owned(),
                ],
            )
            .expect("party should be valid"),
        );

        let alpha_party = registry
            .party_for_actor("Alpha")
            .expect("alpha should resolve to a party");
        assert_eq!(alpha_party.party_id, "party:2");

        let bravo_party = registry
            .party_for_actor("Bravo")
            .expect("bravo should resolve to a party");
        assert_eq!(bravo_party.party_id, "party:1");

        let party_one = registry
            .parties
            .get("party:1")
            .expect("party 1 should still exist");
        assert_eq!(party_one.members(), &vec!["Bravo".to_owned()]);
        assert_eq!(party_one.leader_actor_id, "Bravo");
    }

    #[test]
    fn empty_previous_party_is_removed_when_actor_reassigned() {
        let mut registry = PartyRegistry::default();
        registry.upsert_party(
            Party::new(
                "party:solo",
                "campaign-a",
                "Alpha",
                vec!["Alpha".to_owned()],
            )
            .expect("party should be valid"),
        );

        registry.upsert_party(
            Party::new(
                "party:duo",
                "campaign-a",
                "Alpha",
                vec!["Alpha".to_owned(), "Bravo".to_owned()],
            )
            .expect("party should be valid"),
        );

        assert!(!registry.parties.contains_key("party:solo"));
        assert_eq!(
            registry
                .party_for_actor("Alpha")
                .expect("alpha should resolve to a party")
                .party_id,
            "party:duo"
        );
    }
}
