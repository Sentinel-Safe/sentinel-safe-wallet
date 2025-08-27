# Multi-Signature Demo

This demo shows the core functionality of the Sentinel Safe Wallet: collecting 4 out of 5 signatures to execute a transaction.

## Demo Flow

The demo simulates:
1. Creating a transaction proposal
2. Collecting signatures from 4 different signers (2 humans + 2 AI agents)
3. Executing the transaction once 4 signatures are collected

## Running the Demo

### 1. Start the Orchestrator

```bash
# From the root directory
make demo-orchestrator

# Or directly
cd backend && cargo run --bin orchestrator
```

The orchestrator will start on `http://localhost:3001`

### 2. Run the Demo Script

In another terminal:

```bash
# From the root directory
make demo-test

# Or directly
cd backend && ./demo_test.sh
```

## Demo API Endpoints

- `POST /api/v1/transactions` - Create a new transaction proposal
- `GET /api/v1/transactions/:tx_id` - Get transaction details
- `POST /api/v1/transactions/:tx_id/sign` - Add a signature
- `GET /api/v1/transactions/:tx_id/status` - Check signature collection status
- `POST /api/v1/transactions/:tx_id/execute` - Execute transaction (requires 4+ signatures)

## Example Transaction Flow

```bash
# 1. Create Transaction
curl -X POST http://localhost:3001/api/v1/transactions \
  -H "Content-Type: application/json" \
  -d '{
    "to": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb5",
    "value": "1000000000000000",
    "data": null
  }'

# 2. Sign with 4 Different Signers
curl -X POST http://localhost:3001/api/v1/transactions/{tx_id}/sign \
  -H "Content-Type: application/json" \
  -d '{
    "signer_address": "0x1111111111111111111111111111111111111111",
    "use_demo_signer": true
  }'

# 3. Execute Transaction
curl -X POST http://localhost:3001/api/v1/transactions/{tx_id}/execute
```

## Key Features Demonstrated

✅ **4-of-5 Multi-signature**: Requires exactly 4 signatures to execute  
✅ **Signature Collection**: Tracks which addresses have signed  
✅ **Duplicate Prevention**: Each address can only sign once  
✅ **Status Tracking**: Shows current collection progress  
✅ **Ready State**: Automatically marks transaction as ready when 4 signatures collected  

## For Video Demo

The demo shows:
1. **Transaction Creation** - Proposing a transfer
2. **Progressive Signing** - Each signer adds their signature (1/4, 2/4, 3/4, 4/4)
3. **Execution** - Transaction executes only after 4 signatures
4. **Security** - Cannot execute with less than 4 signatures

This demonstrates the core security model where a super majority (4 out of 5) is required for any transaction.