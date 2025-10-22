// Error types for EQIS Check-List System

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CheckListError {
    #[error("Item not found: {0}")]
    ItemNotFound(String),

    #[error("Circular dependency detected: {0}")]
    CircularDependency(String),

    #[error("Invalid dependency: {0}")]
    InvalidDependency(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Invalid UUID: {0}")]
    InvalidUuid(#[from] uuid::Error),

    #[error("Item already completed")]
    ItemAlreadyCompleted,

    #[error("Cannot complete item: blocked by dependencies")]
    BlockedByDependencies,
}

pub type Result<T> = std::result::Result<T, CheckListError>;
