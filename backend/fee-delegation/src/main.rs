use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    fee_payer_address: String,
}

impl AppState {
    fn new() -> Self {
        let fee_payer_address = std::env::var("FEE_PAYER_ADDRESS")
            .unwrap_or_else(|_| "0x0000000000000000000000000000000000000000".to_string());

        Self { fee_payer_address }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DelegatedTransaction {
    from: String,
    to: String,
    value: String,
    data: String,
    gas: String,
    gas_price: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DelegationRequest {
    transaction: DelegatedTransaction,
    user_signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DelegationResponse {
    transaction_hash: String,
    fee_payer: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    service: String,
    fee_payer: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FeeEstimate {
    estimated_fee: String,
    gas_price: String,
    gas_limit: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .compact(),
        )
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "fee_delegation=debug,tower_http=debug".into()),
        )
        .init();

    dotenv::dotenv().ok();

    let state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/delegate", post(delegate_fee))
        .route("/api/v1/estimate", post(estimate_fee))
        .route("/api/v1/status/:tx_hash", get(get_delegation_status))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();

    info!("Fee Delegation service listening on http://0.0.0.0:3003");

    axum::serve(listener, app).await.unwrap();
}

async fn health(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "fee-delegation".to_string(),
        fee_payer: state.fee_payer_address.clone(),
    })
}

async fn delegate_fee(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DelegationRequest>,
) -> Result<(StatusCode, Json<DelegationResponse>), StatusCode> {
    info!(
        "Delegating fee for transaction from {} to {}",
        request.transaction.from, request.transaction.to
    );

    let tx_hash = format!("0x{}", uuid::Uuid::new_v4().simple());

    Ok((
        StatusCode::OK,
        Json(DelegationResponse {
            transaction_hash: tx_hash,
            fee_payer: state.fee_payer_address.clone(),
            status: "pending".to_string(),
        }),
    ))
}

async fn estimate_fee(
    State(_state): State<Arc<AppState>>,
    Json(transaction): Json<DelegatedTransaction>,
) -> Result<Json<FeeEstimate>, StatusCode> {
    info!("Estimating fee for transaction to: {}", transaction.to);

    Ok(Json(FeeEstimate {
        estimated_fee: "1000000000000000".to_string(),
        gas_price: "25000000000".to_string(),
        gas_limit: "21000".to_string(),
    }))
}

async fn get_delegation_status(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(tx_hash): axum::extract::Path<String>,
) -> Result<Json<DelegationResponse>, StatusCode> {
    info!("Getting delegation status for: {}", tx_hash);

    Ok(Json(DelegationResponse {
        transaction_hash: tx_hash,
        fee_payer: state.fee_payer_address.clone(),
        status: "confirmed".to_string(),
    }))
}
