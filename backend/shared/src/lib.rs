use thiserror::Error;

pub mod constants;
pub mod types;
pub mod utils;

#[derive(Error, Debug)]
pub enum SafeWalletError {
    #[error("Transaction validation failed: {0}")]
    ValidationError(String),

    #[error("Insufficient signatures: got {got}, need {need}")]
    InsufficientSignatures { got: usize, need: usize },

    #[error("Database error: {0}")]
    DatabaseError(#[from] anyhow::Error),

    #[error("Blockchain error: {0}")]
    BlockchainError(String),

    #[error("AI Agent error: {0}")]
    AgentError(String),
}

pub type Result<T> = std::result::Result<T, SafeWalletError>;
