.PHONY: help build test clean dev fmt check

# Default target
help:
	@echo "Available commands:"
	@echo "  make build          - Build all backend services"
	@echo "  make build-release  - Build all backend services in release mode"
	@echo "  make test           - Run all tests"
	@echo "  make clean          - Clean build artifacts"
	@echo "  make dev            - Run all services in development mode"
	@echo "  make fmt            - Format all code"
	@echo "  make check          - Check code without building"
	@echo "  make clippy         - Run clippy linter"
	@echo ""
	@echo "Backend service commands:"
	@echo "  make orchestrator   - Run orchestrator service"
	@echo "  make ai-agents      - Run AI agents service"
	@echo "  make fee-delegation - Run fee delegation service"
	@echo ""
	@echo "Contract commands:"
	@echo "  make contracts-build - Build smart contracts"
	@echo "  make contracts-test  - Test smart contracts"
	@echo "  make contracts-fmt   - Format Solidity code"

# Backend commands
build:
	cd backend && cargo build

build-release:
	cd backend && cargo build --release

test:
	cd backend && cargo test

clean:
	cd backend && cargo clean
	cd contracts && forge clean

fmt:
	cd backend && cargo fmt
	cd contracts && forge fmt

check:
	cd backend && cargo check

clippy:
	cd backend && cargo clippy -- -D warnings

# Run individual services
orchestrator:
	cd backend && cargo run --bin orchestrator

ai-agents:
	cd backend && cargo run --bin ai-agents

fee-delegation:
	cd backend && cargo run --bin fee-delegation

# Run all services (requires GNU parallel or similar)
dev:
	@echo "Starting all backend services..."
	@make -j3 orchestrator ai-agents fee-delegation

# Run demo of multi-signature flow
demo-orchestrator:
	@echo "🚀 Starting orchestrator for demo..."
	cd backend && cargo run --bin orchestrator

demo-test:
	@echo "🔐 Running multi-signature demo..."
	cd backend && ./demo_test.sh

# Contract commands
contracts-build:
	cd contracts && forge build

contracts-test:
	cd contracts && forge test

contracts-test-gas:
	cd contracts && forge test --gas-report

contracts-fmt:
	cd contracts && forge fmt

contracts-clean:
	cd contracts && forge clean

contracts-size:
	cd contracts && forge build --sizes

# Kaia Kairos Testnet deployment (TESTNET ONLY!)
contracts-deploy-testnet:
	@echo "⚠️  Deploying to Kaia Kairos TESTNET (Chain ID: 1001)"
	@echo "Make sure you have set environment variables in contracts/.env"
	cd contracts && forge script script/DeployKairos.s.sol:DeployKairos \
		--rpc-url https://public-en.kairos.node.kaia.io \
		--broadcast \
		-vvvv

contracts-deploy-testnet-verify:
	@echo "⚠️  Deploying to Kaia Kairos TESTNET with verification"
	cd contracts && forge script script/DeployKairos.s.sol:DeployKairos \
		--rpc-url https://public-en.kairos.node.kaia.io \
		--broadcast \
		--verify \
		-vvvv

contracts-deploy-dry-run:
	@echo "🔍 Dry run deployment (no broadcast)"
	cd contracts && forge script script/DeployKairos.s.sol:DeployKairos \
		--rpc-url https://public-en.kairos.node.kaia.io \
		-vvvv

contracts-verify:
	@echo "Verify contract on Kairos testnet explorer:"
	@echo "forge verify-contract <ADDRESS> src/SafeRoleGuard.sol:SafeRoleGuard --chain-id 1001"

# Installation helpers
install-rust:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

install-foundry:
	curl -L https://foundry.paradigm.xyz | bash