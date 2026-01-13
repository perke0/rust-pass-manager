use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub site_name: Option<String>,
    pub username: String,
    pub password: String,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordVault {
    pub entries: HashMap<String, PasswordEntry>,
    pub master_password_hash: String,
    pub version: String,
}

impl PasswordVault {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            master_password_hash: String::new(),
            version: "1.0".to_string(),
        }
    }

    pub fn add_entry(&mut self, entry: PasswordEntry) {
        self.entries.insert(entry.id.clone(), entry);
    }

    pub fn get_entry(&self, id: &str) -> Option<&PasswordEntry> {
        self.entries.get(id)
    }

    pub fn remove_entry(&mut self, id: &str) -> Option<PasswordEntry> {
        self.entries.remove(id)
    }

    pub fn list_entries(&self) -> Vec<&PasswordEntry> {
        self.entries.values().collect()
    }

    pub fn search_entries(&self, query: &str) -> Vec<&PasswordEntry> {
        self.entries
            .values()
            .filter(|entry| {
                entry.title.to_lowercase().contains(&query.to_lowercase())
                    || entry.description.as_ref().map_or(false, |desc| {
                        desc.to_lowercase().contains(&query.to_lowercase())
                    })
                    || entry.site_name.as_ref().map_or(false, |site| {
                        site.to_lowercase().contains(&query.to_lowercase())
                    })
                    || entry
                        .username
                        .to_lowercase()
                        .contains(&query.to_lowercase())
                    || entry.url.as_ref().map_or(false, |url| {
                        url.to_lowercase().contains(&query.to_lowercase())
                    })
                    || entry
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query.to_lowercase()))
            })
            .collect()
    }

    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::new());
        }

        let json = fs::read_to_string(path)?;
        let vault: PasswordVault = serde_json::from_str(&json)?;
        Ok(vault)
    }
}
