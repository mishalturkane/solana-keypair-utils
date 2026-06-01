use std::fs;

/// Converts a JSON keypair array to Base58 encoded private key
///
/// # Arguments
/// * `filename` - Input JSON keypair file
///
/// # Returns
/// Result with Base58 encoded private key or error message
pub fn json_to_base58(filename: &str) -> Result<String, String> {
    let content = fs::read_to_string(filename).map_err(|_| "File not found".to_string())?;

    let bytes: Vec<u8> = serde_json::from_str(&content).map_err(|_| "Invalid JSON".to_string())?;

    let private_key_base58 = bs58::encode(&bytes).into_string();

    Ok(private_key_base58)
}

/// Converts raw bytes to Base58 encoded string
pub fn bytes_to_base58(bytes: Vec<u8>) -> String {
    bs58::encode(&bytes).into_string()
}

/// Converts Base58 string to raw bytes
pub fn base58_to_bytes(base58_str: &str) -> Result<Vec<u8>, String> {
    bs58::decode(base58_str)
        .into_vec()
        .map_err(|_| "Invalid base58 string".to_string())
}
