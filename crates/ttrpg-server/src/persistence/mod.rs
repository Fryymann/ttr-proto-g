use std::collections::HashMap;
use std::fmt;

use ttrpg_protocol::CharacterDraft;

use crate::account::{ensure_campaign_lock, AccountRecord, CampaignJoinDecision, CharacterRecord};

pub trait Persistence: Send + Sync {
    fn create_character(
        &mut self,
        account_handle: &str,
        character: CharacterDraft,
        name_key: &str,
        active_campaign_id: &str,
    ) -> Result<CharacterRecord, CreateCharacterError>;

    fn validate_character_join(
        &mut self,
        name_key: &str,
        active_campaign_id: &str,
    ) -> Result<CharacterRecord, JoinCampaignError>;

    fn log_audit_event(&mut self, event: AuditEvent);
}

#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub id: String,
    pub event_type: String,
    pub actor: String,
    pub payload: String,
    pub timestamp_epoch_secs: u64,
}

#[derive(Debug, Default)]
pub struct InMemoryPersistence {
    next_account_id: u64,
    accounts_by_handle: HashMap<String, AccountRecord>,
    characters_by_name_key: HashMap<String, CharacterRecord>,
    audit_log: Vec<AuditEvent>,
}

impl Persistence for InMemoryPersistence {
    fn create_character(
        &mut self,
        account_handle: &str,
        character: CharacterDraft,
        name_key: &str,
        active_campaign_id: &str,
    ) -> Result<CharacterRecord, CreateCharacterError> {
        if self.characters_by_name_key.contains_key(name_key) {
            return Err(CreateCharacterError::CharacterNameExists);
        }

        let account = self.ensure_account(account_handle).clone();
        let record = CharacterRecord {
            character_id: format!("char-{}", name_key),
            account_id: account.account_id,
            character: character.clone(),
            species: character.ancestry.clone(),
            class: character.class_name.clone(),
            stats: "{}".to_owned(),
            inventory_ref: "none".to_owned(),
            campaign_lock: None,
        };

        let mut record_mut = record;
        let decision = ensure_campaign_lock(&mut record_mut.campaign_lock, active_campaign_id);
        debug_assert!(matches!(decision, CampaignJoinDecision::Allowed));

        self.characters_by_name_key
            .insert(name_key.to_owned(), record_mut.clone());

        Ok(record_mut)
    }

    fn validate_character_join(
        &mut self,
        name_key: &str,
        active_campaign_id: &str,
    ) -> Result<CharacterRecord, JoinCampaignError> {
        let Some(record) = self.characters_by_name_key.get_mut(name_key) else {
            return Err(JoinCampaignError::CharacterNotFound);
        };

        match ensure_campaign_lock(&mut record.campaign_lock, active_campaign_id) {
            CampaignJoinDecision::Allowed => Ok(record.clone()),
            CampaignJoinDecision::LockedToOtherCampaign { locked_campaign_id } => {
                Err(JoinCampaignError::LockedToOtherCampaign { locked_campaign_id })
            }
        }
    }

    fn log_audit_event(&mut self, event: AuditEvent) {
        self.audit_log.push(event);
    }
}

impl InMemoryPersistence {
    fn ensure_account(&mut self, account_handle: &str) -> &AccountRecord {
        self.accounts_by_handle
            .entry(account_handle.to_owned())
            .or_insert_with(|| {
                let next = self.next_account_id + 1;
                self.next_account_id = next;
                AccountRecord {
                    account_id: format!("acct-{}", next),
                    handle: account_handle.to_owned(),
                    created_at_epoch_secs: 0,
                    status: "active".to_owned(),
                }
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateCharacterError {
    CharacterNameExists,
}

impl fmt::Display for CreateCharacterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CharacterNameExists => {
                write!(f, "character already exists")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JoinCampaignError {
    CharacterNotFound,
    LockedToOtherCampaign { locked_campaign_id: String },
}

impl fmt::Display for JoinCampaignError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CharacterNotFound => write!(f, "character does not exist"),
            Self::LockedToOtherCampaign { locked_campaign_id } => write!(
                f,
                "character is locked to campaign '{}'",
                locked_campaign_id
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn character_locks_to_first_campaign_join() {
        let mut store = InMemoryPersistence::default();
        let character = CharacterDraft {
            name: "Ada".to_owned(),
            ancestry: "Human".to_owned(),
            class_name: "Fighter".to_owned(),
            background: "Soldier".to_owned(),
            pronouns: "she/her".to_owned(),
            motto: "Forward.".to_owned(),
        };

        let record = store
            .create_character("acct:ada", character, "ada", "greenhollow")
            .expect("character should create");

        assert_eq!(
            record
                .campaign_lock
                .as_ref()
                .map(|lock| lock.campaign_id.as_str()),
            Some("greenhollow")
        );
    }

    #[test]
    fn join_is_rejected_when_locked_to_other_campaign() {
        let mut store = InMemoryPersistence::default();
        let character = CharacterDraft {
            name: "Ada".to_owned(),
            ancestry: "Human".to_owned(),
            class_name: "Fighter".to_owned(),
            background: "Soldier".to_owned(),
            pronouns: "she/her".to_owned(),
            motto: "Forward.".to_owned(),
        };

        store
            .create_character("acct:ada", character, "ada", "greenhollow")
            .expect("character should create");

        let result = store.validate_character_join("ada", "ashfall");

        assert!(matches!(
            result,
            Err(JoinCampaignError::LockedToOtherCampaign { locked_campaign_id })
            if locked_campaign_id == "greenhollow"
        ));
    }
}
