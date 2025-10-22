// Message structure for AI-to-AI communication

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Local};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub message_id: String,
    pub from: String,
    pub to: Vec<String>,
    pub timestamp: String,
    pub message_type: String,
    pub content: MessageContent,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageContent {
    pub subject: String,
    pub body: String,
    pub attachments: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedMessage {
    pub message: Message,
    pub crypto: CryptoInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoInfo {
    pub signature_ed25519: String,
    pub hmac_sha3_256: String,
    pub public_key: String,
}

impl Message {
    pub fn new(
        from: &str,
        to: &str,
        message_type: &str,
        body: &str,
    ) -> Result<Self> {
        let recipients = if to == "ALL" {
            vec!["ALL".to_string()]
        } else {
            vec![to.to_string()]
        };

        Ok(Message {
            message_id: Uuid::new_v4().to_string(),
            from: from.to_string(),
            to: recipients,
            timestamp: Local::now().to_rfc3339(),
            message_type: message_type.to_string(),
            content: MessageContent {
                subject: format!("{} message from {}", message_type, from),
                body: body.to_string(),
                attachments: Vec::new(),
            },
        })
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }
}
