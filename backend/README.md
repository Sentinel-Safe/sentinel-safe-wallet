# Sentinel Safe Wallet - Rust Backend

AI-collaborative multi-signature wallet backend services built with Rust and Axum.

## Architecture

This backend consists of three microservices:

- **Orchestrator** (port 3001): Central API managing transaction proposals and signature collection
- **AI Agents** (port 3002): Three AI agents (CFO, Security, On-chain Analyst) analyzing transactions
- **Fee Delegation** (port 3003): Kaia fee delegation service for gasless transactions

## Prerequisites

- Rust 1.75 or higher
- PostgreSQL (optional, for persistent storage)
- Redis (optional, for caching)

## Quick Start

```bash
# Build all services
cargo build

# Run all tests
cargo test

# Run individual services
cargo run --bin orchestrator
cargo run --bin ai-agents
cargo run --bin fee-delegation
```

## Development

```bash
# Check code without compiling
cargo check

# Format code
cargo fmt

# Lint with clippy
cargo clippy -- -D warnings

# Watch and rebuild on changes (requires cargo-watch)
cargo install cargo-watch
cargo watch -x run --bin orchestrator
```

## Environment Variables

Create a `.env` file in the backend directory:

```env
# RPC
KAIA_RPC_URL=https://public-en-kairos.node.kaia.io

# Contract Addresses
SAFE_CONTRACT_ADDRESS=
ROLE_GUARD_ADDRESS=

# AI Agent Keys
AI_CFO_PRIVATE_KEY=
AI_SECURITY_PRIVATE_KEY=
AI_ANALYST_PRIVATE_KEY=

# Fee Delegation
FEE_PAYER_ADDRESS=
FEE_PAYER_PRIVATE_KEY=

# Database (optional)
DATABASE_URL=postgresql://user:password@localhost/sentinel_safe

# Redis (optional)
REDIS_URL=redis://localhost:6379
```

## API Endpoints

### Orchestrator Service (3001)
- `GET /health` - Health check
- `POST /api/v1/proposals` - Create transaction proposal
- `GET /api/v1/proposals/:id` - Get proposal details
- `POST /api/v1/proposals/:id/signatures` - Add signature
- `POST /api/v1/proposals/:id/execute` - Execute transaction

### AI Agents Service (3002)
- `GET /health` - Health check with agent status
- `POST /api/v1/analyze` - Analyze transaction with all agents
- `POST /api/v1/cfo/analyze` - CFO agent analysis
- `POST /api/v1/security/analyze` - Security agent analysis
- `POST /api/v1/onchain/analyze` - On-chain analyst analysis

### Fee Delegation Service (3003)
- `GET /health` - Health check with fee payer info
- `POST /api/v1/delegate` - Submit delegated transaction
- `POST /api/v1/estimate` - Estimate transaction fee
- `GET /api/v1/status/:tx_hash` - Get delegation status

## Testing

```bash
# Run all tests
cargo test

# Run tests for specific service
cargo test --bin orchestrator

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## Production Build

```bash
# Build optimized binaries
cargo build --release

# Run production binaries
./target/release/orchestrator
./target/release/ai-agents
./target/release/fee-delegation
```
