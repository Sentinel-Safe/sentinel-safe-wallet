use alloy::{
    network::EthereumWallet,
    primitives::{Address, Bytes, B256, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol,
};
use anyhow::Result;
use std::str::FromStr;

use crate::safe_contract::Signature;

// Define Safe interface using sol! macro
sol!(
    #[allow(clippy::too_many_arguments)]
    #[allow(missing_docs)]
    #[sol(rpc)]
    ISafe,
    "src/ISafe.json"
);

pub struct SafeExecutor {
    rpc_url: String,
    safe_address: Address,
}

impl SafeExecutor {
    pub async fn new(rpc_url: &str, safe_address: &str) -> Result<Self> {
        let safe_addr = Address::from_str(safe_address)?;

        Ok(Self {
            rpc_url: rpc_url.to_string(),
            safe_address: safe_addr,
        })
    }

    pub async fn get_nonce(&self) -> Result<U256> {
        // Create provider for this call
        let provider = ProviderBuilder::new().connect_http(self.rpc_url.parse()?);
        let safe = ISafe::ISafeInstance::new(self.safe_address, &provider);

        let nonce = safe.nonce().call().await?;
        Ok(nonce)
    }

    pub async fn get_transaction_hash(
        &self,
        to: Address,
        value: U256,
        data: Bytes,
        nonce: U256,
    ) -> Result<B256> {
        let provider = ProviderBuilder::new().connect_http(self.rpc_url.parse()?);
        let safe = ISafe::ISafeInstance::new(self.safe_address, &provider);

        let tx_hash = safe
            .getTransactionHash(
                to,
                value,
                data,
                0u8,           // operation: 0 = Call
                U256::ZERO,    // safeTxGas
                U256::ZERO,    // baseGas
                U256::ZERO,    // gasPrice
                Address::ZERO, // gasToken
                Address::ZERO, // refundReceiver
                nonce,
            )
            .call()
            .await?;

        Ok(tx_hash)
    }

    pub async fn execute_transaction(
        &self,
        to: Address,
        value: U256,
        data: Bytes,
        signatures: Vec<Signature>,
    ) -> Result<B256> {
        // Get executor private key from environment or use a default one
        // In production, this should be a proper relayer account with gas
        let executor_key = std::env::var("EXECUTOR_PRIVATE_KEY")
            .or_else(|_| std::env::var("DEPLOYER_PRIVATE_KEY"))
            .unwrap_or_else(|_| {
                // Default test key - should have some KAIA for gas
                tracing::warn!("No EXECUTOR_PRIVATE_KEY found, using test key");
                "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string()
            });

        // Create signer from private key
        let signer = PrivateKeySigner::from_str(&executor_key)?;
        let wallet = EthereumWallet::from(signer);

        // Create provider with wallet
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect_http(self.rpc_url.parse()?);

        let safe = ISafe::ISafeInstance::new(self.safe_address, &provider);

        // Sort signatures by signer address (required by Safe)
        let mut sorted_sigs = signatures.clone();
        sorted_sigs.sort_by_key(|s| s.signer);

        // Log signature details for debugging
        tracing::info!("Executing with {} signatures", sorted_sigs.len());
        for (i, sig) in sorted_sigs.iter().enumerate() {
            tracing::info!(
                "Signature {}: signer={}, sig_len={}",
                i,
                sig.signer,
                sig.signature.len()
            );
        }

        // Encode signatures for Safe (r + s + v format)
        let encoded_signatures = encode_signatures(&sorted_sigs);
        tracing::info!(
            "Total encoded signatures length: {} bytes",
            encoded_signatures.len()
        );

        // Execute the transaction
        let pending_tx = safe
            .execTransaction(
                to,
                value,
                data,
                0u8,           // operation: 0 = Call
                U256::ZERO,    // safeTxGas
                U256::ZERO,    // baseGas
                U256::ZERO,    // gasPrice
                Address::ZERO, // gasToken
                Address::ZERO, // refundReceiver
                Bytes::from(encoded_signatures),
            )
            .send()
            .await?;

        // Get transaction hash before moving pending_tx
        let tx_hash = *pending_tx.tx_hash();

        // Wait for confirmation
        let _receipt = pending_tx.get_receipt().await?;

        Ok(tx_hash)
    }
}

fn encode_signatures(signatures: &[Signature]) -> Vec<u8> {
    let mut encoded = Vec::new();

    for sig in signatures {
        // Safe expects signatures in format: r (32 bytes) + s (32 bytes) + v (1 byte)
        // The signature bytes should already be in this format from the client
        encoded.extend_from_slice(&sig.signature);
    }

    encoded
}
