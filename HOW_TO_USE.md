# 📖 User Guide - How to Use Solana Keypair Utils

This guide shows you everything you need to know to use `solana-keypair-utils` as either a library or a command-line tool.

---

## 🎯 Quick Start (5 Minutes)

### For CLI Users (Everyone)

```bash
# Install the tool
cargo install solana-keypair-utils

# Convert Base58 to JSON
find "4rq21juJsG3H5FtYAhBqB3Uxy6SsfHGt1m3jCyCVJho7VyEGkkaYnuM1Z4dPp8rWKR7h5CHH1WJ46d1wmLByPnZM"

# Convert JSON to Base58
find '[193,5,74,223,97,127,237,64,104,28,158,222,16,135,78,160,193,161,210,241,96,194,75,112,169,141,3,233,179,164,60,209,160,145,187,56,143,179,52,181,76,101,202,86,185,49,139,174,201,26,151,97,103,175,123,107,219,17,208,227,168,34,204,152]'
```

### For Rust Developers

```bash
# Add to your project
cargo add solana-keypair-utils
```

```rust
use solana_keypair_utils::{base58_to_bytes, bytes_to_base58};

fn main() -> Result<(), String> {
    let key = "4rq21juJsG3H5FtYAhBqB3Uxy6SsfHGt1m3jCyCVJho7VyEGkkaYnuM1Z4dPp8rWKR7h5CHH1WJ46d1wmLByPnZM";
    let bytes = base58_to_bytes(key)?;
    println!("Keypair bytes: {} bytes", bytes.len());
    Ok(())
}
```

---

## 📦 Installation

### Option 1: CLI Tool Only

```bash
cargo install solana-keypair-utils
```

Now you can use `find` command from anywhere:

```bash
find "your-base58-key"
```

### Option 2: Library Only

```bash
cargo new my_keypair_app
cd my_keypair_app
cargo add solana-keypair-utils
```

### Option 3: Both (Clone from GitHub)

```bash
git clone https://github.com/mishalturkane/solana-keypair-utils.git
cd solana-keypair-utils

# Use both tools
cargo run --release --bin find -- "your-key"
cargo run --release
```

---

## 🔧 Using the `find` Command (CLI Tool)

### Basic Usage

```bash
# Get help
find
```

### Convert Base58 to JSON

```bash
find "4rq21juJsG3H5FtYAhBqB3Uxy6SsfHGt1m3jCyCVJho7VyEGkkaYnuM1Z4dPp8rWKR7h5CHH1WJ46d1wmLByPnZM"
```

**Output:**
```
✅ Base58 → JSON
📊 Bytes: 64
📄 JSON:
[193,5,74,223,97,127,237,64,104,28,158,222,16,135,78,160,193,161,210,241,96,194,75,112,169,141,3,233,179,164,60,209,160,145,187,56,143,179,52,181,76,101,202,86,185,49,139,174,201,26,151,97,103,175,123,107,219,17,208,227,168,34,204,152]

🔐 Public Key (Base58):
Boo7oAMZdTSUVMNa2VFEybxb2AiT3QRFQ7ofNjKjCFTd
```

### Convert JSON to Base58

```bash
find '[193,5,74,223,97,127,237,64,104,28,158,222,16,135,78,160,193,161,210,241,96,194,75,112,169,141,3,233,179,164,60,209,160,145,187,56,143,179,52,181,76,101,202,86,185,49,139,174,201,26,151,97,103,175,123,107,219,17,208,227,168,34,204,152]'
```

**Output:**
```
✅ JSON → Base58
🔑 Private Key (Base58):
4rq21juJsG3H5FtYAhBqB3Uxy6SsfHGt1m3jCyCVJho7VyEGkkaYnuM1Z4dPp8rWKR7h5CHH1WJ46d1wmLByPnZM

🔐 Public Key (Base58):
Boo7oAMZdTSUVMNa2VFEybxb2AiT3QRFQ7ofNjKjCFTd
```

---

## 📚 Using the Library (Rust Developers)

### Import the Library

```rust
use solana_keypair_utils::{
    base58_to_bytes,
    bytes_to_base58,
    base58_to_json,
    json_to_base58,
};
```

### Example 1: Convert Base58 to Bytes

```rust
use solana_keypair_utils::base58_to_bytes;

fn main() -> Result<(), String> {
    let private_key = "4rq21juJsG3H5FtYAhBqB3Uxy6SsfHGt1m3jCyCVJho7VyEGkkaYnuM1Z4dPp8rWKR7h5CHH1WJ46d1wmLByPnZM";
    
    let bytes = base58_to_bytes(private_key)?;
    
    println!("Private key bytes: {:?}", bytes);
    println!("Total bytes: {}", bytes.len());
    
    // First 32 bytes = private key
    // Last 32 bytes = public key
    let private_part = &bytes[0..32];
    let public_part = &bytes[32..64];
    
    println!("Private key part: {:?}", private_part);
    println!("Public key part: {:?}", public_part);
    
    Ok(())
}
```

### Example 2: Convert Bytes to Base58

```rust
use solana_keypair_utils::bytes_to_base58;

fn main() {
    let keypair_bytes = vec![
        193, 5, 74, 223, 97, 127, 237, 64, 104, 28, 158, 222,
        // ... (64 bytes total)
    ];
    
    let base58_key = bytes_to_base58(keypair_bytes);
    println!("Base58 key: {}", base58_key);
}
```

### Example 3: Work with Files

```rust
use solana_keypair_utils::{base58_to_json, json_to_base58};

fn main() -> Result<(), String> {
    // Save Base58 key as JSON file
    let private_key = "4rq21juJsG3H5FtYAhBqB3Uxy6SsfHGt1m3jCyCVJho7VyEGkkaYnuM1Z4dPp8rWKR7h5CHH1WJ46d1wmLByPnZM";
    base58_to_json(private_key, "keypair.json")?;
    
    // Load JSON file and convert back to Base58
    let restored_key = json_to_base58("keypair.json")?;
    println!("Restored key: {}", restored_key);
    
    Ok(())
}
```

### Example 4: Extract Public Key

```rust
use solana_keypair_utils::base58_to_bytes;

fn main() -> Result<(), String> {
    let private_key = "4rq21juJsG3H5FtYAhBqB3Uxy6SsfHGt1m3jCyCVJho7VyEGkkaYnuM1Z4dPp8rWKR7h5CHH1WJ46d1wmLByPnZM";
    
    let bytes = base58_to_bytes(private_key)?;
    
    // Public key is the last 32 bytes
    let public_key_bytes = &bytes[32..64];
    
    // Convert to Base58
    let public_key_base58 = bs58::encode(public_key_bytes).into_string();
    println!("Public key: {}", public_key_base58);
    
    Ok(())
}
```

---

## 🎯 Common Use Cases

### Use Case 1: Export Phantom Wallet

**Step 1:** Open Phantom wallet → Settings → Trusted Apps → Copy Private Key

**Step 2:** Run the converter

```bash
find "paste-your-copied-key-here"
```

**Step 3:** Use with Solana CLI

```bash
# Copy the JSON output
find "your-key" > keypair.json

# Use with Solana
solana config set --keypair keypair.json
solana balance
```

---

### Use Case 2: Backup Your Solana Keypair

**Step 1:** Get your Solana keypair

```bash
cat ~/.config/solana/id.json
```

**Step 2:** Convert to Base58 for backup

```bash
find '[193,5,74,...]' > backup.txt
```

**Step 3:** Store safely (encrypted drive, password manager, etc.)

---

### Use Case 3: Generate Multiple Vanity Addresses

Create a script `gen_vanity.sh`:

```bash
#!/bin/bash

prefixes=("sol" "dev" "magic" "fire")

for prefix in "${prefixes[@]}"; do
    echo "Generating vanity address with prefix: $prefix"
    
    # Create .env
    cat > .env << EOF
PRIVATE_KEY_BASE58=dummy
INPUT_JSON_FILE=keypair_$prefix.json
VANITY_OUTPUT_FILE=vanity_$prefix.json
VANITY_PREFIX=$prefix
EOF
    
    # Run generator
    cargo run --release
    
    echo "✅ Saved to vanity_$prefix.json"
    echo ""
done
```

Run it:

```bash
chmod +x gen_vanity.sh
./gen_vanity.sh
```

---

### Use Case 4: Batch Convert Keys

Create `convert_keys.sh`:

```bash
#!/bin/bash

# File with one Base58 key per line
input_file="keys.txt"
output_file="converted_keys.txt"

> "$output_file"  # Clear file

while IFS= read -r key; do
    echo "Converting: $key"
    find "$key" >> "$output_file"
    echo "---" >> "$output_file"
done < "$input_file"

echo "✅ All keys converted to $output_file"
```

Use it:

```bash
# Create keys.txt with Base58 keys
echo "key1..." > keys.txt
echo "key2..." >> keys.txt

# Convert all
./convert_keys.sh
```

---

## 📊 API Reference

### Core Functions

```rust
/// Convert Base58 string to raw bytes
pub fn base58_to_bytes(base58_str: &str) -> Result<Vec<u8>, String>

/// Convert raw bytes to Base58 string
pub fn bytes_to_base58(bytes: Vec<u8>) -> String

/// Save Base58 keypair as JSON file
pub fn base58_to_json(key: &str, filename: &str) -> Result<String, String>

/// Load JSON keypair file and convert to Base58
pub fn json_to_base58(filename: &str) -> Result<String, String>

/// Convert Base58 to raw bytes (alias)
pub fn base58_to_json_bytes(key: &str) -> Result<Vec<u8>, String>
```

### Return Types

All functions return `Result<T, String>` for proper error handling:

```rust
match base58_to_bytes(key) {
    Ok(bytes) => println!("Success: {} bytes", bytes.len()),
    Err(e) => eprintln!("Error: {}", e),
}
```

---

## 🔐 Security Best Practices

### ✅ Do This:
- ✅ Keep private keys in secure storage
- ✅ Use different keys for testnet vs mainnet
- ✅ Never commit .env files with real keys
- ✅ Store Base58 keys encrypted
- ✅ Validate addresses before sending funds

### ❌ Don't Do This:
- ❌ Share private keys in chat/email
- ❌ Upload to public repositories
- ❌ Store in plain text files
- ❌ Use on shared computers
- ❌ Paste keys in untrusted tools

---

## 🐛 Troubleshooting

### Error: "find: command not found"

**Solution:** Install the binary

```bash
cargo install solana-keypair-utils
```

### Error: "Invalid base58 private key"

**Causes:**
- Key has spaces or special characters
- Key is corrupted or incomplete
- Key is not valid Solana keypair

**Solution:** 
- Copy key directly from wallet
- Don't modify the key
- Use 88-character Base58 keys

### Error: "Invalid JSON format"

**Causes:**
- Missing square brackets `[]`
- Invalid numbers (must be 0-255)
- Missing commas between numbers

**Solution:**
```bash
# ✅ Correct format
find '[193,5,74,223,...]'

# ❌ Wrong format
find '193,5,74,223'
find '{193,5,74,223}'
```

### Error: "Invalid keypair length"

**Cause:** Keypair must be exactly 64 bytes (32 private + 32 public)

**Solution:** Ensure you have complete keypair, not partial

---

## 📖 More Examples

### Working with Metaplex

```rust
use solana_keypair_utils::base58_to_bytes;
use solana_sdk::signer::Signer;

fn setup_keypair(base58_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = base58_to_bytes(base58_key)?;
    
    // Use with solana-sdk
    // let keypair = Keypair::from_bytes(&bytes)?;
    
    Ok(())
}
```

### Working with Anchor

```rust
use solana_keypair_utils::json_to_base58;

fn load_anchor_keypair(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Load Anchor's keypair.json
    let base58_key = json_to_base58(path)?;
    println!("Anchor keypair: {}", base58_key);
    Ok(base58_key)
}
```

---

## 🆘 Getting Help

- 📖 **README.md** - Full project documentation
- 📝 **CHANGELOG.md** - What's new
- 🐙 **GitHub Issues** - Bug reports & feature requests
- 💬 **GitHub Discussions** - Questions & help

---

## 📍 Links

- **GitHub**: https://github.com/mishalturkane/solana-keypair-utils
- **Crates.io**: https://crates.io/crates/solana-keypair-utils
- **Docs.rs**: https://docs.rs/solana-keypair-utils
- **Issues**: https://github.com/mishalturkane/solana-keypair-utils/issues

---

**Happy Keypair Managing! 🔐**
