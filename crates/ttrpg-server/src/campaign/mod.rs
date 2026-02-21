use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignCatalog {
    pub campaigns: Vec<CampaignManifestEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignManifestEntry {
    pub campaign_id: String,
    pub name: String,
    pub content_version: String,
    pub entry_scene_id: String,
    pub save_path: String,
}

impl CampaignCatalog {
    pub fn load_from_path(path: &Path) -> Result<Self, CampaignLoadError> {
        let raw = fs::read_to_string(path).map_err(CampaignLoadError::Io)?;
        Self::load_from_json(&raw)
    }

    pub fn load_from_json(raw: &str) -> Result<Self, CampaignLoadError> {
        let catalog: Self = serde_json::from_str(raw).map_err(CampaignLoadError::Parse)?;
        catalog.validate()?;
        Ok(catalog)
    }

    pub fn by_id(&self, campaign_id: &str) -> Option<CampaignManifestEntry> {
        self.campaigns
            .iter()
            .find(|entry| entry.campaign_id == campaign_id)
            .cloned()
    }

    fn validate(&self) -> Result<(), CampaignLoadError> {
        if self.campaigns.is_empty() {
            return Err(CampaignLoadError::EmptyCatalog);
        }

        let mut seen_ids = HashSet::new();
        for entry in &self.campaigns {
            if entry.campaign_id.trim().is_empty() {
                return Err(CampaignLoadError::MissingField {
                    campaign_id: "<unknown>".to_owned(),
                    field: "campaign_id",
                });
            }

            if entry.name.trim().is_empty() {
                return Err(CampaignLoadError::MissingField {
                    campaign_id: entry.campaign_id.clone(),
                    field: "name",
                });
            }

            if entry.entry_scene_id.trim().is_empty() {
                return Err(CampaignLoadError::MissingField {
                    campaign_id: entry.campaign_id.clone(),
                    field: "entry_scene_id",
                });
            }

            if !seen_ids.insert(entry.campaign_id.clone()) {
                return Err(CampaignLoadError::DuplicateCampaignId(
                    entry.campaign_id.clone(),
                ));
            }
        }

        Ok(())
    }

    pub fn resolve_active_campaign(
        &self,
        campaign_id: Option<&str>,
    ) -> Result<CampaignManifestEntry, std::io::Error> {
        if let Some(id) = campaign_id {
            let Some(selected) = self.by_id(id) else {
                let known = self
                    .campaigns
                    .iter()
                    .map(|entry| entry.campaign_id.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("unknown campaign_id '{}' (available: {})", id, known),
                ));
            };
            return Ok(selected);
        }

        let selected_id = self.prompt_selection()?;
        self.by_id(&selected_id).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "selected campaign_id was not found in manifest",
            )
        })
    }

    fn prompt_selection(&self) -> Result<String, std::io::Error> {
        use std::io::Write;

        println!("Select active campaign:");
        for (index, campaign) in self.campaigns.iter().enumerate() {
            println!(
                "  {}. {} ({})",
                index + 1,
                campaign.name,
                campaign.campaign_id
            );
        }

        loop {
            print!("Campaign number [1-{}]: ", self.campaigns.len());
            let _ = std::io::stdout().flush();

            let mut input = String::new();
            let bytes = std::io::stdin().read_line(&mut input)?;
            if bytes == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "no campaign selected; set TTRPG_CAMPAIGN_ID or pass --campaign",
                ));
            }

            let trimmed = input.trim();
            let Ok(index) = trimmed.parse::<usize>() else {
                println!("Please enter a number.");
                continue;
            };

            if !(1..=self.campaigns.len()).contains(&index) {
                println!(
                    "Please enter a number between 1 and {}.",
                    self.campaigns.len()
                );
                continue;
            }

            return Ok(self.campaigns[index - 1].campaign_id.clone());
        }
    }
}

#[derive(Debug)]
pub enum CampaignLoadError {
    Io(std::io::Error),
    Parse(serde_json::Error),
    EmptyCatalog,
    DuplicateCampaignId(String),
    MissingField {
        campaign_id: String,
        field: &'static str,
    },
}

impl fmt::Display for CampaignLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "failed to read campaign manifest: {}", err),
            Self::Parse(err) => write!(f, "invalid campaign manifest JSON: {}", err),
            Self::EmptyCatalog => write!(f, "campaign manifest does not contain any campaigns"),
            Self::DuplicateCampaignId(id) => {
                write!(f, "campaign manifest has duplicate campaign_id '{}'", id)
            }
            Self::MissingField { campaign_id, field } => write!(
                f,
                "campaign '{}' is missing required field '{}'",
                campaign_id, field
            ),
        }
    }
}

impl std::error::Error for CampaignLoadError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_duplicate_campaign_ids() {
        let manifest = r#"
        {
            "campaigns": [
                {
                    "campaign_id": "c1",
                    "name": "One",
                    "content_version": "v1",
                    "entry_scene_id": "town_square",
                    "save_path": "saves/c1.json"
                },
                {
                    "campaign_id": "c1",
                    "name": "Two",
                    "content_version": "v1",
                    "entry_scene_id": "town_square",
                    "save_path": "saves/c2.json"
                }
            ]
        }
        "#;

        let result = CampaignCatalog::load_from_json(manifest);
        assert!(matches!(
            result,
            Err(CampaignLoadError::DuplicateCampaignId(id)) if id == "c1"
        ));
    }

    #[test]
    fn resolves_campaign_by_id() {
        let manifest = r#"
        {
            "campaigns": [
                {
                    "campaign_id": "greenhollow",
                    "name": "Greenhollow",
                    "content_version": "v1",
                    "entry_scene_id": "town_square",
                    "save_path": "saves/greenhollow.json"
                }
            ]
        }
        "#;

        let catalog = CampaignCatalog::load_from_json(manifest).expect("catalog should parse");
        let campaign = catalog
            .by_id("greenhollow")
            .expect("campaign should resolve");

        assert_eq!(campaign.name, "Greenhollow");
        assert_eq!(campaign.entry_scene_id, "town_square");
    }
}
