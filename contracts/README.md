# Sentinel Safe Wallet - Smart Contracts

Smart contracts for the AI-collaborative multi-signature wallet on Kaia Kairos testnet.

## Overview

- **SafeRoleGuard.sol**: Enforces 2-human-3-AI signer composition
- **Deployment Target**: Kaia Kairos Testnet Only (Chain ID: 1001)
- **RPC Endpoint**: https://public-en-kairos.node.kaia.io

## Prerequisites

```bash
# Install Foundry
curl -L https://foundry.paradigm.xyz | bash
foundryup
```

## Quick Start

```bash
# Install dependencies
forge install

# Build contracts
forge build

# Run tests
forge test

# Run tests with gas report
forge test --gas-report
```

## Kaia Kairos Testnet Deployment

### 1. Set Environment Variables

Create a `.env` file in the contracts directory:

```env
# Kaia Kairos Testnet RPC
KAIROS_RPC_URL=https://public-en-kairos.node.kaia.io

# Deployment private key (use a testnet-only key!)
DEPLOYER_PRIVATE_KEY=0x...

# Safe configuration
SAFE_THRESHOLD=4
HUMAN_SIGNER_1=0x...
HUMAN_SIGNER_2=0x...
AI_CFO_ADDRESS=0x...
AI_SECURITY_ADDRESS=0x...
AI_ANALYST_ADDRESS=0x...
```

### 2. Deploy to Kairos Testnet

```bash
# Deploy contracts
forge script script/DeployWithKaiaSafe.s.sol:DeployKaiaTestnet \
    --rpc-url $KAIROS_RPC_URL \
    --private-key $DEPLOYER_PRIVATE_KEY \
    --broadcast \
    --verify \
    -vvvv

# Or using make command from root
make contracts-deploy-testnet
```

### 3. Verify Contract

```bash
forge verify-contract <CONTRACT_ADDRESS> src/SafeRoleGuard.sol:SafeRoleGuard \
    --chain-id 1001 \
    --etherscan-api-key <API_KEY>
```

## Test Kaia (KLAY) Faucet

Get test KLAY for Kairos testnet:
- https://faucet.kaia.io/
- https://kairos.wallet.klaytn.foundation/faucet

## Contract Addresses (Kairos Testnet)

After deployment, update these in your `.env`:

```env
SAFE_ROLE_GUARD_ADDRESS=0x...
SAFE_PROXY_ADDRESS=0x...
```

## Testing

```bash
# Run all tests
forge test

# Run specific test
forge test --match-test testSuperMajority

# Run with verbosity
forge test -vvvv

# Fork testing from Kairos
forge test --fork-url https://public-en-kairos.node.kaia.io
```

## Gas Optimization

```bash
# Generate gas snapshot
forge snapshot

# Compare with previous snapshot
forge snapshot --diff
```

## Security Considerations

- This is for TESTNET ONLY - never deploy to mainnet with test keys
- Always use separate keys for testnet
- The 4-of-5 multisig requires super majority for all transactions
- Role guard ensures exactly 2 humans and 3 AI signers

## Useful Commands

```bash
# Clean build artifacts
forge clean

# Format Solidity code
forge fmt

# Update dependencies
forge update

# Check contract size
forge build --sizes
```

## Kaia Network Info

- **Network**: Kaia Kairos Testnet
- **Chain ID**: 1001
- **Currency**: KLAY
- **Block Explorer**: https://kairos.kaiascope.com/
- **RPC URL**: https://public-en-kairos.node.kaia.io
- **WebSocket**: wss://public-en-kairos.node.kaia.io/ws
