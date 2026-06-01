mod keypair;

use dotenv::dotenv;
use std::env;
use std::fs;

use keypair::base58_to_json::base58_to_json;
use keypair::json_to_base58::json_to_base58;
use keypair::search_key::search_key;

fn main() {
    // .env load the env
    dotenv().ok();

    let private_key =
        env::var("PRIVATE_KEY_BASE58").expect("❌ PRIVATE_KEY_BASE58 not found in .env!");

    let vanity_prefix = env::var("VANITY_PREFIX").expect("❌ VANITY_PREFIX  not found in .env!");

    let vanity_output =
        env::var("VANITY_OUTPUT_FILE").expect("❌ VANITY_OUTPUT_FILE not found in .env!");

    let input_json = env::var("INPUT_JSON_FILE").expect("❌ INPUT_JSON_FILE not found in .env!");

    println!("\n--- 1️⃣  Base58 → JSON ---");
    match base58_to_json(&private_key, &input_json) {
        Ok(_json) => {
            println!("✅ {} saved!", input_json);
            // Extract and show public key
            if let Ok(content) = fs::read_to_string(&input_json) {
                if let Ok(bytes) = extract_public_key_from_json(&content) {
                    let public_key_base58 = bs58::encode(&bytes).into_string();
                    println!("🔐 Public Key (Base58): {}", public_key_base58);
                }
            }
        }
        Err(e) => eprintln!("❌ {}", e),
    }

    println!("\n--- 2️⃣  JSON → Base58 ---");
    match json_to_base58(&input_json) {
        Ok(base58) => {
            println!("🔑 Base58 Private Key: {}", base58);
            // Extract and show public key from JSON
            if let Ok(content) = fs::read_to_string(&input_json) {
                if let Ok(bytes) = extract_public_key_from_json(&content) {
                    let public_key_base58 = bs58::encode(&bytes).into_string();
                    println!("🔐 Public Key (Base58): {}", public_key_base58);
                }
            }
        }
        Err(e) => eprintln!("❌ {}", e),
    }

    println!("\n--- 3️⃣  Vanity Address Search ---");
    search_key(&vanity_prefix, &vanity_output);
}

/// Extract public key from JSON keypair (last 32 bytes)
fn extract_public_key_from_json(json: &str) -> Result<Vec<u8>, String> {
    let bytes: Vec<u8> =
        serde_json::from_str(json).map_err(|_| "Invalid JSON format".to_string())?;

    if bytes.len() != 64 {
        return Err("Invalid keypair length".to_string());
    }

    Ok(bytes[32..64].to_vec())
}
