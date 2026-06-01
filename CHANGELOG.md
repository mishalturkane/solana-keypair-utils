# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2024-06-01

### Added
- **Library Distribution**: Project now available as a Rust library via `cargo add solana-keypair-utils`
- **Smart CLI Tool (`find`)**: New command-line tool with auto-detection of Base58 or JSON format
- **Public Key Display**: Both CLI and library now show extracted public keys in Base58 format
- **Proper Error Handling**: All functions now return `Result<T, String>` instead of panicking
- **Comprehensive Documentation**: 
  - README.md with full usage guide
  - USAGE.md with examples and API reference
  - PUBLISHING.md with publishing instructions
  - QUICK_REFERENCE.md for quick lookups
- **Library API Functions**:
  - `base58_to_bytes()` - Convert Base58 to raw bytes
  - `bytes_to_base58()` - Convert bytes to Base58
  - `base58_to_json()` - Save keypair as JSON file
  - `json_to_base58()` - Load and convert JSON keypair
  - `base58_to_json_bytes()` - Get bytes from Base58 without file I/O

### Changed
- **Version**: 0.1.0 → 0.2.0
- **Edition**: 2024 → 2021 (stable Rust)
- **Error Handling**: Replaced `expect()` calls with `Result` types
- **Output**: Now displays both private and public keys in Base58 format

### Fixed
- Error handling in keypair conversion functions
- Validation of keypair length (must be 64 bytes)

## [0.1.0] - Initial Release

### Features
- Base58 ↔ JSON keypair conversion
- Vanity address generation with custom prefixes
- Command-line interface with .env configuration
- Ed25519 keypair support
