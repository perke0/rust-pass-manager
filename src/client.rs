use crate::password_manager::{PasswordEntry, PasswordVault};
use anyhow::Result;
use chrono::Utc;
use dialoguer::{Confirm, Input, Password, Select};
use uuid::Uuid;

pub struct PasswordManagerClient {
    pub vault: PasswordVault,
    vault_path: String,
    pub authenticated: bool,
}

impl PasswordManagerClient {
    pub fn new(vault_path: &str) -> Self {
        Self {
            vault: PasswordVault::load_from_file(std::path::Path::new(vault_path))
                .unwrap_or_else(|_| PasswordVault::new()),
            vault_path: vault_path.to_string(),
            authenticated: false,
        }
    }

    pub fn authenticate(&mut self) -> Result<bool> {
        if self.vault.master_password_hash.is_empty() {
            println!("No master password set. Let's create one!");
            let master_password = Password::new()
                .with_prompt("Set master password")
                .with_confirmation("Confirm master password", "Passwords do not match")
                .interact()?;

            self.vault.master_password_hash = crate::hasher::hash_password(&master_password);
            self.save_vault()?;
            self.authenticated = true;
            return Ok(true);
        }

        let master_password = Password::new()
            .with_prompt("Enter master password")
            .interact()?;

        self.authenticated =
            crate::hasher::verify_password(&master_password, &self.vault.master_password_hash);
        Ok(self.authenticated)
    }

    pub fn run_interactive(&mut self) -> Result<()> {
        if !self.authenticate()? {
            println!("Authentication failed!");
            return Ok(());
        }

        loop {
            let choices = vec![
                "Add new password",
                "List all passwords",
                "Search passwords",
                "Get password by ID",
                "Update password",
                "Delete password",
                "Generate password",
                "Exit",
            ];

            let selection = Select::new()
                .with_prompt("What would you like to do?")
                .items(&choices)
                .interact()?;

            match selection {
                0 => self.add_password_interactive()?,
                1 => self.list_passwords()?,
                2 => self.search_passwords_interactive()?,
                3 => self.get_password_interactive()?,
                4 => self.update_password_interactive()?,
                5 => self.delete_password_interactive()?,
                6 => self.generate_password_interactive()?,
                7 => {
                    println!("Goodbye!");
                    break;
                }
                _ => unreachable!(),
            }
        }

        Ok(())
    }

    fn add_password_interactive(&mut self) -> Result<()> {
        let title = Input::<String>::new()
            .with_prompt("Title/Name")
            .interact()?;

        let description = Input::<String>::new()
            .with_prompt("Description (what is this password for?)")
            .allow_empty(true)
            .interact()?;
        let description = if description.is_empty() {
            None
        } else {
            Some(description)
        };

        let site_name = Input::<String>::new()
            .with_prompt("Site/Service name (e.g., Google, GitHub, etc.)")
            .allow_empty(true)
            .interact()?;
        let site_name = if site_name.is_empty() {
            None
        } else {
            Some(site_name)
        };

        let username = Input::<String>::new()
            .with_prompt("Username/Email")
            .interact()?;

        let password = Password::new().with_prompt("Password").interact()?;

        let url = Input::<String>::new()
            .with_prompt("URL (optional)")
            .allow_empty(true)
            .interact()?;
        let url = if url.is_empty() { None } else { Some(url) };

        let notes = Input::<String>::new()
            .with_prompt("Notes (optional)")
            .allow_empty(true)
            .interact()?;
        let notes = if notes.is_empty() { None } else { Some(notes) };

        let tags_input = Input::<String>::new()
            .with_prompt("Tags (comma-separated, optional)")
            .allow_empty(true)
            .interact()?;
        let tags = if tags_input.is_empty() {
            Vec::new()
        } else {
            tags_input
                .split(',')
                .map(|s| s.trim().to_string())
                .collect()
        };

        self.add_password(
            title,
            description,
            site_name,
            username,
            password,
            url,
            notes,
            tags,
        )
    }

    pub fn add_password(
        &mut self,
        title: String,
        description: Option<String>,
        site_name: Option<String>,
        username: String,
        password: String,
        url: Option<String>,
        notes: Option<String>,
        tags: Vec<String>,
    ) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        let entry = PasswordEntry {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            site_name,
            username,
            password,
            url,
            notes,
            created_at: now.clone(),
            updated_at: now,
            tags,
        };

        self.vault.add_entry(entry);
        self.save_vault()?;
        println!("Password added successfully!");
        Ok(())
    }

    pub fn list_passwords(&self) -> Result<()> {
        let entries = self.vault.list_entries();
        if entries.is_empty() {
            println!("No passwords stored yet.");
            return Ok(());
        }

        println!("\nStored Passwords:");
        println!(
            "{:<40} {:<25} {:<20} {:<30} {:<50}",
            "Title", "Site", "Username", "Description", "URL"
        );
        println!("{}", "-".repeat(165));

        for entry in entries {
            let default_url = "-".to_string();
            let default_site = "-".to_string();
            let default_desc = "-".to_string();
            let url_str = entry.url.as_ref().unwrap_or(&default_url);
            let site_str = entry.site_name.as_ref().unwrap_or(&default_site);
            let desc_str = entry.description.as_ref().unwrap_or(&default_desc);

            println!(
                "{:<40} {:<25} {:<20} {:<30} {:<50}",
                &entry.title[..entry.title.len().min(40)],
                &site_str[..site_str.len().min(25)],
                &entry.username[..entry.username.len().min(20)],
                &desc_str[..desc_str.len().min(30)],
                &url_str[..url_str.len().min(50)]
            );
        }
        println!();
        Ok(())
    }

    fn search_passwords_interactive(&self) -> Result<()> {
        let query = Input::<String>::new()
            .with_prompt("Search query")
            .interact()?;

        self.search_passwords(&query)
    }

    pub fn search_passwords(&self, query: &str) -> Result<()> {
        let results = self.vault.search_entries(query);
        if results.is_empty() {
            println!("No passwords found matching '{}'", query);
            return Ok(());
        }

        println!("\nSearch Results for '{}':", query);
        println!(
            "{:<40} {:<25} {:<20} {:<30} {:<50}",
            "Title", "Site", "Username", "Description", "URL"
        );
        println!("{}", "-".repeat(165));

        for entry in results {
            let default_url = "-".to_string();
            let default_site = "-".to_string();
            let default_desc = "-".to_string();
            let url_str = entry.url.as_ref().unwrap_or(&default_url);
            let site_str = entry.site_name.as_ref().unwrap_or(&default_site);
            let desc_str = entry.description.as_ref().unwrap_or(&default_desc);

            println!(
                "{:<40} {:<25} {:<20} {:<30} {:<50}",
                &entry.title[..entry.title.len().min(40)],
                &site_str[..site_str.len().min(25)],
                &entry.username[..entry.username.len().min(20)],
                &desc_str[..desc_str.len().min(30)],
                &url_str[..url_str.len().min(50)]
            );
        }
        println!();
        Ok(())
    }

    fn get_password_interactive(&self) -> Result<()> {
        let id = Input::<String>::new()
            .with_prompt("Enter password ID")
            .interact()?;

        if let Some(entry) = self.vault.get_entry(&id) {
            println!("\nPassword Entry:");
            println!("ID: {}", entry.id);
            println!("Title: {}", entry.title);
            if let Some(description) = &entry.description {
                println!("Description: {}", description);
            }
            if let Some(site_name) = &entry.site_name {
                println!("Site/Service: {}", site_name);
            }
            println!("Username: {}", entry.username);
            println!("Password: {}", entry.password);
            if let Some(url) = &entry.url {
                println!("URL: {}", url);
            }
            if let Some(notes) = &entry.notes {
                println!("Notes: {}", notes);
            }
            if !entry.tags.is_empty() {
                println!("Tags: {}", entry.tags.join(", "));
            }
            println!("Created: {}", entry.created_at);
            println!("Updated: {}", entry.updated_at);
        } else {
            println!("Password not found!");
        }
        Ok(())
    }

    fn update_password_interactive(&mut self) -> Result<()> {
        let id = Input::<String>::new()
            .with_prompt("Enter password ID to update")
            .interact()?;

        if let Some(entry) = self.vault.get_entry(&id) {
            println!("Current entry: {}", entry.title);

            let new_title = Input::<String>::new()
                .with_prompt(&format!("Title [{}]", entry.title))
                .default(entry.title.clone())
                .interact()?;

            let new_description = Input::<String>::new()
                .with_prompt(&format!(
                    "Description [{}]",
                    entry.description.as_ref().unwrap_or(&"-".to_string())
                ))
                .allow_empty(true)
                .interact()?;
            let new_description = if new_description.is_empty() {
                entry.description.clone()
            } else {
                Some(new_description)
            };

            let new_site_name = Input::<String>::new()
                .with_prompt(&format!(
                    "Site/Service [{}]",
                    entry.site_name.as_ref().unwrap_or(&"-".to_string())
                ))
                .allow_empty(true)
                .interact()?;
            let new_site_name = if new_site_name.is_empty() {
                entry.site_name.clone()
            } else {
                Some(new_site_name)
            };

            let new_username = Input::<String>::new()
                .with_prompt(&format!("Username [{}]", entry.username))
                .default(entry.username.clone())
                .interact()?;

            let new_password = Password::new()
                .with_prompt("Password (leave empty to keep current)")
                .allow_empty_password(true)
                .interact()?;

            let new_password = if new_password.is_empty() {
                entry.password.clone()
            } else {
                new_password
            };

            let mut updated_entry = entry.clone();
            updated_entry.title = new_title;
            updated_entry.description = new_description;
            updated_entry.site_name = new_site_name;
            updated_entry.username = new_username;
            updated_entry.password = new_password;
            updated_entry.updated_at = Utc::now().to_rfc3339();

            self.vault.add_entry(updated_entry);
            self.save_vault()?;
            println!("Password updated successfully!");
        } else {
            println!("Password not found!");
        }
        Ok(())
    }

    fn delete_password_interactive(&mut self) -> Result<()> {
        let id = Input::<String>::new()
            .with_prompt("Enter password ID to delete")
            .interact()?;

        if let Some(entry) = self.vault.get_entry(&id) {
            println!("Entry to delete: {}", entry.title);
            let confirm = Confirm::new()
                .with_prompt("Are you sure you want to delete this password?")
                .default(false)
                .interact()?;

            if confirm {
                self.vault.remove_entry(&id);
                self.save_vault()?;
                println!("Password deleted successfully!");
            } else {
                println!("Deletion cancelled.");
            }
        } else {
            println!("Password not found!");
        }
        Ok(())
    }

    fn generate_password_interactive(&self) -> Result<()> {
        let length: usize = Input::<String>::new()
            .with_prompt("Password length (default: 16)")
            .default("16".to_string())
            .interact()?
            .parse()
            .unwrap_or(16);

        let include_symbols = Confirm::new()
            .with_prompt("Include symbols?")
            .default(true)
            .interact()?;

        let password = crate::password_generator::generate_password(length, include_symbols);
        println!("Generated password: {}", password);
        Ok(())
    }

    fn save_vault(&self) -> Result<()> {
        self.vault
            .save_to_file(std::path::Path::new(&self.vault_path))
    }
}
