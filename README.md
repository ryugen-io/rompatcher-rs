# ROM Patcher RS

A modern, modular ROM patcher written in Rust supporting multiple patch formats.

**Current Status:** v0.1.6 | 2,784 LOC | 36 Tests | Binary: 564KB

## Supported Formats

- **IPS** (International Patching System) - Implemented
- **BPS** (Beat Patching System) - Implemented (v0.1.6)
- **UPS** (Universal Patching System) - Planned
- **APS** (Nintendo 64 APS Format) - Planned
- **RUP** (Rupture Patches) - Planned
- **PPF** (PlayStation Patch Format) - Planned
- **xdelta** (Generic binary diff) - Planned

## Features

### Implemented
- Apply patches to ROMs (IPS, BPS)
- Automatic format detection
- Patch validation with CRC32 checksums
- CRC32 and MD5 hash computation
- RetroAchievements integration (console detection, hash verification)
- Support for 10+ console types (GB, GBA, SNES, NES, N64, Genesis, etc.)

### Planned
- UPS, APS, RUP, PPF, xdelta format support
- SHA-1, SHA-256 checksums
- Additional output options and verbosity controls

## Architecture

The project is organized as a Cargo workspace with 4 crates:

```
rom-patcher-rs/
├── crates/
│   ├── core/           # Core traits and types
│   ├── formats/        # Patch format implementations
│   ├── features/       # Extended features (validation, hash checking)
│   └── cli/            # Command-line interface
```

### Design Principles

1. **Modular** - Each format and feature is independently implemented
2. **Extensible** - Easy to add new formats via trait implementation
3. **Type-safe** - Leverages Rust's type system for safety
4. **Zero-copy** - Efficient memory usage with slice references
5. **Performance** - SIMD-optimized CRC32 (crc32fast), ~16µs per 1MB ROM
6. **Clean Code** - No file exceeds 200 lines, subdirectory organization

## Building

Requires Rust 1.91+ with 2024 edition support:

```bash
cargo build --release
```

The binary will be at `target/release/rompatchrs`.

## Usage

### Apply a patch

```bash
# Basic usage (auto-generates output path)
rompatchrs game.smc hack.ips

# Specify output path
rompatchrs game.smc hack.ips game-patched.smc
```

The patcher automatically detects the patch format (IPS, BPS) and applies it.

## Development

### Prerequisites

Install development tools:

```bash
cargo install just cargo-watch cargo-audit cargo-outdated cargo-tarpaulin
```

### Common Tasks

```bash
just              # Show all available commands
just build        # Build in release mode
just test         # Run all tests
just clippy       # Run linter (warnings as errors)
just fmt          # Format code
just bench        # Run benchmarks
just ci           # Run all CI checks
just doc          # Generate and open documentation
```

### Adding a New Format

1. Create a new module in `crates/formats/src/`
2. Implement the `PatchFormat` trait
3. Add format detection in `detect_format()`
4. Add CLI support in `crates/cli/src/main.rs`
5. Add tests and benchmarks

### Testing

```bash
just test                 # Run all tests
just watch-test           # Watch and run tests on changes
cargo test --all-features # Direct cargo command
```

## Documentation

- [CHANGELOG.md](CHANGELOG.md) - Version history and release notes
- [ARCHITECTURE.md](ARCHITECTURE.md) - Detailed design documentation
- [docs/API.md](docs/API.md) - Complete API reference
- [docs/CLI_USAGE.md](docs/CLI_USAGE.md) - CLI usage guide
- [examples/](examples/) - Code examples

## License

MIT OR Apache-2.0

## Performance

- IPS apply: ~16 µs for 1MB ROM
- BPS validation: 3x CRC32 checksums with hardware acceleration
- Binary size: 564KB (optimized with LTO + strip)
- Zero runtime dependencies (static linking)

## Project Stats

- **Version:** 0.1.6
- **Lines of Code:** 2,784
- **Test Coverage:** 36 tests
- **Largest File:** 149 lines (all files under 200 lines)
- **Build Time:** ~3s (release with LTO)
