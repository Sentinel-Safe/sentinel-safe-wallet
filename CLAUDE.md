# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Sentinel Safe Wallet is an AI-collaborative multi-signature wallet that requires super majority approval (4-of-5 signatures) for all transactions. The wallet consists of 2 human signers and 3 AI agents that automatically analyze and sign transactions based on security, accounting, and pattern recognition rules. Built on Kaia blockchain with EVM compatibility, it combines on-chain security enforcement with off-chain AI analysis.

## Key Features

- **4-of-5 Multi-signature**: Requires approval from at least 4 out of 5 signers (2 humans + 3 AI agents)
- **AI Agent Analysis**: Three specialized AI agents (CFO, Security Expert, On-chain Analyst) automatically evaluate transactions
- **Fee Delegation**: Kaia's native feature allows users to transact without holding KAIA for gas fees
- **Role Enforcement**: On-chain guard ensures the 2-human-3-AI composition cannot be altered

## Build and Development Commands

### Backend Services (Node.js)

```bash
# Install dependencies for all services
npm install

# Run the orchestrator API server
npm run dev:orchestrator

# Run AI agent services
npm run dev:ai-agents

# Run fee delegation server
npm run dev:fee-delegation

# Run all services concurrently
npm run dev

# Run tests
npm test

# Run specific test suite
npm test -- --grep "orchestrator"

# Lint code
npm run lint

# Format code
npm run format
```

### Smart Contracts (Foundry)

```bash
# Navigate to contracts directory
cd contracts/

# Install dependencies
forge install

# Build contracts
forge build

# Run contract tests
forge test

# Run specific test
forge test --match-test testSuperMajority

# Run tests with gas reporting
forge test --gas-report

# Format Solidity code
forge fmt

# Deploy to Kaia Kairos testnet
forge script script/DeploySafe.s.sol --rpc-url https://public-en.kairos.node.kaia.io --broadcast

# Verify contract
forge verify-contract <CONTRACT_ADDRESS> src/KaiaSafe.sol:KaiaSafe --chain-id 1001
```

### Frontend (React/Next.js)

```bash
# Navigate to frontend directory
cd frontend/

# Install dependencies
npm install

# Run development server
npm run dev

# Build for production
npm run build

# Run production server
npm start

# Run tests
npm test
```

## Architecture

### On-Chain Components (Solidity)

- **`KaiaSafe.sol`**: Core multi-signature contract enforcing 4-of-5 approval policy, based on Gnosis Safe
- **`RoleGuard.sol`**: Ensures owner composition remains exactly 2 humans and 3 AI agents
- **`FeeDelegation.sol`**: Handles Kaia's fee delegation for gasless transactions

### Off-Chain Services (Node.js)

- **`orchestrator/`**: Central API that manages transaction proposals, collects signatures, and submits to blockchain
  - Transaction queue management
  - Signature collection from all 5 signers
  - WebSocket for real-time updates
  
- **`ai-agents/`**: Three independent AI services that analyze and sign transactions
  - **CFO Agent**: Validates against internal financial rules and budgets
  - **Security Agent**: Checks external threat databases and blacklists
  - **On-chain Analyst**: Parses transaction data and smart contract risks
  
- **`fee-delegation/`**: Manages Kaia fee delegation for gasless user transactions

### Frontend Dashboard (React)

- **`components/`**: Reusable UI components
- **`pages/`**: Next.js pages for routing
- **`hooks/`**: Custom React hooks for wallet interaction
- **`utils/`**: Helper functions and Safe SDK integration

## Transaction Flow

1. User proposes transaction via Dashboard
2. Orchestrator notifies all 5 signers (2 humans + 3 AI agents)
3. AI agents analyze transaction in parallel:
   - CFO checks budget compliance
   - Security verifies address reputation
   - Analyst parses transaction data
4. Humans review AI analysis and provide signatures
5. Once 4+ signatures collected, orchestrator submits to blockchain
6. KaiaSafe contract verifies 4-of-5 policy and executes

## Key Dependencies

### Smart Contracts
- **Foundry**: Development framework
- **OpenZeppelin**: Security-audited contract libraries
- **Safe Contracts**: Multi-signature wallet implementation

### Backend
- **Express.js/Fastify**: API framework
- **Ethers.js/Viem**: Blockchain interaction
- **Kaia SDK**: Fee delegation support
- **PostgreSQL**: Transaction history storage
- **Redis**: Caching and queue management
- **Socket.io**: Real-time updates

### Frontend
- **Next.js**: React framework
- **Wagmi/Viem**: Wallet connection
- **Safe SDK**: Multi-sig transaction handling
- **TailwindCSS**: Styling

## Environment Variables

```bash
# Blockchain
KAIA_RPC_URL=https://public-en.kairos.node.kaia.io
SAFE_CONTRACT_ADDRESS=
ROLE_GUARD_ADDRESS=

# AI Agent Private Keys
AI_CFO_PRIVATE_KEY=
AI_SECURITY_PRIVATE_KEY=
AI_ANALYST_PRIVATE_KEY=

# Fee Delegation
FEE_PAYER_PRIVATE_KEY=

# Database
DATABASE_URL=postgresql://...
REDIS_URL=redis://...

# API Keys
CHAINALYSIS_API_KEY=
TRM_LABS_API_KEY=
```

## Testing Strategy

- **Smart Contracts**: Foundry tests for all security-critical functions, especially 4-of-5 policy enforcement
- **Backend**: Jest/Mocha tests for API endpoints, AI agent logic, signature collection
- **Integration**: E2E tests simulating full transaction flow from proposal to execution
- **AI Agents**: Unit tests for each analysis rule and decision logic