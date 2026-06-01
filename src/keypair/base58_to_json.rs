use std::fs;

/// Converts a Base58 encoded private key to JSON format
///
/// # Arguments
/// * `private_key_base58` - Base58 encoded private key string
/// * `filename` - Output filename for JSON keypair
///
/// # Returns
/// Result with JSON string or error message
pub fn base58_to_json(private_key_base58: &str, filename: &str) -> Result<String, String> {
    let keypair_bytes = bs58::decode(private_key_base58)
        .into_vec()
        .map_err(|_| "Invalid base58 private key".to_string())?;

    let json =
        serde_json::to_string(&keypair_bytes).map_err(|_| "JSON conversion failed".to_string())?;

    fs::write(filename, &json).map_err(|_| "File write failed".to_string())?;

    Ok(json)
}

/// Converts Base58 private key to JSON bytes without file writing
pub fn base58_to_json_bytes(private_key_base58: &str) -> Result<Vec<u8>, String> {
    bs58::decode(private_key_base58)
        .into_vec()
        .map_err(|_| "Invalid base58 private key".to_string())
}
