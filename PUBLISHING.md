# 📦 Publishing Solana Keypair Utils to crates.io

## Overview

Your project is now structured as both:
- **Library Crate**: For use in other Rust projects via `cargo add solana-keypair-utils`
- **Binary/CLI Tool**: For use as a command-line tool `find`

## Project Structure

```
solana-keypair-utils/
├── src/
│   ├── lib.rs                    # Library root - exports public API
│   ├── main.rs                   # Binary for full features (env-based)
│   └── bin/
│       └── find.rs              # CLI binary - auto-detection converter
├── Cargo.toml                    # Package metadata
├── README.md                     # Project documentation
├── PUBLISHING.md                 # This file
└── .env.example                  # Environment template
```

## Step 1: Setup Your Credentials

### Create a crates.io Account
1. Go to https://crates.io
2. Click "Log in with GitHub"
3. Authorize the application

### Generate API Token
1. Go to https://crates.io/me
2. Click "API Tokens"
3. Click "New Token"
4. Copy the token

### Configure Cargo
```bash
cargo login <YOUR_API_TOKEN>
```

This creates/updates `~/.cargo/credentials.toml`

## Step 2: Verify Your Package

Before publishing, run these checks:

### Test Documentation
```bash
cargo test --doc
```

### Run Tests
```bash
cargo test --release
```

### Build Both Targets
```bash
cargo build --release --bin solana-keypair-utils
cargo build --release --bin find
```

### Check Library Documentation
```bash
cargo doc --open
```

### Lint Code
```bash
cargo clippy --all-targets --all-features
```

### Format Code
```bash
cargo fmt --check
```

## Step 3: Update Version Numbers

In `Cargo.toml`:
```toml
[package]
name = "solana-keypair-utils"
version = "0.2.0"  # Bump version
```

**Versioning Guide** (Semantic Versioning):
- `0.0.1` → `0.0.2` = Bug fixes (patch)
- `0.1.0` → `0.2.0` = New features (minor)
- `1.0.0` → `2.0.0` = Breaking changes (major)

## Step 4: Prepare for Publishing

### Clean Build
```bash
cargo clean
cargo build --release
```

### Check What Will Be Published
```bash
cargo package --allow-dirty
```

This creates a `.crate` file in `target/package/`

### Verify Package Contents
```bash
tar -tzf target/package/solana-keypair-utils-0.2.0.crate | head -20
```

## Step 5: Publish!

### Dry Run (Recommended)
```bash
cargo publish --dry-run
```

### Actual Publish
```bash
cargo publish
```

**Wait 30-60 seconds** for the crate to become available on crates.io

## Step 6: Verify Publication

### Check on crates.io
https://crates.io/crates/solana-keypair-utils

### Test Installation
```bash
# In a temporary directory
cargo add solana-keypair-utils
```

### Test Binary Installation
```bash
cargo install solana-keypair-utils --bin find
find --help
```

## Publishing Multiple Versions

### After Adding Features
1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with changes
3. Commit changes
4. Run `cargo publish`

### Example Workflow
```bash
# After making changes
git add .
git commit -m "feat: add public key extraction"
cargo publish
git tag v0.3.0
git push origin main --tags
```

## What Users Can Do After Publication

### Use as a Library
```bash
cargo new my_project
cd my_project
cargo add solana-keypair-utils
```

```rust
use solana_keypair_utils::{base58_to_bytes, bytes_to_base58};

fn main() -> Result<(), String> {
    let base58_key = "5KL9q3S1hKfxYYVMSxTXH3ztjyWnbwA6kRXr1FvW8d9m";
    let bytes = base58_to_bytes(base58_key)?;
    println!("Bytes: {:?}", bytes);
    Ok(())
}
```

### Use as a CLI Tool
```bash
# Install
cargo install solana-keypair-utils

# Use
find '[193,5,74,...]'
# Output: ✅ JSON → Base58 🔑 Private Key: 4rq21juJsG3H5FtYAhBqB3Uxy6SsfHGt1m3jCyCVJho7...
```

## Tips for Success

✅ **Do This:**
- Write clear commit messages
- Keep README updated
- Document breaking changes
- Test thoroughly before publishing
- Bump version appropriately
- Add CHANGELOG.md entries

❌ **Don't Do This:**
- Publish with uncommitted changes
- Skip tests
- Publish unstable code
- Delete published versions (they're permanent)
- Forget to update docs

## Troubleshooting

### "Name too similar to existing crate"
- Choose a unique name in Cargo.toml
- Publish under a namespace: `@username/crate-name`

### "Documentation build failed"
```bash
cargo doc --no-deps
```
Check for broken doc links

### "Package rejected"
- Check crate name (must be lowercase, alphanumeric + hyphens)
- Ensure Cargo.toml has all required fields
- Verify LICENSE file exists

### Yanking (Un-publishing) a Version
```bash
cargo yank --vers 0.2.0
```
This removes a buggy version from being downloaded

## Next Steps

1. ✅ Run `cargo test --release`
2. ✅ Run `cargo clippy --all-targets`
3. ✅ Run `cargo publish --dry-run`
4. ✅ Run `cargo publish`
5. ✅ Verify on https://crates.io/crates/solana-keypair-utils
6. ✅ Test `cargo install solana-keypair-utils --bin find`

## Useful Links

- 📚 [Publishing Guide](https://doc.rust-lang.org/cargo/publishing/)
- 🎯 [Crates.io](https://crates.io)
- 📖 [Semantic Versioning](https://semver.org)
- 🔧 [Cargo Documentation](https://doc.rust-lang.org/cargo/)

---

**Once published, users worldwide can use your crate! 🚀**
