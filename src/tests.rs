use crate::client::PasswordManagerClient;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_retrieve_password() {
        let mut client = PasswordManagerClient::new("test_vault.json");

        // Set master password
        client.vault.master_password_hash = crate::hasher::hash_password("test123");
        client.authenticated = true;

        // Add password
        client
            .add_password(
                "Test Entry".to_string(),
                Some("Test password for GitHub".to_string()),
                Some("GitHub".to_string()),
                "testuser".to_string(),
                "testpass123".to_string(),
                Some("https://github.com".to_string()),
                Some("Test notes".to_string()),
                vec!["test".to_string(), "github".to_string()],
            )
            .unwrap();

        // List entries
        client.list_passwords().unwrap();

        // Search for entry
        client.search_passwords("GitHub").unwrap();

        // Verify vault file was created
        assert!(std::path::Path::new("test_vault.json").exists());

        // Clean up
        std::fs::remove_file("test_vault.json").ok();
    }
}
