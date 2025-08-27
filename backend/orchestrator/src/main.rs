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
}

impl AppState {
    fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TransactionProposal {
    to: String,
    value: String,
    data: String,
    nonce: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProposalResponse {
    proposal_id: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Signature {
    signer: String,
    signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    service: String,
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
                .unwrap_or_else(|_| "orchestrator=debug,tower_http=debug".into()),
        )
        .init();

    dotenv::dotenv().ok();

    let state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/proposals", post(create_proposal))
        .route("/api/v1/proposals/:id", get(get_proposal))
        .route("/api/v1/proposals/:id/signatures", post(add_signature))
        .route("/api/v1/proposals/:id/execute", post(execute_transaction))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .unwrap();

    info!("Orchestrator service listening on http://0.0.0.0:3001");

    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "orchestrator".to_string(),
    })
}

async fn create_proposal(
    State(_state): State<Arc<AppState>>,
    Json(proposal): Json<TransactionProposal>,
) -> Result<(StatusCode, Json<ProposalResponse>), StatusCode> {
    info!("Creating proposal: {:?}", proposal);
    
    let proposal_id = uuid::Uuid::new_v4().to_string();
    
    Ok((
        StatusCode::CREATED,
        Json(ProposalResponse {
            proposal_id,
            status: "pending".to_string(),
        }),
    ))
}

async fn get_proposal(
    State(_state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ProposalResponse>, StatusCode> {
    info!("Getting proposal: {}", id);
    
    Ok(Json(ProposalResponse {
        proposal_id: id,
        status: "pending".to_string(),
    }))
}

async fn add_signature(
    State(_state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(signature): Json<Signature>,
) -> Result<StatusCode, StatusCode> {
    info!("Adding signature to proposal {}: {:?}", id, signature);
    
    Ok(StatusCode::OK)
}

async fn execute_transaction(
    State(_state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ProposalResponse>, StatusCode> {
    info!("Executing transaction for proposal: {}", id);
    
    Ok(Json(ProposalResponse {
        proposal_id: id,
        status: "executed".to_string(),
    }))
}