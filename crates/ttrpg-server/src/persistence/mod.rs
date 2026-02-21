use std::collections::HashMap;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::json;
use ttrpg_protocol::CharacterDraft;

use crate::account::{
    build_campaign_lock, ensure_campaign_lock, AccountRecord, CampaignJoinDecision, CharacterRecord,
};

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
        account_handle: &str,
        name_key: &str,
        active_campaign_id: &str,
    ) -> Result<CharacterRecord, JoinCampaignError>;

    #[allow(dead_code)]
    fn admin_override_campaign_lock(
        &mut self,
        name_key: &str,
        target_campaign_id: &str,
        actor: &str,
        reason: &str,
    ) -> Result<CharacterRecord, CampaignUnlockError>;

    #[allow(dead_code)]
    fn audit_events(&self) -> &[AuditEvent];
}

#[allow(dead_code)]
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
    #[allow(dead_code)]
    next_audit_id: u64,
    accounts_by_handle: HashMap<String, AccountRecord>,
    characters_by_name_key: HashMap<String, CharacterRecord>,
    #[allow(dead_code)]
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
        account_handle: &str,
        name_key: &str,
        active_campaign_id: &str,
    ) -> Result<CharacterRecord, JoinCampaignError> {
        let Some(account) = self.accounts_by_handle.get(account_handle) else {
            return Err(JoinCampaignError::AccountNotFound {
                account_handle: account_handle.to_owned(),
            });
        };

        let Some(record) = self.characters_by_name_key.get_mut(name_key) else {
            return Err(JoinCampaignError::CharacterNotFound);
        };

        if record.account_id != account.account_id {
            return Err(JoinCampaignError::OwnershipMismatch {
                character_id: record.character_id.clone(),
                expected_account_id: record.account_id.clone(),
                presented_account_id: account.account_id.clone(),
            });
        }

        match ensure_campaign_lock(&mut record.campaign_lock, active_campaign_id) {
            CampaignJoinDecision::Allowed => Ok(record.clone()),
            CampaignJoinDecision::LockedToOtherCampaign { locked_campaign_id } => {
                Err(JoinCampaignError::LockedToOtherCampaign { locked_campaign_id })
            }
        }
    }

    fn admin_override_campaign_lock(
        &mut self,
        name_key: &str,
        target_campaign_id: &str,
        actor: &str,
        reason: &str,
    ) -> Result<CharacterRecord, CampaignUnlockError> {
        if actor.trim().is_empty() {
            return Err(CampaignUnlockError::ActorRequired);
        }
        if reason.trim().is_empty() {
            return Err(CampaignUnlockError::ReasonRequired);
        }
        if target_campaign_id.trim().is_empty() {
            return Err(CampaignUnlockError::TargetCampaignRequired);
        }

        let Some(current_record) = self.characters_by_name_key.get(name_key).cloned() else {
            return Err(CampaignUnlockError::CharacterNotFound);
        };

        let previous_campaign_id = current_record
            .campaign_lock
            .as_ref()
            .map(|lock| lock.campaign_id.clone());
        let payload = json!({
            "character_id": current_record.character_id,
            "account_id": current_record.account_id,
            "previous_campaign_id": previous_campaign_id,
            "target_campaign_id": target_campaign_id,
            "reason": reason,
        })
        .to_string();
        let audit_id = self.append_audit_event("campaign_unlock_override", actor, payload);

        let record = self
            .characters_by_name_key
            .get_mut(name_key)
            .expect("record must exist after pre-check");
        record.campaign_lock = Some(build_campaign_lock(
            target_campaign_id,
            Some(audit_id.clone()),
        ));

        Ok(record.clone())
    }

    fn audit_events(&self) -> &[AuditEvent] {
        &self.audit_log
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

    #[allow(dead_code)]
    fn append_audit_event(&mut self, event_type: &str, actor: &str, payload: String) -> String {
        self.next_audit_id += 1;
        let id = format!("audit-{}", self.next_audit_id);
        self.audit_log.push(AuditEvent {
            id: id.clone(),
            event_type: event_type.to_owned(),
            actor: actor.to_owned(),
            payload,
            timestamp_epoch_secs: unix_now_secs(),
        });
        id
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
    AccountNotFound {
        account_handle: String,
    },
    OwnershipMismatch {
        character_id: String,
        expected_account_id: String,
        presented_account_id: String,
    },
    LockedToOtherCampaign {
        locked_campaign_id: String,
    },
}

impl fmt::Display for JoinCampaignError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CharacterNotFound => write!(f, "character does not exist"),
            Self::AccountNotFound { account_handle } => {
                write!(f, "account handle '{}' does not exist", account_handle)
            }
            Self::OwnershipMismatch {
                character_id,
                expected_account_id,
                presented_account_id,
            } => write!(
                f,
                "character '{}' belongs to account '{}' not '{}'",
                character_id, expected_account_id, presented_account_id
            ),
            Self::LockedToOtherCampaign { locked_campaign_id } => write!(
                f,
                "character is locked to campaign '{}'",
                locked_campaign_id
            ),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CampaignUnlockError {
    CharacterNotFound,
    TargetCampaignRequired,
    ActorRequired,
    ReasonRequired,
}

impl fmt::Display for CampaignUnlockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CharacterNotFound => write!(f, "character does not exist"),
            Self::TargetCampaignRequired => write!(f, "target campaign is required"),
            Self::ActorRequired => write!(f, "unlock actor is required"),
            Self::ReasonRequired => write!(f, "unlock reason is required"),
        }
    }
}

#[allow(dead_code)]
fn unix_now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
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

        let result = store.validate_character_join("acct:ada", "ada", "ashfall");

        assert!(matches!(
            result,
            Err(JoinCampaignError::LockedToOtherCampaign { locked_campaign_id })
            if locked_campaign_id == "greenhollow"
        ));
    }

    #[test]
    fn join_rejects_when_account_ownership_mismatch_is_detected() {
        let mut store = InMemoryPersistence::default();
        store
            .create_character("acct:ada", build_character("Ada"), "ada", "greenhollow")
            .expect("ada should create");
        store
            .create_character("acct:eve", build_character("Eve"), "eve", "greenhollow")
            .expect("eve should create");

        let eve_account_id = store
            .accounts_by_handle
            .get("acct:eve")
            .expect("eve account should exist")
            .account_id
            .clone();
        let ada_record = store
            .characters_by_name_key
            .get_mut("ada")
            .expect("ada character should exist");
        ada_record.account_id = eve_account_id;

        let result = store.validate_character_join("acct:ada", "ada", "greenhollow");
        assert!(matches!(
            result,
            Err(JoinCampaignError::OwnershipMismatch { character_id, .. })
            if character_id == "char-ada"
        ));
    }

    #[test]
    fn admin_override_rebinds_lock_and_appends_audit_event() {
        let mut store = InMemoryPersistence::default();
        store
            .create_character("acct:ada", build_character("Ada"), "ada", "greenhollow")
            .expect("character should create");

        let overridden = store
            .admin_override_campaign_lock("ada", "ashfall", "admin:ops", "support ticket #42")
            .expect("override should succeed");

        assert_eq!(
            overridden
                .campaign_lock
                .as_ref()
                .map(|lock| lock.campaign_id.as_str()),
            Some("ashfall")
        );
        assert_eq!(
            overridden
                .campaign_lock
                .as_ref()
                .and_then(|lock| lock.unlock_audit_ref.as_deref()),
            Some("audit-1")
        );

        let audit_log = store.audit_events();
        assert_eq!(audit_log.len(), 1);
        assert_eq!(audit_log[0].id, "audit-1");
        assert_eq!(audit_log[0].event_type, "campaign_unlock_override");
        assert_eq!(audit_log[0].actor, "admin:ops");
        assert!(audit_log[0]
            .payload
            .contains("\"target_campaign_id\":\"ashfall\""));
        assert!(audit_log[0]
            .payload
            .contains("\"reason\":\"support ticket #42\""));

        let join = store.validate_character_join("acct:ada", "ada", "ashfall");
        assert!(join.is_ok());
    }

    #[test]
    fn admin_override_requires_actor_and_reason() {
        let mut store = InMemoryPersistence::default();
        store
            .create_character("acct:ada", build_character("Ada"), "ada", "greenhollow")
            .expect("character should create");

        let missing_actor = store.admin_override_campaign_lock("ada", "ashfall", "", "ticket");
        assert!(matches!(
            missing_actor,
            Err(CampaignUnlockError::ActorRequired)
        ));

        let missing_reason = store.admin_override_campaign_lock("ada", "ashfall", "admin:ops", "");
        assert!(matches!(
            missing_reason,
            Err(CampaignUnlockError::ReasonRequired)
        ));
    }

    fn build_character(name: &str) -> CharacterDraft {
        CharacterDraft {
            name: name.to_owned(),
            ancestry: "Human".to_owned(),
            class_name: "Fighter".to_owned(),
            background: "Soldier".to_owned(),
            pronouns: "they/them".to_owned(),
            motto: "Forward.".to_owned(),
        }
    }
}
