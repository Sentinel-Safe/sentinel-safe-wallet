const express = require('express');
const cors = require('cors');

const app = express();
const PORT = process.env.PORT || 3001;

app.use(cors());
app.use(express.json());

// Placeholder: Transaction proposal endpoint
app.post('/api/transactions/propose', async (req, res) => {
  console.log('Transaction proposal received:', req.body);
  // TODO: Implement transaction proposal logic
  res.json({ 
    message: 'Transaction proposal placeholder',
    transactionId: 'tx_' + Date.now() 
  });
});

// Placeholder: Get transaction status
app.get('/api/transactions/:id', async (req, res) => {
  console.log('Get transaction status:', req.params.id);
  // TODO: Implement transaction status retrieval
  res.json({ 
    id: req.params.id,
    status: 'pending',
    signatures: '0/5'
  });
});

// Placeholder: Collect signatures
app.post('/api/transactions/:id/sign', async (req, res) => {
  console.log('Signature received for transaction:', req.params.id);
  // TODO: Implement signature collection logic
  res.json({ 
    message: 'Signature collection placeholder',
    currentSignatures: '1/5'
  });
});

// Placeholder: Submit transaction to blockchain
app.post('/api/transactions/:id/submit', async (req, res) => {
  console.log('Submit transaction to blockchain:', req.params.id);
  // TODO: Implement blockchain submission logic
  res.json({ 
    message: 'Transaction submission placeholder',
    txHash: '0x' + '0'.repeat(64)
  });
});

// Health check
app.get('/health', (req, res) => {
  res.json({ status: 'ok', service: 'orchestrator' });
});

app.listen(PORT, () => {
  console.log(`Orchestrator API running on port ${PORT}`);
});