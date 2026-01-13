# SecTool

A secure password manager written in Rust.

## Overview

SecTool is a comprehensive password management application built with Rust, designed to securely store, generate, and manage passwords. The application provides an interactive command-line interface for managing password entries with robust security features.

## Features

- **Secure Storage**: Passwords are stored in an encrypted vault with master password protection
- **Password Generation**: Generate strong, random passwords with customizable parameters
- **Entry Management**: Add, edit, delete, and search password entries
- **Rich Metadata**: Store usernames, URLs, notes, tags, and timestamps with each entry
- **Search Functionality**: Quickly find entries by title, description, username, or tags
- **Interactive CLI**: User-friendly command-line interface with intuitive menus

## Installation

1. Clone the repository
2. Install Rust (if not already installed)
3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

Run the application:
```bash
cargo run
```

The application will start an interactive session where you can:
- Set up a master password
- Add new password entries
- Generate secure passwords
- Search and manage existing entries
- Export/import password data

## Security Features

- SHA-512 hashing for master password
- Local storage of encrypted password vault
- Secure password generation using cryptographically secure random number generation
- Memory-safe operations using Rust's ownership system

## Dependencies

- `serde` - JSON serialization/deserialization
- `sha2` - Cryptographic hashing
- `rand` - Secure random number generation
- `rpassword` - Secure password input
- `dialoguer` - Interactive CLI prompts
- `clap` - Command line argument parsing

## Project Structure

```
src/
├── main.rs              # Application entry point
├── password_manager.rs  # Core password vault logic
├── password_generator.rs # Password generation utilities
├── hasher.rs           # Cryptographic hashing
├── file_mng.rs         # File management operations
├── client.rs           # Interactive client interface
├── scanner.rs          # Input scanning utilities
└── cli.rs              # Command line interface
```

## License

This project is open source and available under the MIT License.