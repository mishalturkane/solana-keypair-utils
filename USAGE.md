# 🚀 Quick Start Guide for Users

## For Library Users (Rust Developers)

### Installation
```bash
cargo add solana-keypair-utils
```

### Basic Usage

#### Convert Base58 to Bytes
```rust
use solana_keypair_utils::base58_to_bytes;

fn main() -> Result<(), String> {
    let private_key = "5KL9q3S1hKfxYYVMSxTXH3ztjyWnbwA6kRXr1FvW8d9m";
    let bytes = base58_to_bytes(private_key)?;
    
    println!("Keypair bytes: {:?}", bytes);
    println!("Total bytes: {}", bytes.len());
    Ok(())
}
```

#### Convert Bytes to Base58
```rust
use solana_keypair_utils::bytes_to_base58;

fn main() {
    let keypair_bytes = vec![193, 5, 74, 223, /* ... */];
    let base58_key = bytes_to_base58(keypair_bytes);
    println!("Base58 key: {}", base58_key);
}
```

#### File Conversion
```rust
use solana_keypair_utils::{base58_to_json, json_to_base58};

fn main() -> Result<(), String> {
    // Base58 -> JSON file
    let private_key = "5KL9q...";
    base58_to_json(private_key, "keypair.json")?;
    
    // JSON file -> Base58
    let base58_result = json_to_base58("keypair.json")?;
    println!("Restored key: {}", base58_result);
    
    Ok(())
}
```

---

## For CLI Users

### Installation
```bash
cargo install solana-keypair-utils
```

This installs two binaries:
- `find` - CLI converter tool
- `solana-keypair-utils` - Full-featured tool (requires .env)

### Using the `find` Command

#### Get Help
```bash
find
# or
find --help
```

#### Convert Base58 to JSON
```bash
find "5KL9q3S1hKfxYYVMSxTXH3ztjyWnbwA6kRXr1FvW8d9m"
```

**Output:**
```
✅ Base58 → JSON
📊 Bytes: 64
📄 JSON:
[193,5,74,223,97,127,...]
```

#### Convert JSON to Base58
```bash
find '[193,5,74,223,97,127,237,64,104,28,158,222,...]'
```

**Output:**
```
✅ JSON → Base58
🔑 Private Key:
5KL9q3S1hKfxYYVMSxTXH3ztjyWnbwA6kRXr1FvW8d9m
```

### Using the Full Tool (requires .env)

#### Setup
```bash
# Clone or create project
cd my-project

# Create .env file
cat > .env << EOF
PRIVATE_KEY_BASE58=your-base58-key-here
INPUT_JSON_FILE=keypair.json
VANITY_OUTPUT_FILE=vanity.json
VANITY_PREFIX=Sol
EOF
```

#### Run
```bash
solana-keypair-utils
```

---

## Common Workflows

### 1. Export Phantom Wallet to JSON
```bash
# In Phantom: Settings → Trusted Apps → Copy Private Key
PHANTOM_KEY="your-copied-base58-key"

# Convert using find
find "$PHANTOM_KEY" > keypair.json

# Now you have keypair.json for use with Solana CLI
solana config set --keypair keypair.json
```

### 2. Backup Your Solana CLI Keypair
```bash
# Your Solana keypair is at ~/.config/solana/id.json

# Convert to Base58 for secure backup
find "$(cat ~/.config/solana/id.json)" > backup.txt

# Store backup.txt securely (encrypted drive, password manager, etc.)
```

### 3. Generate Vanity Address
```bash
# Create .env with VANITY_PREFIX=Your3CharPrefix
cargo run --release

# Waits until it finds a keypair starting with your prefix
# Saves to VANITY_OUTPUT_FILE
```

### 4. Batch Convert Multiple Keys
```bash
#!/bin/bash

# keys.txt contains one Base58 key per line
while IFS= read -r key; do
    echo "Converting: $key"
    find "$key" >> conversions.txt
    echo "---" >> conversions.txt
done < keys.txt
```

---

## Troubleshooting

### Q: "find: command not found"
**A:** Install the binary:
```bash
cargo install solana-keypair-utils
```

### Q: "Invalid base58 private key"
**A:** Check that:
- Your Base58 key has no spaces
- It's a valid Solana private key (usually 88 characters)
- Copy directly from your wallet (don't modify)

### Q: "Invalid JSON format"
**A:** Ensure your JSON is:
- Valid array syntax: `[193,5,74,...]`
- Numbers only (0-255)
- Surrounded by square brackets `[]`

### Q: How to extract public key?
**A:** Your 64-byte JSON contains:
- First 32 bytes = private key
- Last 32 bytes = public key

```bash
# If you have: [193,5,...first32...,160,145,...last32...]
# Public key is: [160,145,...last32...]

# To encode as Base58:
find '[160,145,...last32...]'
```

---

## Security Notes ⚠️

- Never commit `.env` files with real keys
- Use different keys for testnet vs mainnet
- Keep Base58 keys in secure storage
- Treat JSON keypair files like passwords
- Always validate addresses before sending funds

---

## API Reference

### Core Functions

| Function | Input | Output | Purpose |
|----------|-------|--------|---------|
| `base58_to_bytes(s)` | Base58 string | `Result<Vec<u8>>` | Decode Base58 to bytes |
| `bytes_to_base58(v)` | `Vec<u8>` | `String` | Encode bytes to Base58 |
| `base58_to_json(s, f)` | Base58 + filename | `Result<String>` | Save JSON keypair file |
| `json_to_base58(f)` | filename | `Result<String>` | Read and decode JSON file |

See `docs.rs/solana-keypair-utils` for full API documentation.

---

**Need help?** Check the main [README.md](README.md) or open an issue on GitHub!
