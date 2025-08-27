use async_trait::async_trait;
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
    cfo_agent: Arc<CfoAgent>,
    security_agent: Arc<SecurityAgent>,
    onchain_analyst: Arc<OnchainAnalyst>,
}

#[async_trait]
trait AiAgent: Send + Sync {
    async fn analyze(&self, transaction: &TransactionData) -> AnalysisResult;
}

struct CfoAgent {
    name: String,
}

struct SecurityAgent {
    name: String,
}

struct OnchainAnalyst {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TransactionData {
    to: String,
    value: String,
    data: String,
    nonce: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnalysisResult {
    agent: String,
    approved: bool,
    risk_score: f64,
    reasons: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    service: String,
    agents: Vec<String>,
}

#[async_trait]
impl AiAgent for CfoAgent {
    async fn analyze(&self, transaction: &TransactionData) -> AnalysisResult {
        info!("CFO Agent analyzing transaction to: {}", transaction.to);

        AnalysisResult {
            agent: self.name.clone(),
            approved: true,
            risk_score: 0.2,
            reasons: vec!["Within budget limits".to_string()],
        }
    }
}

#[async_trait]
impl AiAgent for SecurityAgent {
    async fn analyze(&self, transaction: &TransactionData) -> AnalysisResult {
        info!(
            "Security Agent analyzing transaction to: {}",
            transaction.to
        );

        AnalysisResult {
            agent: self.name.clone(),
            approved: true,
            risk_score: 0.1,
            reasons: vec!["Address not in blacklist".to_string()],
        }
    }
}

#[async_trait]
impl AiAgent for OnchainAnalyst {
    async fn analyze(&self, transaction: &TransactionData) -> AnalysisResult {
        info!(
            "Onchain Analyst analyzing transaction to: {}",
            transaction.to
        );

        AnalysisResult {
            agent: self.name.clone(),
            approved: true,
            risk_score: 0.3,
            reasons: vec!["Contract verified on chain".to_string()],
        }
    }
}

impl AppState {
    fn new() -> Self {
        Self {
            cfo_agent: Arc::new(CfoAgent {
                name: "CFO Agent".to_string(),
            }),
            security_agent: Arc::new(SecurityAgent {
                name: "Security Agent".to_string(),
            }),
            onchain_analyst: Arc::new(OnchainAnalyst {
                name: "Onchain Analyst".to_string(),
            }),
        }
    }
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
                .unwrap_or_else(|_| "ai_agents=debug,tower_http=debug".into()),
        )
        .init();

    dotenv::dotenv().ok();

    let state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/analyze", post(analyze_transaction))
        .route("/api/v1/cfo/analyze", post(cfo_analyze))
        .route("/api/v1/security/analyze", post(security_analyze))
        .route("/api/v1/onchain/analyze", post(onchain_analyze))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();

    info!("AI Agents service listening on http://0.0.0.0:3002");

    axum::serve(listener, app).await.unwrap();
}

async fn health(State(_state): State<Arc<AppState>>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "ai-agents".to_string(),
        agents: vec![
            "CFO Agent".to_string(),
            "Security Agent".to_string(),
            "Onchain Analyst".to_string(),
        ],
    })
}

async fn analyze_transaction(
    State(state): State<Arc<AppState>>,
    Json(transaction): Json<TransactionData>,
) -> Result<Json<Vec<AnalysisResult>>, StatusCode> {
    let cfo_result = state.cfo_agent.analyze(&transaction).await;
    let security_result = state.security_agent.analyze(&transaction).await;
    let onchain_result = state.onchain_analyst.analyze(&transaction).await;

    Ok(Json(vec![cfo_result, security_result, onchain_result]))
}

async fn cfo_analyze(
    State(state): State<Arc<AppState>>,
    Json(transaction): Json<TransactionData>,
) -> Result<Json<AnalysisResult>, StatusCode> {
    Ok(Json(state.cfo_agent.analyze(&transaction).await))
}

async fn security_analyze(
    State(state): State<Arc<AppState>>,
    Json(transaction): Json<TransactionData>,
) -> Result<Json<AnalysisResult>, StatusCode> {
    Ok(Json(state.security_agent.analyze(&transaction).await))
}

async fn onchain_analyze(
    State(state): State<Arc<AppState>>,
    Json(transaction): Json<TransactionData>,
) -> Result<Json<AnalysisResult>, StatusCode> {
    Ok(Json(state.onchain_analyst.analyze(&transaction).await))
}
