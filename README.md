# 🔐 Solana Keypair Utils - CLI Tool

A lightning-fast command-line tool for converting Solana keypairs between **Base58** and **JSON** formats instantly.

**Just run:** `find "your-key"` and get instant results! No configuration needed.

---

## ⚡ Quick Start (30 seconds)

### Install
```bash
cargo install solana-keypair-utils
```

### Use Instantly
```bash
# Convert Base58 to JSON
find "4rq21juJsG3H5FtYAhBqB3Uxy6SsfHGt1m3jCyCVJho7VyEGkkaYnuM1Z4dPp8rWKR7h5CHH1WJ46d1wmLByPnZM"

# Convert JSON to Base58
find '[193,5,74,223,97,127,237,64,104,28,158,222,16,135,78,160,193,161,210,241,96,194,75,112,169,141,3,233,179,164,60,209,160,145,187,56,143,179,52,181,76,101,202,86,185,49,139,174,201,26,151,97,103,175,123,107,219,17,208,227,168,34,204,152]'
```

**Done!** No project setup. No configuration. Just instant results. 🚀

---

## 🎯 What It Does

The `find` command is a **smart keypair converter** that:

✨ **Auto-Detects Format**
- Automatically knows if input is Base58 or JSON
- No flags needed
- Just paste and go!

🔐 **Shows Public Keys**
- Extracts and displays public key automatically
- Shows in Base58 format
- Perfect for verification

⚡ **Zero Configuration**
- No `.env` files
- No setup required
- Works immediately

🌍 **Works Everywhere**
- Use from any terminal
- Works on any machine
- No Rust project needed

---

## 💡 Real-World Examples

### Export from Phantom Wallet

```bash
# 1. Open Phantom → Settings → Copy Private Key
# 2. Run in terminal
find "paste-your-key-here"

# Output:
# ✅ Base58 → JSON
# 📊 Bytes: 64
# 📄 JSON:
# [193,5,74,223,97,127,237,64,...]
# 
# 🔐 Public Key (Base58):
# Boo7oAMZdTSUVMNa2VFEybxb2AiT3QRFQ7ofNjKjCFTd
```

### Backup Your Solana CLI Keypair

```bash
# Get your keypair
cat ~/.config/solana/id.json

# Convert to Base58 for backup
find '[193,5,74,...]' > keypair_backup.txt

# Store safely!
```

### Quick Format Check

```bash
# Need to know if a key is valid?
find "some-key"

# Instantly see if it's valid Base58 or JSON
# Plus public key verification!
```

---

## 🔧 Command Reference

### Get Help
```bash
find
```

### Convert Base58 to JSON
```bash
find "YOUR_BASE58_KEY_HERE"
```

**Output shows:**
- ✅ Direction (Base58 → JSON)
- 📊 Total bytes
- 📄 JSON array format
- 🔐 Public key in Base58

### Convert JSON to Base58
```bash
find '[193,5,74,223,97,127,...]'
```

**Output shows:**
- ✅ Direction (JSON → Base58)
- 🔑 Private key in Base58
- 🔐 Public key in Base58

---

## 📋 Input/Output Examples

### Example 1: Base58 Input
```bash
$ find "4rq21juJsG3H5FtYAhBqB3Uxy6SsfHGt1m3jCyCVJho7VyEGkkaYnuM1Z4dPp8rWKR7h5CHH1WJ46d1wmLByPnZM"

✅ Base58 → JSON
📊 Bytes: 64
📄 JSON:
[193,5,74,223,97,127,237,64,104,28,158,222,16,135,78,160,193,161,210,241,96,194,75,112,169,141,3,233,179,164,60,209,160,145,187,56,143,179,52,181,76,101,202,86,185,49,139,174,201,26,151,97,103,175,123,107,219,17,208,227,168,34,204,152]

🔐 Public Key (Base58):
Boo7oAMZdTSUVMNa2VFEybxb2AiT3QRFQ7ofNjKjCFTd
```

### Example 2: JSON Input
```bash
$ find '[193,5,74,223,97,127,237,64,104,28,158,222,16,135,78,160,193,161,210,241,96,194,75,112,169,141,3,233,179,164,60,209,160,145,187,56,143,179,52,181,76,101,202,86,185,49,139,174,201,26,151,97,103,175,123,107,219,17,208,227,168,34,204,152]'

✅ JSON → Base58
🔑 Private Key (Base58):
4rq21juJsG3H5FtYAhBqB3Uxy6SsfHGt1m3jCyCVJho7VyEGkkaYnuM1Z4dPp8rWKR7h5CHH1WJ46d1wmLByPnZM

🔐 Public Key (Base58):
Boo7oAMZdTSUVMNa2VFEybxb2AiT3QRFQ7ofNjKjCFTd
```

---

## 🎯 Use Cases

### For Solana Users
- Export wallet keys
- Backup keypairs
- Verify public addresses
- Convert between formats quickly

### For Developers
- Quick keypair validation
- Format conversion in scripts
- Extract public keys instantly
- Backup management

### For DevOps
- Integrate in automation scripts
- Batch convert keypairs
- Validate keypair files
- Export in different formats

---

## 🔐 Security Notes

✅ **All local** - No network calls
✅ **No storage** - Doesn't save anything
✅ **Instant** - Processes in milliseconds
✅ **Safe** - Open source, fully auditable
✅ **Private** - Your keys stay on your machine

---

## 🆘 Troubleshooting

### "find: command not found"
```bash
cargo install solana-keypair-utils
```

### "Invalid base58 private key"
- Check key has no spaces
- Use 88-character keys
- Copy directly from wallet
- Don't modify the key

### "Invalid JSON format"
Use proper JSON array format:
```bash
✅ Correct:   find '[193,5,74,...]'
❌ Wrong:     find '193,5,74,...'
❌ Wrong:     find '{193,5,74}'
```

### "Invalid keypair length"
Keypair must be 64 bytes (32 private + 32 public)
- Check you have complete keypair
- Not partial key

---

## 💻 Installation Options

### Option 1: From crates.io (Recommended)
```bash
cargo install solana-keypair-utils
```

### Option 2: From GitHub
```bash
cargo install --git https://github.com/mishalturkane/solana-keypair-utils.git
```

### Option 3: From Source
```bash
git clone https://github.com/mishalturkane/solana-keypair-utils.git
cd solana-keypair-utils
cargo install --path .
```

---

## 📚 Also Included: Rust Library

If you're a Rust developer, you can also use this as a library:

```bash
cargo add solana-keypair-utils
```

```rust
use solana_keypair_utils::base58_to_bytes;

fn main() -> Result<(), String> {
    let bytes = base58_to_bytes("your-key")?;
    println!("Bytes: {:?}", bytes);
    Ok(())
}
```

See **HOW_TO_USE.md** for library details.

---

## 🚀 Why Use This?

### ⚡ **Instant**
- Single command
- No setup
- Immediate results
- Millisecond conversion

### 🎯 **Smart**
- Auto-detects format
- Shows public key
- Validates input
- Clear error messages

### 🔒 **Secure**
- Local processing only
- No network calls
- Open source
- Fully auditable

### 📦 **Reliable**
- Battle-tested
- MIT licensed
- Well documented
- Active maintenance

---

## 📖 Full Documentation

| Document | Purpose |
|----------|---------|
| **README.md** | This page - CLI tool guide |
| **HOW_TO_USE.md** | Complete usage guide |
| **CHANGELOG.md** | What's new |
| **PUBLISH_GUIDE.md** | Publishing updates |

---

## 🤝 Contributing

Found a bug? Have a suggestion?

- 🐛 **Report issues**: https://github.com/mishalturkane/solana-keypair-utils/issues
- 💡 **Suggest features**: Open an issue
- 🔧 **Submit PRs**: Pull requests welcome!

---

## 📋 Version Info

- **Version**: 0.2.0
- **License**: MIT
- **Repository**: https://github.com/mishalturkane/solana-keypair-utils
- **Crates.io**: https://crates.io/crates/solana-keypair-utils

---

## ✨ Features

✅ Base58 ↔ JSON conversion
✅ Auto-format detection
✅ Public key extraction
✅ Zero configuration
✅ Works everywhere
✅ Lightning fast
✅ MIT licensed
✅ Open source
✅ Well documented

---

## 🎊 Get Started Now!

```bash
# Install (one-time)
cargo install solana-keypair-utils

# Use (anytime, anywhere)
find "your-base58-key"

# That's it! 🚀
```

**No project setup. No configuration. Just instant keypair conversion!**

---

## 📞 Quick Links

- 🌐 **GitHub**: https://github.com/mishalturkane/solana-keypair-utils
- 📦 **Crates.io**: https://crates.io/crates/solana-keypair-utils
- 📖 **Docs**: https://docs.rs/solana-keypair-utils
- 💬 **Issues**: https://github.com/mishalturkane/solana-keypair-utils/issues

---

**Made for Solana users, developers, and DevOps engineers.** 🔐

**Questions?** Check **HOW_TO_USE.md** for detailed examples and troubleshooting.
