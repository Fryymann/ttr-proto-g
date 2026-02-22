mod snapshot;

use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::error;
use ttrpg_protocol::CharacterDraft;

use crate::account::{
    build_campaign_lock, ensure_campaign_lock, AccountRecord, CampaignJoinDecision, CharacterRecord,
};

pub use snapshot::LoadStatus as SnapshotLoadStatus;

pub trait Persistence: Send + Sync {
    fn ensure_account_session(&mut self, account_handle: &str) -> AccountRecord;

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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    fn ensure_account_session(&mut self, account_handle: &str) -> AccountRecord {
        self.ensure_account(account_handle).clone()
    }

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

    fn snapshot_state(&self) -> snapshot::PersistenceState {
        snapshot::PersistenceState {
            next_account_id: self.next_account_id,
            next_audit_id: self.next_audit_id,
            accounts_by_handle: self.accounts_by_handle.clone(),
            characters_by_name_key: self.characters_by_name_key.clone(),
            audit_log: self.audit_log.clone(),
        }
    }

    fn from_snapshot_state(state: snapshot::PersistenceState) -> Self {
        Self {
            next_account_id: state.next_account_id,
            next_audit_id: state.next_audit_id,
            accounts_by_handle: state.accounts_by_handle,
            characters_by_name_key: state.characters_by_name_key,
            audit_log: state.audit_log,
        }
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

#[derive(Debug)]
pub struct FilePersistence {
    campaign_id: String,
    save_path: PathBuf,
    store: InMemoryPersistence,
}

impl FilePersistence {
    pub fn open(
        campaign_id: &str,
        save_path: impl AsRef<Path>,
    ) -> Result<(Self, SnapshotLoadStatus), PersistenceInitError> {
        let save_path = save_path.as_ref().to_path_buf();
        let loaded = snapshot::load_with_recovery(campaign_id, &save_path)
            .map_err(PersistenceInitError::Snapshot)?;

        let store = match loaded.state {
            Some(state) => InMemoryPersistence::from_snapshot_state(state),
            None => InMemoryPersistence::default(),
        };

        let persistence = Self {
            campaign_id: campaign_id.to_owned(),
            save_path,
            store,
        };

        if loaded.status == SnapshotLoadStatus::FreshStart {
            persistence
                .persist_now()
                .map_err(PersistenceInitError::BootstrapPersist)?;
        }

        Ok((persistence, loaded.status))
    }

    fn persist_now(&self) -> Result<(), String> {
        snapshot::persist_state(
            &self.campaign_id,
            &self.save_path,
            &self.store.snapshot_state(),
        )
        .map_err(|err| err.to_string())
    }

    fn persist_or_log(&self, operation: &str) {
        if let Err(err) = self.persist_now() {
            error!(
                operation = operation,
                campaign_id = %self.campaign_id,
                save_path = %self.save_path.display(),
                error = %err,
                "file persistence write failed"
            );
        }
    }
}

impl Persistence for FilePersistence {
    fn ensure_account_session(&mut self, account_handle: &str) -> AccountRecord {
        let record = self.store.ensure_account_session(account_handle);
        self.persist_or_log("ensure_account_session");
        record
    }

    fn create_character(
        &mut self,
        account_handle: &str,
        character: CharacterDraft,
        name_key: &str,
        active_campaign_id: &str,
    ) -> Result<CharacterRecord, CreateCharacterError> {
        let pre_mutation = self.store.snapshot_state();
        let record =
            self.store
                .create_character(account_handle, character, name_key, active_campaign_id)?;

        if let Err(err) = self.persist_now() {
            self.store = InMemoryPersistence::from_snapshot_state(pre_mutation);
            return Err(CreateCharacterError::PersistFailed(err));
        }

        Ok(record)
    }

    fn validate_character_join(
        &mut self,
        account_handle: &str,
        name_key: &str,
        active_campaign_id: &str,
    ) -> Result<CharacterRecord, JoinCampaignError> {
        let pre_mutation = self.store.snapshot_state();
        let record =
            self.store
                .validate_character_join(account_handle, name_key, active_campaign_id)?;

        if let Err(err) = self.persist_now() {
            self.store = InMemoryPersistence::from_snapshot_state(pre_mutation);
            return Err(JoinCampaignError::PersistFailed(err));
        }

        Ok(record)
    }

    fn admin_override_campaign_lock(
        &mut self,
        name_key: &str,
        target_campaign_id: &str,
        actor: &str,
        reason: &str,
    ) -> Result<CharacterRecord, CampaignUnlockError> {
        let pre_mutation = self.store.snapshot_state();
        let record =
            self.store
                .admin_override_campaign_lock(name_key, target_campaign_id, actor, reason)?;

        if let Err(err) = self.persist_now() {
            self.store = InMemoryPersistence::from_snapshot_state(pre_mutation);
            return Err(CampaignUnlockError::PersistFailed(err));
        }

        Ok(record)
    }

    fn audit_events(&self) -> &[AuditEvent] {
        self.store.audit_events()
    }
}

#[derive(Debug)]
pub enum PersistenceInitError {
    Snapshot(snapshot::SnapshotError),
    BootstrapPersist(String),
}

impl fmt::Display for PersistenceInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Snapshot(err) => write!(f, "failed to initialize snapshot persistence: {}", err),
            Self::BootstrapPersist(err) => {
                write!(f, "failed to write initial snapshot save file: {}", err)
            }
        }
    }
}

impl std::error::Error for PersistenceInitError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreateCharacterError {
    CharacterNameExists,
    PersistFailed(String),
}

impl fmt::Display for CreateCharacterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CharacterNameExists => write!(f, "character already exists"),
            Self::PersistFailed(err) => write!(f, "character created but save failed: {}", err),
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
    PersistFailed(String),
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
            Self::PersistFailed(err) => {
                write!(f, "character join processed but save failed: {}", err)
            }
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
    PersistFailed(String),
}

impl fmt::Display for CampaignUnlockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CharacterNotFound => write!(f, "character does not exist"),
            Self::TargetCampaignRequired => write!(f, "target campaign is required"),
            Self::ActorRequired => write!(f, "unlock actor is required"),
            Self::ReasonRequired => write!(f, "unlock reason is required"),
            Self::PersistFailed(err) => write!(f, "unlock applied but save failed: {}", err),
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
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

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
        store.ensure_account_session("acct:eve");

        let result = store.validate_character_join("acct:eve", "ada", "greenhollow");
        assert!(matches!(
            result,
            Err(JoinCampaignError::OwnershipMismatch { character_id, .. })
            if character_id == "char-ada"
        ));
    }

    #[test]
    fn join_succeeds_for_valid_same_account_character() {
        let mut store = InMemoryPersistence::default();
        store
            .create_character("acct:ada", build_character("Ada"), "ada", "greenhollow")
            .expect("character should create");

        let join = store.validate_character_join("acct:ada", "ada", "greenhollow");
        assert!(join.is_ok());
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

    #[test]
    fn file_persistence_recovers_from_corrupted_primary_snapshot() {
        let root = unique_temp_dir("file_persistence_recovery");
        let save_path = root.join("campaign.json");

        let (mut store, first_status) = FilePersistence::open("greenhollow", &save_path)
            .expect("initial bootstrap should work");
        assert_eq!(first_status, SnapshotLoadStatus::FreshStart);

        store
            .create_character("acct:ada", build_character("Ada"), "ada", "greenhollow")
            .expect("first character should persist");
        drop(store);

        let (mut store, second_status) =
            FilePersistence::open("greenhollow", &save_path).expect("primary reload should work");
        assert_eq!(second_status, SnapshotLoadStatus::LoadedPrimary);

        store
            .create_character("acct:ada", build_character("Bea"), "bea", "greenhollow")
            .expect("second snapshot should persist and create rollback");
        drop(store);

        assert!(snapshot::rollback_path(&save_path).exists());
        fs::write(&save_path, b"{\"corrupted\":").expect("simulate partial/corrupt write");

        let (mut recovered, recovery_status) = FilePersistence::open("greenhollow", &save_path)
            .expect("rollback recovery should work");
        assert_eq!(recovery_status, SnapshotLoadStatus::RecoveredRollback);

        let ada_join = recovered.validate_character_join("acct:ada", "ada", "greenhollow");
        assert!(ada_join.is_ok());

        let bea_join = recovered.validate_character_join("acct:ada", "bea", "greenhollow");
        assert!(matches!(
            bea_join,
            Err(JoinCampaignError::CharacterNotFound)
        ));
    }

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be monotonic")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("ttrpg-{}-{}", prefix, nonce));
        fs::create_dir_all(&dir).expect("temp dir should create");
        dir
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
