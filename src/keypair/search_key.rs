use std::fs;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

pub fn search_key(prefix: &str, filename: &str) {
    let mut count = 0;

    println!("🔍 searching the address which starts from '{}'....", prefix);

    loop {
        let signing_key = SigningKey::generate(&mut OsRng);
        let public_key = signing_key.verifying_key();
        let address = bs58::encode(public_key.as_bytes()).into_string();

        count += 1;

        if count % 100_000 == 0 {
            println!("⏳ {} attempts...", count);
        }

        if address.starts_with(prefix) {
            println!("✅ got it!  in {} attempts", count);
            println!("📬 Address: {}", address);

            let mut keypair_bytes = [0u8; 64];
            keypair_bytes[..32].copy_from_slice(signing_key.as_bytes());
            keypair_bytes[32..].copy_from_slice(public_key.as_bytes());

            let json = serde_json::to_string(&keypair_bytes.to_vec())
                .expect("❌ JSON conversion failed!");

            fs::write(filename, &json)
                .expect("❌ File write failed!");

            println!("💾 {} saved!", filename);
            break;
        }
    }
}