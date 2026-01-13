#[cfg(test)]
mod tests;

mod cli;
mod client;
mod file_mng;
mod hasher;
mod password_generator;
mod password_manager;
mod scanner;

use client::PasswordManagerClient;

fn main() -> anyhow::Result<()> {
    let vault_path = "password_vault.json";
    let mut client = PasswordManagerClient::new(vault_path);

    println!("=== Password Manager Client ===");
    client.run_interactive()?;

    Ok(())
}
