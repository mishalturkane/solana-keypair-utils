use solana_keypair_utils::{base58_to_bytes, bytes_to_base58};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let input = &args[1];

    // Detect format and convert
    match detect_and_convert(input) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
}

/// Detects format (Base58 or JSON) and converts accordingly
fn detect_and_convert(input: &str) -> Result<String, String> {
    let trimmed = input.trim();

    // Try to parse as JSON array
    if trimmed.starts_with('[') && trimmed.ends_with(']') {
        return convert_json_to_base58(trimmed);
    }

    // Otherwise treat as Base58
    convert_base58_to_json(trimmed)
}

/// Converts Base58 to JSON array
fn convert_base58_to_json(base58: &str) -> Result<String, String> {
    let bytes = base58_to_bytes(base58)?;

    if bytes.len() != 64 {
        return Err(
            "Invalid keypair length. Expected 64 bytes (32 private + 32 public)".to_string(),
        );
    }

    // Extract public key (last 32 bytes)
    let public_key_bytes = &bytes[32..64];
    let public_key_base58 = bs58::encode(public_key_bytes).into_string();

    let json =
        serde_json::to_string(&bytes).map_err(|_| "Failed to serialize to JSON".to_string())?;

    Ok(format!(
        "✅ Base58 → JSON\n📊 Bytes: {}\n📄 JSON:\n{}\n\n🔐 Public Key (Base58):\n{}",
        bytes.len(),
        json,
        public_key_base58
    ))
}

/// Converts JSON array to Base58
fn convert_json_to_base58(json: &str) -> Result<String, String> {
    let bytes: Vec<u8> = serde_json::from_str(json)
        .map_err(|_| "Invalid JSON format. Expected array of numbers [0-255]".to_string())?;

    if bytes.len() != 64 {
        return Err(
            "Invalid keypair length. Expected 64 bytes (32 private + 32 public)".to_string(),
        );
    }

    let base58 = bytes_to_base58(bytes.clone());

    // Extract public key (last 32 bytes)
    let public_key_bytes = &bytes[32..64];
    let public_key_base58 = bs58::encode(public_key_bytes).into_string();

    Ok(format!(
        "✅ JSON → Base58\n🔑 Private Key (Base58):\n{}\n\n🔐 Public Key (Base58):\n{}",
        base58, public_key_base58
    ))
}

fn print_help() {
    println!();
    println!("🔐 Solana Keypair Converter - find");
    println!();
    println!("USAGE:");
    println!("    find <INPUT>");
    println!();
    println!("ARGUMENTS:");
    println!("    <INPUT>    Base58 string or JSON array");
    println!("               - If Base58: converts to JSON");
    println!("               - If JSON:   converts to Base58");
    println!();
    println!("EXAMPLES:");
    println!();
    println!("  Convert Base58 to JSON:");
    println!("    $ find 5KL9q3S1hKfxYYVMSxTXH3ztjyWnbwA6kRXr1FvW8d9m");
    println!();
    println!("  Convert JSON to Base58:");
    println!("    $ find '[193,5,74,223,97,127,...]'");
    println!();
}
