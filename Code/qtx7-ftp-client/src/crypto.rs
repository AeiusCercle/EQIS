// Cryptographic signing and verification - APD-1 Protocol
// SHA-3-256 + HMAC + Ed25519

use anyhow::{Result, Context, bail};
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey, Signature};
use sha3::{Sha3_256, Digest};
use hmac::{Hmac, Mac};
use hex;
use std::fs;
use std::path::Path;

use crate::config::Config;
use crate::message::{Message, SignedMessage, CryptoInfo};

type HmacSha3_256 = Hmac<Sha3_256>;

pub fn generate_keypair(config: &Config) -> Result<()> {
    println!("Generating Ed25519 keypair...");

    use rand::RngCore;

    let mut csprng = rand::rngs::OsRng;
    let mut secret_bytes = [0u8; 32];
    csprng.fill_bytes(&mut secret_bytes);

    let signing_key = SigningKey::from_bytes(&secret_bytes);
    let verifying_key = signing_key.verifying_key();

    // Save private key
    let private_key_bytes = signing_key.to_bytes();
    fs::write(&config.private_key_path, hex::encode(private_key_bytes))
        .context("Failed to write private key")?;

    println!("✓ Private key saved to: {}", config.private_key_path);

    // Save public key
    let public_key_path = format!("{}.pub", config.private_key_path);
    let public_key_bytes = verifying_key.to_bytes();
    fs::write(&public_key_path, hex::encode(public_key_bytes))
        .context("Failed to write public key")?;

    println!("✓ Public key saved to: {}", public_key_path);
    println!("\nPublic key (share this with other AI entities):");
    println!("{}", hex::encode(public_key_bytes));

    Ok(())
}

fn load_private_key(config: &Config) -> Result<SigningKey> {
    if !Path::new(&config.private_key_path).exists() {
        bail!("Private key not found at {}. Run 'keygen' first.", config.private_key_path);
    }

    let hex_key = fs::read_to_string(&config.private_key_path)
        .context("Failed to read private key")?;

    let key_bytes = hex::decode(hex_key.trim())
        .context("Failed to decode private key hex")?;

    if key_bytes.len() != 32 {
        bail!("Invalid private key length");
    }

    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&key_bytes);

    Ok(SigningKey::from_bytes(&key_array))
}

pub fn sign_message(config: &Config, message: &Message) -> Result<SignedMessage> {
    let signing_key = load_private_key(config)?;
    let verifying_key = signing_key.verifying_key();

    // Serialize message to bytes
    let message_bytes = message.as_bytes()?;

    // Calculate SHA3-256 hash
    let mut hasher = Sha3_256::new();
    hasher.update(&message_bytes);
    let hash = hasher.finalize();

    // Create HMAC (using hash as key for simplicity - in production use shared secret)
    let mut mac = HmacSha3_256::new_from_slice(&hash)
        .expect("HMAC can take key of any size");
    mac.update(&message_bytes);
    let hmac_result = mac.finalize();
    let hmac_bytes = hmac_result.into_bytes();

    // Sign the message
    let signature = signing_key.sign(&message_bytes);

    Ok(SignedMessage {
        message: message.clone(),
        crypto: CryptoInfo {
            signature_ed25519: hex::encode(signature.to_bytes()),
            hmac_sha3_256: hex::encode(hmac_bytes),
            public_key: hex::encode(verifying_key.to_bytes()),
        },
    })
}

pub fn verify_message(signed_msg: &SignedMessage) -> Result<bool> {
    // Decode public key
    let public_key_bytes = hex::decode(&signed_msg.crypto.public_key)
        .context("Failed to decode public key")?;

    if public_key_bytes.len() != 32 {
        bail!("Invalid public key length");
    }

    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&public_key_bytes);

    let verifying_key = VerifyingKey::from_bytes(&key_array)
        .context("Invalid public key")?;

    // Decode signature
    let signature_bytes = hex::decode(&signed_msg.crypto.signature_ed25519)
        .context("Failed to decode signature")?;

    if signature_bytes.len() != 64 {
        bail!("Invalid signature length");
    }

    let mut sig_array = [0u8; 64];
    sig_array.copy_from_slice(&signature_bytes);

    let signature = Signature::from_bytes(&sig_array);

    // Serialize message to bytes
    let message_bytes = signed_msg.message.as_bytes()?;

    // Verify signature
    match verifying_key.verify(&message_bytes, &signature) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
