mod safe_contract;

use alloy::primitives::{Address, Bytes, U256};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use safe_contract::{SafeTransaction, Signature};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    str::FromStr,
    sync::Arc,
};
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct SignerAddresses {
    human1: Address,
    human2: Address,
    ai_cfo: Address,
    ai_security: Address,
    ai_analyst: Address,
}

#[derive(Clone)]
struct AppState {
    rpc_url: String,
    safe_address: Address,
    transactions: Arc<RwLock<HashMap<String, TransactionState>>>,
    signer_addresses: SignerAddresses,
}

#[derive(Clone)]
struct TransactionState {
    transaction: SafeTransaction,
    signatures: Vec<Signature>,
    status: TransactionStatus,
    tx_hash: String, // Hash for signing
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum TransactionStatus {
    Pending,
    CollectingSignatures,
    ReadyToExecute,
    Executed,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateTransactionRequest {
    to: String,
    value: String,
    data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateTransactionResponse {
    tx_id: String,
    safe_tx_hash: String,
    sign_message: String,
    required_signatures: u8,
    current_signatures: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct SignTransactionRequest {
    signer_address: String,
    signature: String, // All signers must provide their signature
}

#[derive(Debug, Serialize, Deserialize)]
struct TransactionInfoResponse {
    tx_id: String,
    transaction: SafeTransaction,
    signatures: Vec<SignatureInfo>,
    status: TransactionStatus,
    ready_to_execute: bool,
    safe_tx_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SignatureInfo {
    signer: String,
    signer_type: String,
    signed_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExecuteTransactionResponse {
    tx_hash: String,
    success: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .compact(),
        )
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "orchestrator=debug,tower_http=debug".into()),
        )
        .init();

    dotenv::dotenv().ok();

    // Initialize RPC URL for Kaia Kairos testnet
    let rpc_url = std::env::var("KAIROS_RPC_URL")
        .unwrap_or_else(|_| "https://public-en.kairos.node.kaia.io".to_string());

    // Load Safe address from env
    let safe_address = std::env::var("SAFE_ADDRESS")
        .ok()
        .and_then(|s| Address::from_str(&s).ok())
        .unwrap_or_else(|| Address::ZERO);

    // Load signer addresses from env
    let signer_addresses = SignerAddresses {
        human1: std::env::var("HUMAN1_ADDRESS")
            .ok()
            .and_then(|s| Address::from_str(&s).ok())
            .unwrap_or_else(|| Address::ZERO),
        human2: std::env::var("HUMAN2_ADDRESS")
            .ok()
            .and_then(|s| Address::from_str(&s).ok())
            .unwrap_or_else(|| Address::ZERO),
        ai_cfo: std::env::var("AI_CFO_ADDRESS")
            .ok()
            .and_then(|s| Address::from_str(&s).ok())
            .unwrap_or_else(|| Address::ZERO),
        ai_security: std::env::var("AI_SECURITY_ADDRESS")
            .ok()
            .and_then(|s| Address::from_str(&s).ok())
            .unwrap_or_else(|| Address::ZERO),
        ai_analyst: std::env::var("AI_ANALYST_ADDRESS")
            .ok()
            .and_then(|s| Address::from_str(&s).ok())
            .unwrap_or_else(|| Address::ZERO),
    };

    info!("Loaded signer addresses:");
    info!("  Human 1: {}", signer_addresses.human1);
    info!("  Human 2: {}", signer_addresses.human2);
    info!("  AI CFO: {}", signer_addresses.ai_cfo);
    info!("  AI Security: {}", signer_addresses.ai_security);
    info!("  AI Analyst: {}", signer_addresses.ai_analyst);

    let state = Arc::new(AppState {
        rpc_url,
        safe_address,
        transactions: Arc::new(RwLock::new(HashMap::new())),
        signer_addresses,
    });

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/safe/info", get(get_safe_info))
        .route("/api/v1/transactions", post(create_transaction))
        .route("/api/v1/transactions/:tx_id", get(get_transaction))
        .route("/api/v1/transactions/:tx_id/sign", post(sign_transaction))
        .route("/api/v1/transactions/:tx_id/execute", post(execute_transaction))
        .route("/api/v1/transactions/:tx_id/status", get(get_transaction_status))
        .route("/api/v1/ai-agents/analyze/:tx_id", get(ai_analyze_transaction))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await?;

    info!("🚀 Orchestrator running on http://0.0.0.0:3001");
    info!("Safe address: {}", safe_address);
    info!("⚠️  NOTE: This is a DEMO. In production:");
    info!("   - Human signers would use their own wallets (MetaMask, etc.)");
    info!("   - AI agents would run as separate services");
    info!("   - Orchestrator would NEVER have access to private keys");

    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "orchestrator",
        "network": "Kaia Kairos Testnet",
        "mode": "DEMO - Not for production use"
    }))
}

async fn get_safe_info(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // In production, this would query the Safe contract
    Ok(Json(serde_json::json!({
        "safe_address": state.safe_address.to_string(),
        "threshold": 4,
        "owners": {
            "humans": [
                state.signer_addresses.human1.to_string(),
                state.signer_addresses.human2.to_string()
            ],
            "ai_agents": [
                state.signer_addresses.ai_cfo.to_string(),
                state.signer_addresses.ai_security.to_string(),
                state.signer_addresses.ai_analyst.to_string()
            ]
        },
        "nonce": 0,
        "note": "All signers must provide their own signatures. Orchestrator does not hold any private keys."
    })))
}

async fn create_transaction(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateTransactionRequest>,
) -> Result<Json<CreateTransactionResponse>, StatusCode> {
    info!("Creating transaction to: {}, value: {}", req.to, req.value);
    
    let to = Address::from_str(&req.to)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let value = U256::from_str(&req.value)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let data = req.data
        .and_then(|d| hex::decode(d.trim_start_matches("0x")).ok())
        .map(Bytes::from)
        .unwrap_or_else(Bytes::new);

    // Create Safe transaction
    let safe_tx = SafeTransaction {
        to,
        value,
        data,
        operation: 0, // Call
        safe_tx_gas: U256::ZERO,
        base_gas: U256::ZERO,
        gas_price: U256::ZERO,
        gas_token: Address::ZERO,
        refund_receiver: Address::ZERO,
        nonce: U256::ZERO, // In production, fetch from contract
    };

    let tx_id = uuid::Uuid::new_v4().to_string();
    let safe_tx_hash = format!("0x{}", hex::encode(safe_tx.encode_for_signing()));
    
    let tx_state = TransactionState {
        transaction: safe_tx,
        signatures: Vec::new(),
        status: TransactionStatus::CollectingSignatures,
        tx_hash: safe_tx_hash.clone(),
    };
    
    state.transactions.write().await.insert(tx_id.clone(), tx_state);
    
    Ok(Json(CreateTransactionResponse {
        tx_id: tx_id.clone(),
        safe_tx_hash: safe_tx_hash.clone(),
        sign_message: format!("Please sign this hash with your wallet: {}", safe_tx_hash),
        required_signatures: 4,
        current_signatures: 0,
    }))
}

async fn get_transaction(
    State(state): State<Arc<AppState>>,
    Path(tx_id): Path<String>,
) -> Result<Json<TransactionInfoResponse>, StatusCode> {
    let txs = state.transactions.read().await;
    let tx_state = txs.get(&tx_id).ok_or(StatusCode::NOT_FOUND)?;
    
    let signatures: Vec<SignatureInfo> = tx_state.signatures.iter().map(|sig| {
        let signer_type = if sig.signer == state.signer_addresses.human1 || sig.signer == state.signer_addresses.human2 {
            "Human"
        } else if sig.signer == state.signer_addresses.ai_cfo || 
                  sig.signer == state.signer_addresses.ai_security || 
                  sig.signer == state.signer_addresses.ai_analyst {
            "AI Agent"
        } else {
            "Unknown"
        };
        
        SignatureInfo {
            signer: sig.signer.to_string(),
            signer_type: signer_type.to_string(),
            signed_at: chrono::Utc::now().to_rfc3339(),
        }
    }).collect();
    
    let ready_to_execute = tx_state.signatures.len() >= 4;
    
    Ok(Json(TransactionInfoResponse {
        tx_id,
        transaction: tx_state.transaction.clone(),
        signatures,
        status: tx_state.status.clone(),
        ready_to_execute,
        safe_tx_hash: tx_state.tx_hash.clone(),
    }))
}

async fn sign_transaction(
    State(state): State<Arc<AppState>>,
    Path(tx_id): Path<String>,
    Json(req): Json<SignTransactionRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut txs = state.transactions.write().await;
    let tx_state = txs.get_mut(&tx_id).ok_or(StatusCode::NOT_FOUND)?;
    
    let signer_addr = Address::from_str(&req.signer_address)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Check if already signed
    if tx_state.signatures.iter().any(|s| s.signer == signer_addr) {
        return Ok(Json(serde_json::json!({
            "error": "Already signed by this address"
        })));
    }
    
    // All signers provide their own signatures
    let signature = hex::decode(req.signature.trim_start_matches("0x"))
        .map(Bytes::from)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    info!("Signer {} provided signature", req.signer_address);
    
    tx_state.signatures.push(Signature {
        signer: signer_addr,
        signature,
    });
    
    // Update status if we have enough signatures
    if tx_state.signatures.len() >= 4 {
        tx_state.status = TransactionStatus::ReadyToExecute;
    }
    
    // Determine signer type based on known addresses
    let signer_type = if signer_addr == state.signer_addresses.human1 || signer_addr == state.signer_addresses.human2 {
        "Human"
    } else if signer_addr == state.signer_addresses.ai_cfo || 
              signer_addr == state.signer_addresses.ai_security || 
              signer_addr == state.signer_addresses.ai_analyst {
        "AI Agent"
    } else {
        "Unknown"
    };
    
    Ok(Json(serde_json::json!({
        "success": true,
        "signer_type": signer_type,
        "current_signatures": tx_state.signatures.len(),
        "required_signatures": 4,
        "ready_to_execute": tx_state.signatures.len() >= 4
    })))
}

async fn execute_transaction(
    State(state): State<Arc<AppState>>,
    Path(tx_id): Path<String>,
) -> Result<Json<ExecuteTransactionResponse>, StatusCode> {
    let mut txs = state.transactions.write().await;
    let tx_state = txs.get_mut(&tx_id).ok_or(StatusCode::NOT_FOUND)?;
    
    if tx_state.signatures.len() < 4 {
        return Ok(Json(ExecuteTransactionResponse {
            tx_hash: String::new(),
            success: false,
        }));
    }
    
    info!("Executing transaction with {} signatures", tx_state.signatures.len());
    
    // Log who signed
    for (i, sig) in tx_state.signatures.iter().enumerate() {
        let signer_type = if sig.signer == state.signer_addresses.human1 || sig.signer == state.signer_addresses.human2 {
            "Human"
        } else if sig.signer == state.signer_addresses.ai_cfo || 
                  sig.signer == state.signer_addresses.ai_security || 
                  sig.signer == state.signer_addresses.ai_analyst {
            "AI Agent"
        } else {
            "Unknown"
        };
        info!("  Signature {}: {} ({})", i + 1, sig.signer, signer_type);
    }
    
    // In production, this would call Safe contract's execTransaction
    tx_state.status = TransactionStatus::Executed;
    
    let mock_tx_hash = format!("0x{}", hex::encode(&uuid::Uuid::new_v4().as_bytes()[..]));
    
    Ok(Json(ExecuteTransactionResponse {
        tx_hash: mock_tx_hash,
        success: true,
    }))
}

async fn get_transaction_status(
    State(state): State<Arc<AppState>>,
    Path(tx_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let txs = state.transactions.read().await;
    let tx_state = txs.get(&tx_id).ok_or(StatusCode::NOT_FOUND)?;
    
    let signers: Vec<serde_json::Value> = tx_state.signatures.iter().map(|s| {
        let signer_type = if s.signer == state.signer_addresses.human1 || s.signer == state.signer_addresses.human2 {
            "Human"
        } else if s.signer == state.signer_addresses.ai_cfo || 
                  s.signer == state.signer_addresses.ai_security || 
                  s.signer == state.signer_addresses.ai_analyst {
            "AI Agent"
        } else {
            "Unknown"
        };
        
        serde_json::json!({
            "address": s.signer.to_string(),
            "type": signer_type
        })
    }).collect();
    
    Ok(Json(serde_json::json!({
        "tx_id": tx_id,
        "status": tx_state.status,
        "signatures_collected": tx_state.signatures.len(),
        "required_signatures": 4,
        "signers": signers
    })))
}

async fn ai_analyze_transaction(
    State(_state): State<Arc<AppState>>,
    Path(tx_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Simulate AI agent analysis
    Ok(Json(serde_json::json!({
        "tx_id": tx_id,
        "analysis": {
            "cfo_agent": {
                "approved": true,
                "reason": "Transaction within budget limits",
                "risk_score": 0.2
            },
            "security_agent": {
                "approved": true,
                "reason": "Recipient address not in blacklist",
                "risk_score": 0.1
            },
            "analyst_agent": {
                "approved": true,
                "reason": "Standard transfer, no complex interactions",
                "risk_score": 0.15
            }
        },
        "recommendation": "Safe to execute"
    })))
}