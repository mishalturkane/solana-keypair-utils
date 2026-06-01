//! # Solana Keypair Utils
//!
//! A Rust library for managing Solana keypairs with easy conversion between Base58 and JSON formats.
//!
//! ## Features
//! - Convert between Base58 and JSON keypair formats
//! - Extract and work with keypair bytes
//! - Generate vanity addresses with custom prefixes
//! - Support for Ed25519 keypairs
//!
//! ## Example
//!
//! ```ignore
//! use solana_keypair_utils::base58_to_json_bytes;
//!
//! let base58_key = "5KL9q...";
//! let bytes = base58_to_json_bytes(base58_key)?;
//! println!("Keypair bytes: {:?}", bytes);
//! # Ok::<(), String>(())
//! ```

pub mod keypair;

pub use keypair::{
    base58_to_json::{base58_to_json, base58_to_json_bytes},
    json_to_base58::{base58_to_bytes, bytes_to_base58, json_to_base58},
    search_key,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base58_conversion() {
        // Test that we can convert base58 to bytes
        let test_base58 = "11111111111111111111111111111111";
        let result = base58_to_json_bytes(test_base58);
        assert!(result.is_ok());
    }
}
