use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub to: String,
    pub value: String,
    pub data: String,
    pub nonce: u64,
    pub gas_limit: String,
    pub gas_price: String,
    pub created_at: DateTime<Utc>,
    pub status: TransactionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionStatus {
    Pending,
    Collecting,
    Ready,
    Executed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub signer_address: String,
    pub signature: String,
    pub signer_type: SignerType,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignerType {
    Human,
    AiCfo,
    AiSecurity,
    AiAnalyst,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeConfig {
    pub safe_address: String,
    pub required_signatures: u8,
    pub total_signers: u8,
    pub human_signers: Vec<String>,
    pub ai_signers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalMetadata {
    pub proposer: String,
    pub description: Option<String>,
    pub deadline: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
}