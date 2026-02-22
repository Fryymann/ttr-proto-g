use std::collections::HashMap;
use std::fmt;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::account::{AccountRecord, CharacterRecord};

use super::AuditEvent;

const SNAPSHOT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceState {
    pub next_account_id: u64,
    pub next_audit_id: u64,
    pub accounts_by_handle: HashMap<String, AccountRecord>,
    pub characters_by_name_key: HashMap<String, CharacterRecord>,
    pub audit_log: Vec<AuditEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CampaignSaveSnapshot {
    schema_version: u32,
    campaign_id: String,
    state: PersistenceState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadStatus {
    FreshStart,
    LoadedPrimary,
    RecoveredRollback,
}

#[derive(Debug)]
pub struct LoadResult {
    pub state: Option<PersistenceState>,
    pub status: LoadStatus,
}

pub fn load_with_recovery(
    campaign_id: &str,
    save_path: &Path,
) -> Result<LoadResult, SnapshotError> {
    if !save_path.exists() {
        return Ok(LoadResult {
            state: None,
            status: LoadStatus::FreshStart,
        });
    }

    match read_validated(campaign_id, save_path) {
        Ok(state) => Ok(LoadResult {
            state: Some(state),
            status: LoadStatus::LoadedPrimary,
        }),
        Err(primary_err) => {
            let rollback = rollback_path(save_path);
            if !rollback.exists() {
                return Err(primary_err);
            }

            let recovered = read_validated(campaign_id, &rollback).map_err(|rollback_err| {
                SnapshotError::RecoveryFailed {
                    primary_error: primary_err.to_string(),
                    rollback_error: rollback_err.to_string(),
                }
            })?;

            persist_state(campaign_id, save_path, &recovered)?;
            Ok(LoadResult {
                state: Some(recovered),
                status: LoadStatus::RecoveredRollback,
            })
        }
    }
}

pub fn persist_state(
    campaign_id: &str,
    save_path: &Path,
    state: &PersistenceState,
) -> Result<(), SnapshotError> {
    let snapshot = CampaignSaveSnapshot {
        schema_version: SNAPSHOT_SCHEMA_VERSION,
        campaign_id: campaign_id.to_owned(),
        state: state.clone(),
    };

    let encoded = serde_json::to_vec_pretty(&snapshot).map_err(SnapshotError::Serialize)?;
    atomic_save(save_path, &encoded)
}

fn read_validated(campaign_id: &str, path: &Path) -> Result<PersistenceState, SnapshotError> {
    let raw = fs::read_to_string(path).map_err(|source| SnapshotError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let snapshot: CampaignSaveSnapshot =
        serde_json::from_str(&raw).map_err(|source| SnapshotError::Parse {
            path: path.to_path_buf(),
            source,
        })?;

    if snapshot.schema_version != SNAPSHOT_SCHEMA_VERSION {
        return Err(SnapshotError::SchemaVersionMismatch {
            path: path.to_path_buf(),
            expected: SNAPSHOT_SCHEMA_VERSION,
            found: snapshot.schema_version,
        });
    }

    if snapshot.campaign_id != campaign_id {
        return Err(SnapshotError::CampaignMismatch {
            path: path.to_path_buf(),
            expected: campaign_id.to_owned(),
            found: snapshot.campaign_id,
        });
    }

    Ok(snapshot.state)
}

fn atomic_save(path: &Path, payload: &[u8]) -> Result<(), SnapshotError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| SnapshotError::Io {
            path: parent.to_path_buf(),
            source,
        })?;
    }

    let temp_path = temp_path(path);
    let rollback = rollback_path(path);

    if path.exists() {
        fs::copy(path, &rollback).map_err(|source| SnapshotError::Io {
            path: rollback.clone(),
            source,
        })?;
    }

    let mut file = File::create(&temp_path).map_err(|source| SnapshotError::Io {
        path: temp_path.clone(),
        source,
    })?;
    file.write_all(payload)
        .map_err(|source| SnapshotError::Io {
            path: temp_path.clone(),
            source,
        })?;
    file.sync_all().map_err(|source| SnapshotError::Io {
        path: temp_path.clone(),
        source,
    })?;

    fs::rename(&temp_path, path).map_err(|source| SnapshotError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(())
}

fn temp_path(path: &Path) -> PathBuf {
    path_with_suffix(path, ".tmp")
}

pub fn rollback_path(path: &Path) -> PathBuf {
    path_with_suffix(path, ".rollback")
}

fn path_with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut raw = path.as_os_str().to_os_string();
    raw.push(suffix);
    PathBuf::from(raw)
}

#[derive(Debug)]
pub enum SnapshotError {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    Serialize(serde_json::Error),
    Parse {
        path: PathBuf,
        source: serde_json::Error,
    },
    CampaignMismatch {
        path: PathBuf,
        expected: String,
        found: String,
    },
    SchemaVersionMismatch {
        path: PathBuf,
        expected: u32,
        found: u32,
    },
    RecoveryFailed {
        primary_error: String,
        rollback_error: String,
    },
}

impl fmt::Display for SnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => {
                write!(f, "snapshot IO failure at {}: {}", path.display(), source)
            }
            Self::Serialize(source) => write!(f, "failed to serialize snapshot: {}", source),
            Self::Parse { path, source } => write!(
                f,
                "failed to parse snapshot JSON at {}: {}",
                path.display(),
                source
            ),
            Self::CampaignMismatch {
                path,
                expected,
                found,
            } => write!(
                f,
                "campaign mismatch in snapshot {} (expected '{}', found '{}')",
                path.display(),
                expected,
                found
            ),
            Self::SchemaVersionMismatch {
                path,
                expected,
                found,
            } => write!(
                f,
                "snapshot schema mismatch in {} (expected {}, found {})",
                path.display(),
                expected,
                found
            ),
            Self::RecoveryFailed {
                primary_error,
                rollback_error,
            } => write!(
                f,
                "primary snapshot invalid and rollback recovery failed (primary: {}; rollback: {})",
                primary_error, rollback_error
            ),
        }
    }
}

impl std::error::Error for SnapshotError {}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use ttrpg_protocol::CharacterDraft;

    use crate::account::{build_campaign_lock, CharacterRecord};

    use super::*;

    #[test]
    fn snapshot_round_trip_loads_primary_state() {
        let root = unique_temp_dir("snapshot_round_trip");
        let save_path = root.join("campaign.json");
        let state = sample_state("greenhollow", "Ada");

        persist_state("greenhollow", &save_path, &state).expect("save should succeed");

        let loaded = load_with_recovery("greenhollow", &save_path).expect("load should succeed");
        assert_eq!(loaded.status, LoadStatus::LoadedPrimary);
        let restored = loaded.state.expect("state should be present");
        assert_eq!(
            restored.characters_by_name_key["ada"].character.name,
            "Ada".to_owned()
        );
    }

    #[test]
    fn recovery_uses_rollback_when_primary_snapshot_is_corrupted() {
        let root = unique_temp_dir("snapshot_recovery");
        let save_path = root.join("campaign.json");

        let baseline = sample_state("greenhollow", "Ada");
        persist_state("greenhollow", &save_path, &baseline).expect("baseline save should succeed");

        let upgraded = sample_state("greenhollow", "Bea");
        persist_state("greenhollow", &save_path, &upgraded).expect("upgrade save should succeed");

        fs::write(&save_path, b"{\"broken\":").expect("corrupt primary snapshot");

        let recovered =
            load_with_recovery("greenhollow", &save_path).expect("recovery should work");
        assert_eq!(recovered.status, LoadStatus::RecoveredRollback);
        let state = recovered.state.expect("state should recover");
        assert!(state.characters_by_name_key.contains_key("ada"));
        assert!(!state.characters_by_name_key.contains_key("bea"));

        let post_heal =
            load_with_recovery("greenhollow", &save_path).expect("healed load should work");
        assert_eq!(post_heal.status, LoadStatus::LoadedPrimary);
    }

    #[test]
    fn load_rejects_campaign_mismatch() {
        let root = unique_temp_dir("snapshot_campaign_mismatch");
        let save_path = root.join("campaign.json");
        let state = sample_state("greenhollow", "Ada");
        persist_state("greenhollow", &save_path, &state).expect("save should succeed");

        let error = load_with_recovery("ashfall", &save_path)
            .expect_err("should reject mismatched campaign");
        assert!(matches!(error, SnapshotError::CampaignMismatch { .. }));
    }

    fn sample_state(campaign_id: &str, character_name: &str) -> PersistenceState {
        let name_key = character_name.to_lowercase();
        let mut accounts = HashMap::new();
        accounts.insert(
            "acct:ada".to_owned(),
            AccountRecord {
                account_id: "acct-1".to_owned(),
                handle: "acct:ada".to_owned(),
                created_at_epoch_secs: 0,
                status: "active".to_owned(),
            },
        );

        let mut characters = HashMap::new();
        characters.insert(
            name_key.clone(),
            CharacterRecord {
                character_id: format!("char-{}", name_key),
                account_id: "acct-1".to_owned(),
                character: CharacterDraft {
                    name: character_name.to_owned(),
                    ancestry: "Human".to_owned(),
                    class_name: "Fighter".to_owned(),
                    background: "Soldier".to_owned(),
                    pronouns: "they/them".to_owned(),
                    motto: "Forward.".to_owned(),
                },
                species: "Human".to_owned(),
                class: "Fighter".to_owned(),
                stats: "{}".to_owned(),
                inventory_ref: "none".to_owned(),
                campaign_lock: Some(build_campaign_lock(campaign_id, None)),
            },
        );

        PersistenceState {
            next_account_id: 1,
            next_audit_id: 0,
            accounts_by_handle: accounts,
            characters_by_name_key: characters,
            audit_log: Vec::new(),
        }
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
}
