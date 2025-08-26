const express = require('express');
const cors = require('cors');
const CFOAgent = require('./cfo-agent');
const SecurityAgent = require('./security-agent');
const OnchainAnalyst = require('./onchain-analyst');

const app = express();
const PORT = process.env.AI_AGENTS_PORT || 3002;

app.use(cors());
app.use(express.json());

// Initialize AI agents (with placeholder private keys)
const agents = {
  cfo: new CFOAgent(process.env.AI_CFO_PRIVATE_KEY || '0x' + '1'.repeat(64)),
  security: new SecurityAgent(process.env.AI_SECURITY_PRIVATE_KEY || '0x' + '2'.repeat(64)),
  analyst: new OnchainAnalyst(process.env.AI_ANALYST_PRIVATE_KEY || '0x' + '3'.repeat(64))
};

// Placeholder: Analyze transaction
app.post('/api/analyze', async (req, res) => {
  const { transaction } = req.body;
  console.log('AI Agents analyzing transaction:', transaction);
  
  try {
    // Run all agents in parallel
    const [cfoAnalysis, securityAnalysis, analystAnalysis] = await Promise.all([
      agents.cfo.analyzeTransaction(transaction),
      agents.security.analyzeTransaction(transaction),
      agents.analyst.analyzeTransaction(transaction)
    ]);

    res.json({
      analyses: {
        cfo: cfoAnalysis,
        security: securityAnalysis,
        analyst: analystAnalysis
      },
      overallApproval: cfoAnalysis.approved && securityAnalysis.approved && analystAnalysis.approved
    });
  } catch (error) {
    console.error('Analysis error:', error);
    res.status(500).json({ error: 'Analysis failed' });
  }
});

// Placeholder: Sign transaction
app.post('/api/sign', async (req, res) => {
  const { transaction, agentType } = req.body;
  console.log(`Agent ${agentType} signing transaction`);
  
  try {
    const agent = agents[agentType];
    if (!agent) {
      return res.status(400).json({ error: 'Invalid agent type' });
    }

    const signature = await agent.signTransaction(transaction);
    res.json(signature);
  } catch (error) {
    console.error('Signing error:', error);
    res.status(500).json({ error: 'Signing failed' });
  }
});

// Health check
app.get('/health', (req, res) => {
  res.json({ 
    status: 'ok', 
    service: 'ai-agents',
    agents: Object.keys(agents)
  });
});

app.listen(PORT, () => {
  console.log(`AI Agents service running on port ${PORT}`);
});