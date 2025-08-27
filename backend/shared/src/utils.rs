use alloy_primitives::{Address, B256, U256};

pub fn parse_address(addr: &str) -> Result<Address, String> {
    addr.parse::<Address>()
        .map_err(|e| format!("Invalid address: {}", e))
}

pub fn parse_u256(value: &str) -> Result<U256, String> {
    value
        .parse::<U256>()
        .map_err(|e| format!("Invalid U256 value: {}", e))
}

pub fn parse_b256(hash: &str) -> Result<B256, String> {
    hash.parse::<B256>()
        .map_err(|e| format!("Invalid B256 hash: {}", e))
}

pub fn validate_signature(signature: &str) -> bool {
    signature.starts_with("0x") && signature.len() == 132
}

pub fn calculate_safe_hash(_to: &str, _value: &str, _data: &str, nonce: u64) -> String {
    format!("0x{:064x}", nonce)
}
