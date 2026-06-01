use std::fs;

pub fn json_to_base58(filename: &str) -> String {
    let content = fs::read_to_string(filename)
        .expect("❌ File not found!");

    let bytes: Vec<u8> = serde_json::from_str(&content)
        .expect("❌ Invalid JSON!");

    let private_key_base58 = bs58::encode(&bytes).into_string();

    println!("🔑 Base58 Private Key: {}", private_key_base58);

    private_key_base58
}