use std::collections::HashSet;
use std::env;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdminUnlockCommand {
    pub character_name_key: String,
    pub target_campaign_id: String,
    pub token: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdminCommandError {
    Usage,
    TokenRequired,
    ReasonRequired,
}

impl AdminUnlockCommand {
    pub fn parse(args: &[&str]) -> Result<Self, AdminCommandError> {
        if args.len() < 6 {
            return Err(AdminCommandError::Usage);
        }

        let character_name_key = args[0].trim().to_lowercase();
        let target_campaign_id = args[1].trim().to_owned();
        if character_name_key.is_empty() || target_campaign_id.is_empty() {
            return Err(AdminCommandError::Usage);
        }

        if args[2] != "--token" {
            return Err(AdminCommandError::Usage);
        }
        let token = args[3].trim().to_owned();
        if token.is_empty() {
            return Err(AdminCommandError::TokenRequired);
        }

        if args[4] != "--reason" {
            return Err(AdminCommandError::Usage);
        }

        let reason = args[5..].join(" ").trim().to_owned();
        if reason.is_empty() {
            return Err(AdminCommandError::ReasonRequired);
        }

        Ok(Self {
            character_name_key,
            target_campaign_id,
            token,
            reason,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AdminPolicy {
    pub allowed_accounts: HashSet<String>,
    pub unlock_token: String,
}

impl AdminPolicy {
    pub fn from_env() -> Self {
        Self {
            allowed_accounts: configured_admin_accounts(),
            unlock_token: env::var("TTRPG_ADMIN_UNLOCK_TOKEN").unwrap_or_default(),
        }
    }

    #[cfg(test)]
    pub fn for_tests(accounts: &[&str], unlock_token: &str) -> Self {
        Self {
            allowed_accounts: accounts
                .iter()
                .map(|entry| normalize_account_entry(entry))
                .collect::<HashSet<_>>(),
            unlock_token: unlock_token.to_owned(),
        }
    }

    pub fn is_authorized(&self, account_handle: &str, presented_token: &str) -> bool {
        if self.unlock_token.is_empty() {
            return false;
        }

        self.allowed_accounts.contains(account_handle) && presented_token == self.unlock_token
    }
}

fn configured_admin_accounts() -> HashSet<String> {
    let configured = env::var("TTRPG_ADMIN_ACCOUNTS").unwrap_or_default();
    configured
        .split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(normalize_account_entry)
        .collect::<HashSet<_>>()
}

fn normalize_account_entry(entry: &str) -> String {
    let lowered = entry.trim().to_lowercase();
    if let Some(suffix) = lowered.strip_prefix("acct:") {
        format!("acct:{}", suffix)
    } else {
        format!("acct:{}", lowered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_unlock_command_requires_token_and_reason_flags() {
        let parsed = AdminUnlockCommand::parse(&[
            "ada",
            "ashfall",
            "--token",
            "admin-secret",
            "--reason",
            "support",
            "42",
        ])
        .expect("command should parse");
        assert_eq!(parsed.character_name_key, "ada");
        assert_eq!(parsed.target_campaign_id, "ashfall");
        assert_eq!(parsed.token, "admin-secret");
        assert_eq!(parsed.reason, "support 42");
    }

    #[test]
    fn parse_unlock_command_rejects_missing_reason_or_token() {
        let missing_reason =
            AdminUnlockCommand::parse(&["ada", "ashfall", "--token", "admin-secret", "--reason"]);
        assert_eq!(missing_reason, Err(AdminCommandError::Usage));

        let missing_token =
            AdminUnlockCommand::parse(&["ada", "ashfall", "--token", "", "--reason", "support"]);
        assert_eq!(missing_token, Err(AdminCommandError::TokenRequired));
    }

    #[test]
    fn admin_policy_requires_allowlist_and_token() {
        let policy = AdminPolicy::for_tests(&["acct:admin_ops"], "topsecret");
        assert!(policy.is_authorized("acct:admin_ops", "topsecret"));
        assert!(!policy.is_authorized("acct:admin_ops", "wrong"));
        assert!(!policy.is_authorized("acct:not_admin", "topsecret"));

        let no_token = AdminPolicy::for_tests(&["acct:admin_ops"], "");
        assert!(!no_token.is_authorized("acct:admin_ops", ""));

        let normalized = AdminPolicy::for_tests(&["admin_tools"], "topsecret");
        assert!(normalized.is_authorized("acct:admin_tools", "topsecret"));

        let mixed_case = AdminPolicy::for_tests(&["acct:Admin_Ops"], "topsecret");
        assert!(mixed_case.is_authorized("acct:admin_ops", "topsecret"));
    }

    #[test]
    fn parse_unlock_command_rejects_missing_flags() {
        let result = AdminUnlockCommand::parse(&["ada", "ashfall", "--reason", "foo"]);
        assert_eq!(result, Err(AdminCommandError::Usage));
    }
}
