use std::fs;

pub fn base58_to_json(private_key_base58: &str, filename: &str) {
    let keypair_bytes = bs58::decode(private_key_base58)
        .into_vec()
        .expect("❌ Invalid base58 private key!");

    println!("🔍 Total bytes: {}", keypair_bytes.len());

    let json = serde_json::to_string(&keypair_bytes)
        .expect("❌ JSON conversion failed!");

    fs::write(filename, &json)
        .expect("❌ File write failed!");

    println!("✅ {} saved!", filename);
}