use alloy_primitives::{Address, Bytes, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeTransaction {
    pub to: Address,
    pub value: U256,
    pub data: Bytes,
    pub operation: u8,
    pub safe_tx_gas: U256,
    pub base_gas: U256,
    pub gas_price: U256,
    pub gas_token: Address,
    pub refund_receiver: Address,
    pub nonce: U256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub signer: Address,
    pub signature: Bytes,
}

impl SafeTransaction {
    pub fn new_simple_transfer(to: Address, value: U256, nonce: U256) -> Self {
        Self {
            to,
            value,
            data: Bytes::new(),
            operation: 0, // Call
            safe_tx_gas: U256::ZERO,
            base_gas: U256::ZERO,
            gas_price: U256::ZERO,
            gas_token: Address::ZERO,
            refund_receiver: Address::ZERO,
            nonce,
        }
    }
    
    pub fn encode_for_signing(&self) -> Vec<u8> {
        // EIP-712 encoding would go here
        // For demo, we'll use a simplified version
        let mut data = Vec::new();
        data.extend_from_slice(self.to.as_slice());
        data.extend_from_slice(&self.value.to_be_bytes::<32>());
        data.extend_from_slice(self.data.as_ref());
        data.push(self.operation);
        data.extend_from_slice(&self.nonce.to_be_bytes::<32>());
        data
    }
}

pub fn encode_signatures(signatures: &[Signature]) -> Bytes {
    // Safe signature encoding: sorted by signer address
    let mut sorted_sigs = signatures.to_vec();
    sorted_sigs.sort_by_key(|s| s.signer);
    
    let mut encoded = Vec::new();
    for sig in sorted_sigs {
        // r (32 bytes) + s (32 bytes) + v (1 byte)
        encoded.extend_from_slice(sig.signature.as_ref());
    }
    
    Bytes::from(encoded)
}