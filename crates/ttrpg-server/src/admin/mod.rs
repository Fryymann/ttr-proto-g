use std::env;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdminUnlockCommand {
    pub character_name_key: String,
    pub target_campaign_id: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdminCommandError {
    Usage,
    ReasonRequired,
}

impl AdminUnlockCommand {
    pub fn parse(args: &[&str]) -> Result<Self, AdminCommandError> {
        if args.len() < 4 {
            return Err(AdminCommandError::Usage);
        }

        let character_name_key = args[0].trim().to_lowercase();
        let target_campaign_id = args[1].trim().to_owned();
        if character_name_key.is_empty() || target_campaign_id.is_empty() {
            return Err(AdminCommandError::Usage);
        }

        if args[2] != "--reason" {
            return Err(AdminCommandError::Usage);
        }

        let reason = args[3..].join(" ").trim().to_owned();
        if reason.is_empty() {
            return Err(AdminCommandError::ReasonRequired);
        }

        Ok(Self {
            character_name_key,
            target_campaign_id,
            reason,
        })
    }
}

pub fn is_admin_authorized(account_handle: &str) -> bool {
    if account_handle.starts_with("acct:admin") {
        return true;
    }

    let configured = env::var("TTRPG_ADMIN_ACCOUNTS").unwrap_or_default();
    configured
        .split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .any(|entry| {
            let normalized = if entry.starts_with("acct:") {
                entry.to_owned()
            } else {
                format!("acct:{}", entry.to_lowercase())
            };
            normalized == account_handle
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_unlock_command_requires_reason_flag() {
        let parsed = AdminUnlockCommand::parse(&["ada", "ashfall", "--reason", "support", "42"])
            .expect("command should parse");
        assert_eq!(parsed.character_name_key, "ada");
        assert_eq!(parsed.target_campaign_id, "ashfall");
        assert_eq!(parsed.reason, "support 42");
    }

    #[test]
    fn parse_unlock_command_rejects_missing_reason() {
        let result = AdminUnlockCommand::parse(&["ada", "ashfall", "--reason"]);
        assert_eq!(result, Err(AdminCommandError::Usage));
    }
}
