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
