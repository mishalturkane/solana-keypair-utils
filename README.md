# 🔐 Solana Keypair Utils

A Rust utility tool for managing Solana keypairs with ease. Convert between formats, extract public keys, and generate vanity addresses with custom prefixes.

## ✨ Features

- **🔄 Base58 ↔ JSON Conversion**: Seamlessly convert between Base58 encoded private keys and JSON keypair arrays
- **🔑 Public Key Extraction**: Extract and display public keys from keypairs in multiple formats
- **💎 Vanity Address Generator**: Generate keypairs with custom address prefixes (e.g., addresses starting with "Sol")
- **📦 Multiple Format Support**: Work with Base58, JSON, and raw byte representations

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+
- Cargo

### Installation

Clone the repository:

```bash
git clone https://github.com/mishalturkane/solana-keypair-utils.git
cd solana-keypair-utils
```

Build the project:

```bash
cargo build --release
```

## 🛠️ Usage

### Setup Environment Variables

Create a `.env` file in the project root:

```bash
# Your Base58 encoded Solana private key
PRIVATE_KEY_BASE58=<your-base58-private-key>

# Input/Output files
INPUT_JSON_FILE=id.json
VANITY_OUTPUT_FILE=vanity_keypair.json

# Vanity address prefix to search for
VANITY_PREFIX=Sol
```

### Run the Tool

```bash
cargo run --release
```

This will execute all three operations in sequence.

## 📚 Operations

### 1️⃣ Base58 → JSON Conversion

Converts a Base58 encoded private key to a JSON keypair array format.

**What it does:**
- Decodes Base58 private key
- Stores keypair as JSON array (64 bytes: 32 private + 32 public)
- Saves to specified file

**Output:**
```json
[193, 5, 74, 223, 97, 127, ...]
```

### 2️⃣ JSON → Base58 Conversion

Converts a JSON keypair array back to Base58 format for easy sharing/storage.

**What it does:**
- Reads JSON keypair array
- Encodes to Base58 format
- Prints the Base58 private key

**Output:**
```
🔑 Base58 Private Key: 5KL9q...
```

### 3️⃣ Vanity Address Search

Generates keypairs until it finds one with a custom address prefix.

**What it does:**
- Generates random Ed25519 keypairs
- Checks if the public key (Base58 encoded) starts with your prefix
- Saves the matching keypair to a JSON file
- Shows attempt count

**Example:**
```
🔍 searching the address which starts from 'Sol'....
⏳ 100000 attempts...
⏳ 200000 attempts...
✅ got it! in 523421 attempts
📬 Address: SolXyz123...
💾 vanity_keypair.json saved!
```

## 🔒 Security Considerations

- **Never commit `.env` files** with real private keys to version control
- **Keep private keys offline** when not in use
- **Use devnet keys** for testing and development
- **Validate addresses** before transferring funds
- All operations are local - no keys are sent to external servers

## 📦 Dependencies

```toml
bs58 = "0.5"              # Base58 encoding/decoding
serde_json = "1.0"        # JSON serialization
ed25519-dalek = "2.0"     # Ed25519 signature algorithm
rand = "0.8"              # Cryptographically secure randomness
dotenv = "0.15"           # Environment variable management
```

## 🎯 Common Use Cases

### Generate a Devnet Keypair with Custom Prefix

```bash
# Set VANITY_PREFIX=dev in .env
cargo run --release
# Wait for the vanity generator to find a matching keypair
```

### Convert Your Phantom Wallet Export

```bash
# Export private key from Phantom
# Set PRIVATE_KEY_BASE58 in .env
cargo run --release
# Your keypair will be saved as JSON
```

### Extract Public Address from Keypair

```bash
# Set INPUT_JSON_FILE to your keypair JSON
cargo run --release
# Check the output - the public address is printed
```

## 📊 Project Structure

```
solana-keypair-utils/
├── src/
│   ├── main.rs                          # Entry point
│   └── keypair/
│       ├── mod.rs                       # Module definitions
│       ├── base58_to_json.rs           # Base58 → JSON conversion
│       ├── json_to_base58.rs           # JSON → Base58 conversion
│       └── search_key.rs               # Vanity address generator
├── Cargo.toml                          # Project configuration
├── .env.example                        # Environment template
├── README.md                           # This file
└── id.json                             # Example keypair JSON
```

## 💡 Examples

### Example 1: Working with Your Solana CLI Keypair

```bash
# Your Solana keypair at ~/.config/solana/id.json is already in JSON format
# Convert it to Base58 for backup:
cp ~/.config/solana/id.json ./INPUT_JSON_FILE
PRIVATE_KEY_BASE58="" cargo run --release
```

### Example 2: Generate Multiple Vanity Addresses

Simply run the tool multiple times with different `VANITY_PREFIX` values:

```bash
# Search for "Sol" prefix
VANITY_PREFIX=Sol cargo run --release

# Search for "dev" prefix
VANITY_PREFIX=dev cargo run --release
```

### Example 3: Batch Conversion

Store your private keys in Base58 format and convert them all to JSON:

```bash
# Update .env with each Base58 key and run
cargo run --release
```

## ⚡ Performance Notes

- **Vanity address generation**: Average attempts = `58^n` where `n` is prefix length
  - 3-char prefix: ~200,000 attempts (seconds)
  - 4-char prefix: ~12,000,000 attempts (minutes)
  - 5-char prefix: ~700,000,000 attempts (hours+)

## 🐛 Troubleshooting

### "PRIVATE_KEY_BASE58 not found in .env"
Make sure your `.env` file exists and contains all required variables.

### "Invalid base58 private key"
Check that your Base58 string is valid and not corrupted. Copy directly from your wallet export.

### "Invalid JSON"
Ensure your JSON file contains a valid array of numbers (0-255).

### Vanity search taking too long
Longer prefixes take exponentially longer. Try shorter prefixes or run with `--release` for optimized binary.

## 📝 License

MIT License - feel free to use this tool in your projects!

## 🤝 Contributing

Contributions are welcome! Feel free to:
- Report bugs
- Suggest features
- Submit pull requests

## 📮 Support

For issues and questions:
- 🐛 Open an issue on [GitHub Issues](https://github.com/mishalturkane/solana-keypair-utils/issues)
- 💬 Reach out with questions

---

**Made with ❤️ for the Solana ecosystem**
