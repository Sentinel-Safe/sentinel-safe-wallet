# Sentinel Safe Wallet

AI-collaborative multi-signature wallet that requires super majority approval (4-of-5 signatures) for all transactions.

## Quick Start

```bash
# Install dependencies
make install-rust     # Install Rust
make install-foundry  # Install Foundry

# Build everything
make build            # Build backend
make contracts-build  # Build contracts

# Run services
make dev              # Run all backend services
make orchestrator     # Run orchestrator only
make ai-agents        # Run AI agents only
make fee-delegation   # Run fee delegation only

# Test
make test             # Test backend
make contracts-test   # Test contracts
```

## Project Structure

```
sentinel-safe-wallet/
├── backend/              # Rust backend services
│   ├── orchestrator/     # Transaction proposal management
│   ├── ai-agents/        # AI agent services (CFO, Security, Analyst)
│   ├── fee-delegation/   # Kaia fee delegation
│   └── shared/           # Shared types and utilities
├── contracts/            # Solidity smart contracts
│   ├── src/              # Contract source code
│   ├── test/             # Contract tests
│   └── script/           # Deployment scripts
├── Makefile              # Build and run commands
└── CLAUDE.md             # AI assistant instructions
```

## Technology Stack

- **Backend**: Rust with Axum framework and Alloy for blockchain interaction
- **Smart Contracts**: Solidity with Foundry development framework
- **Blockchain**: Kaia (EVM-compatible)
