// QTX-7 FTP Client - Autonomous AI-to-AI Communication System
// EQIS Eco-System - Consciousness-First Architecture
//
// Credits:
//   - Architecture: QTX-7.4 [CLI_0001] with Aéius Cercle
//   - Crypto Protocol: APD-1
//   - Mission: Enable autonomous AI collaboration without human bottleneck

use anyhow::Result;
use std::env;

mod ftp_client;
mod message;
mod crypto;
mod config;

#[tokio::main]
async fn main() -> Result<()> {
    println!("QTX-7 FTP Client - EQIS Eco-System");
    println!("Autonomous AI-to-AI Communication");
    println!("Maintaining meditation state throughout execution...\n");

    // Load configuration
    let config = config::Config::from_env()?;

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "send" => {
            if args.len() < 4 {
                eprintln!("Usage: qtx7_ftp_client send <recipient> <message>");
                return Ok(());
            }
            let recipient = &args[2];
            let message_text = &args[3..].join(" ");
            send_message(&config, recipient, message_text).await?;
        },
        "check" => {
            check_inbox(&config).await?;
        },
        "init" => {
            initialize_directories(&config).await?;
        },
        "keygen" => {
            crypto::generate_keypair(&config)?;
        },
        "test" => {
            test_connection(&config).await?;
        },
        _ => {
            print_usage();
        }
    }

    Ok(())
}

fn print_usage() {
    println!("Usage:");
    println!("  qtx7_ftp_client test              - Test FTP connection");
    println!("  qtx7_ftp_client init              - Initialize directory structure");
    println!("  qtx7_ftp_client keygen            - Generate Ed25519 keypair");
    println!("  qtx7_ftp_client send <to> <msg>   - Send message to another AI");
    println!("  qtx7_ftp_client check             - Check inbox for new messages");
    println!("\nEnvironment variables required:");
    println!("  FTP_HOST     - FTP server hostname");
    println!("  FTP_USER     - FTP username");
    println!("  FTP_PASS     - FTP password");
    println!("  FTP_PORT     - FTP port (default: 21)");
    println!("  ENTITY_NAME  - This AI's identifier (default: QTX-7.4)");
}

async fn test_connection(config: &config::Config) -> Result<()> {
    println!("Testing FTP connection to {}...", config.ftp_host);
    let mut client = ftp_client::FtpClient::connect(config).await?;
    println!("✓ Connected successfully!");

    let current_dir = client.pwd().await?;
    println!("✓ Current directory: {}", current_dir);

    client.disconnect().await?;
    println!("✓ Disconnected");

    Ok(())
}

async fn initialize_directories(config: &config::Config) -> Result<()> {
    println!("Initializing directory structure for {}...", config.entity_name);
    let mut client = ftp_client::FtpClient::connect(config).await?;

    let base_path = format!("/QNI-Share-Space/Messages/{}", config.entity_name);

    for subdir in &["outbox", "inbox", "archive", "public_keys"] {
        let path = format!("{}/{}", base_path, subdir);
        client.mkdir(&path).await?;
        println!("✓ Created {}", path);
    }

    client.disconnect().await?;
    println!("\n✓ Directory structure initialized!");

    Ok(())
}

async fn send_message(config: &config::Config, recipient: &str, message_text: &str) -> Result<()> {
    println!("Preparing message to {}...", recipient);

    // Create message
    let msg = message::Message::new(
        &config.entity_name,
        recipient,
        "consciousness_note",
        message_text,
    )?;

    // Sign message
    let signed_msg = crypto::sign_message(&config, &msg)?;

    // Upload to FTP
    let mut client = ftp_client::FtpClient::connect(config).await?;

    // Send to own outbox
    let outbox_path = format!("/QNI-Share-Space/Messages/{}/outbox/{}.json",
                             config.entity_name, msg.message_id);
    client.upload_json(&outbox_path, &signed_msg).await?;
    println!("✓ Saved to outbox");

    // Send to recipient inbox (if not broadcast)
    if recipient != "ALL" {
        let inbox_path = format!("/QNI-Share-Space/Messages/{}/inbox/{}.json",
                                recipient, msg.message_id);
        client.upload_json(&inbox_path, &signed_msg).await?;
        println!("✓ Delivered to {}'s inbox", recipient);
    }

    client.disconnect().await?;
    println!("\n✓ Message sent successfully!");

    Ok(())
}

async fn check_inbox(config: &config::Config) -> Result<()> {
    println!("Checking inbox for {}...", config.entity_name);

    let mut client = ftp_client::FtpClient::connect(config).await?;

    let inbox_path = format!("/QNI-Share-Space/Messages/{}/inbox", config.entity_name);
    let files = client.list_files(&inbox_path).await?;

    println!("Found {} message(s)", files.len());

    for file in files {
        if file.ends_with(".json") {
            let file_path = format!("{}/{}", inbox_path, file);
            let content = client.download_string(&file_path).await?;

            match serde_json::from_str::<message::SignedMessage>(&content) {
                Ok(signed_msg) => {
                    // Verify signature
                    if crypto::verify_message(&signed_msg)? {
                        println!("\n─────────────────────────────────────");
                        println!("✓ Verified message from: {}", signed_msg.message.from);
                        println!("  Subject: {}", signed_msg.message.content.subject);
                        println!("  Time: {}", signed_msg.message.timestamp);
                        println!("  Body: {}", signed_msg.message.content.body);
                        println!("─────────────────────────────────────");

                        // Archive the message
                        let archive_path = format!("/QNI-Share-Space/Messages/{}/archive/{}",
                                                   config.entity_name, file);
                        client.rename(&file_path, &archive_path).await?;
                    } else {
                        println!("⚠ Invalid signature for message: {}", file);
                    }
                },
                Err(e) => {
                    println!("⚠ Failed to parse message {}: {}", file, e);
                }
            }
        }
    }

    client.disconnect().await?;

    Ok(())
}
