use std::time::{SystemTime, UNIX_EPOCH};

use ttrpg_protocol::CharacterDraft;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AccountRecord {
    pub account_id: String,
    pub handle: String,
    pub created_at_epoch_secs: u64,
    pub status: String, // e.g., "active", "suspended"
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CharacterCampaignLock {
    pub campaign_id: String,
    pub locked_at_epoch_secs: u64,
    pub unlock_audit_ref: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CharacterRecord {
    pub character_id: String,
    pub account_id: String,
    pub character: CharacterDraft,
    pub species: String,
    pub class: String,
    pub stats: String,         // Placeholder for complex stats struct
    pub inventory_ref: String, // Placeholder for inventory ID
    pub campaign_lock: Option<CharacterCampaignLock>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CampaignJoinDecision {
    Allowed,
    LockedToOtherCampaign { locked_campaign_id: String },
}

pub fn evaluate_campaign_join(
    lock: Option<&CharacterCampaignLock>,
    active_campaign_id: &str,
) -> CampaignJoinDecision {
    let Some(lock) = lock else {
        return CampaignJoinDecision::Allowed;
    };

    if lock.campaign_id == active_campaign_id {
        CampaignJoinDecision::Allowed
    } else {
        CampaignJoinDecision::LockedToOtherCampaign {
            locked_campaign_id: lock.campaign_id.clone(),
        }
    }
}

pub fn ensure_campaign_lock(
    lock: &mut Option<CharacterCampaignLock>,
    active_campaign_id: &str,
) -> CampaignJoinDecision {
    match evaluate_campaign_join(lock.as_ref(), active_campaign_id) {
        CampaignJoinDecision::Allowed => {
            if lock.is_none() {
                *lock = Some(build_campaign_lock(active_campaign_id, None));
            }

            CampaignJoinDecision::Allowed
        }
        blocked => blocked,
    }
}

pub fn build_campaign_lock(
    campaign_id: &str,
    unlock_audit_ref: Option<String>,
) -> CharacterCampaignLock {
    CharacterCampaignLock {
        campaign_id: campaign_id.to_owned(),
        locked_at_epoch_secs: unix_now_secs(),
        unlock_audit_ref,
    }
}

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
    fn lock_is_created_for_unlocked_character() {
        let mut lock = None;

        let decision = ensure_campaign_lock(&mut lock, "greenhollow");

        assert_eq!(decision, CampaignJoinDecision::Allowed);
        assert_eq!(
            lock.as_ref().map(|saved| saved.campaign_id.as_str()),
            Some("greenhollow")
        );
    }

    #[test]
    fn lock_rejects_other_campaign() {
        let mut lock = Some(CharacterCampaignLock {
            campaign_id: "ashfall".to_owned(),
            locked_at_epoch_secs: 1,
            unlock_audit_ref: None,
        });

        let decision = ensure_campaign_lock(&mut lock, "greenhollow");

        assert_eq!(
            decision,
            CampaignJoinDecision::LockedToOtherCampaign {
                locked_campaign_id: "ashfall".to_owned()
            }
        );
    }
}
