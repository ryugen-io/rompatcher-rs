# Architecture Guide

## Overview

`rom-patcher-rs` is a modular, high-performance ROM patcher written in Rust 2024 Edition (minimum version 1.91). The project follows strict modularity principles with clear separation of concerns.

**Current Status:**
- IPS format: Fully implemented (apply, validate, metadata)
- CLI: Simplified interface with positional arguments
- Validation: CRC32 checksums always shown
- Tests: 17 tests passing (14 IPS + 3 validation)
- Benchmarks: 16µs for 1MB ROM patching
- Other formats: BPS, UPS, APS, RUP, PPF, xdelta (stubs ready)

**Performance Characteristics:**
- Apply 1KB patch: ~39ns
- Apply 1MB patch: ~16µs
- Validate patch: ~18ns (constant time)

## Project Structure

```
rom-patcher-rs/ (1850 total lines)
├── crates/
│   ├── cli/           # Command-line interface (195 lines)
│   │   ├── commands/  # Single apply command (148 lines)
│   │   └── utils/     # Validation utilities (21 lines)
│   ├── core/          # Core types and traits (197 lines)
│   │   ├── types.rs   # PatchType enum (85 lines)
│   │   ├── format.rs  # PatchFormat trait (59 lines - no create)
│   │   └── error.rs   # Error types (37 lines)
│   ├── formats/       # Patch format implementations (1110 lines)
│   │   ├── ips/       # IPS format - FULLY IMPLEMENTED (385 lines)
│   │   ├── bps.rs     # BPS format - stub (67 lines)
│   │   ├── ups.rs     # UPS format - stub (59 lines)
│   │   ├── aps.rs     # APS format - stub (53 lines)
│   │   ├── rup.rs     # RUP format - stub (46 lines)
│   │   ├── ppf.rs     # PPF format - stub (69 lines)
│   │   └── xdelta.rs  # xdelta format - stub (57 lines)
│   └── features/      # Optional features (348 lines)
│       ├── validation/        # CRC32, MD5, SHA (180 lines)
│       └── retroachievements/ # RA database (50 lines)
└── test_roms/         # Test patches for integration testing
    └── ips/           # IPS test files (14 passing tests)
```

## Detailed Module Breakdown

### Core Crate (209 lines)
```
core/src/
├── lib.rs (12 lines)       # Crate exports
├── types.rs (85 lines)     # PatchType enum (IPS, BPS, UPS, etc.)
├── format.rs (71 lines)    # PatchFormat trait
└── error.rs (37 lines)     # PatchError type
```

### Formats Crate (1390 lines)

**IPS Format - FULLY IMPLEMENTED (5 modules, 385 source lines)**
```
formats/src/ips/
├── apply.rs (127 lines)    # Apply patches with RLE support
├── metadata.rs (94 lines)  # Extract patch info (records, truncation)
├── validate.rs (69 lines)  # Validate patch structure
├── mod.rs (48 lines)       # Public API + PatchFormat impl
├── io.rs (15 lines)        # read_u24_be, read_u16_be helpers
└── constants.rs (13 lines) # HEADER, EOF_MARKER, MAX_ROM_SIZE

Each module has single responsibility - no monolithic files.
Purpose: APPLY patches only (no creation functionality)
Largest individual file: apply.rs (127 lines)
```

**Other Formats - Stubs (294 lines)**
```
formats/src/
├── bps.rs (67 lines)       # BeatsPatchingSystem - stub
├── ups.rs (59 lines)       # Universal Patching System - stub
├── aps.rs (53 lines)       # Advanced Patching System - stub
├── ppf.rs (69 lines)       # PlayStation Patch Format - stub
├── rup.rs (46 lines)       # Rupture Patch Format - stub
└── xdelta.rs (57 lines)    # xdelta3 format - stub
```

### Features Crate (348 lines)

**Validation Feature (180 lines)**
```
features/src/validation/
├── mod.rs (13 lines)           # Feature exports
├── types.rs (14 lines)         # HashAlgorithm enum
├── trait_def.rs (22 lines)     # ValidationFeature trait
├── validator.rs (80 lines)     # Main Validator implementation
└── algorithms/
    ├── mod.rs (3 lines)
    └── crc32.rs (51 lines)     # CRC32 hasher with lookup table
```

**RetroAchievements Feature (50 lines)**
```
features/src/retroachievements/
├── mod.rs (7 lines)        # Feature exports
└── types.rs (43 lines)     # Console enum (NES, SNES, GB, GBA, etc.)
```

### CLI Crate (195 lines)
```
cli/src/
├── main.rs (30 lines)          # Minimal entry point
├── commands/
│   ├── mod.rs (5 lines)
│   └── apply.rs (143 lines)    # Apply with transactional safety
└── utils/
    ├── mod.rs (4 lines)
    └── validation.rs (17 lines) # compute_crc32, format_crc32
```

### Tests (220 lines)
```
formats/tests/ips/
├── apply_tests.rs (100 lines)    # 6 tests
├── validate_tests.rs (38 lines)  # 5 tests
└── metadata_tests.rs (36 lines)  # 3 tests

features/tests/
└── validation_tests.rs (46 lines) # 3 tests

Total: 17 tests passing (14 IPS + 3 validation)
```

### Benchmarks (65 lines)
```
formats/benches/
└── ips_bench.rs # Apply, validate benchmarks
                 # Performance: 16µs for 1MB ROM
```

## Design Principles

### 1. Modular Architecture

**Every module must have a SINGLE, clear responsibility.**

####  Good Module Structure
```
formats/src/ips/
├── mod.rs           # Public API + trait impl (minimal)
├── constants.rs     # Format constants only
├── apply.rs         # Patch application logic
├── create.rs        # Patch creation logic
├── metadata.rs      # Metadata extraction
├── validate.rs      # Validation logic
└── io.rs            # Low-level I/O helpers
```

####  Bad Module Structure
```
formats/src/
└── ips.rs           # 300+ lines with everything mixed
```

### 2. Test Organization

**Tests MUST be in separate `tests/` directory, NOT inline with `#[cfg(test)]`.**

####  Good Test Structure
```
formats/tests/ips/
├── mod.rs
├── apply_tests.rs
├── create_tests.rs
├── metadata_tests.rs
├── validate_tests.rs
└── io_tests.rs
```

####  Bad Test Structure
```rust
// In ips.rs
#[cfg(test)]
mod tests {
    // 50+ lines of tests in the same file
}
```

### 3. Visibility Rules

- Use `pub` for public API
- Use `pub(super)` for parent module access
- Use `pub(in crate::module)` for specific module tree access
- Never expose implementation details

```rust
//  Good
pub struct IpsPatcher;  // Public API

pub(super) const EOF_MARKER: u32 = 0x454F46;  // Module internal

pub(in crate::validation) struct Crc32Hasher { }  // Validation tree only

//  Bad
pub const EOF_MARKER: u32 = 0x454F46;  // Exposes internal constant
```

### 4. Error Handling

Always use `Result<T>` from `rom_patcher_core`:

```rust
use rom_patcher_core::Result;

pub fn apply(rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
    validate_header(patch)?;
    // ...
}
```

### 5. Documentation

Every public item MUST have documentation:

```rust
/// Apply IPS patch to ROM data
///
/// # Errors
/// Returns error if:
/// - Invalid magic header
/// - Corrupted patch data
/// - Missing EOF marker
pub fn apply(rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
    // ...
}
```

## Module Guidelines

### Core Crate (`crates/core/`)

Provides foundational types and traits. Should remain minimal and stable.

**Files:**
- `types.rs` - Core type definitions
- `format.rs` - `PatchFormat` trait
- `error.rs` - Error types

### Formats Crate (`crates/formats/`)

Each patch format MUST follow this structure:

```
formats/src/{format_name}/
├── mod.rs          # Public API, trait implementation
├── constants.rs    # Format-specific constants
├── apply.rs        # Patch application
├── create.rs       # Patch creation
├── metadata.rs     # Metadata extraction
├── validate.rs     # Validation
└── io.rs          # I/O utilities (if needed)
```

**Template for mod.rs:**
```rust
//! {FORMAT} format implementation
//!
//! ## Format Specification
//! - [specification details]
//!
//! ## Limitations
//! - [any limitations]

use rom_patcher_core::{PatchFormat, Result};

mod apply;
mod constants;
mod metadata;
mod validate;

pub use constants::{/* public constants */};

pub struct {Format}Patcher;

impl PatchFormat for {Format}Patcher {
    fn can_handle(data: &[u8]) -> bool {
        apply::can_handle(data)
    }

    fn apply(&self, rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
        apply::apply(rom, patch)
    }

    fn metadata(patch: &[u8]) -> Result<PatchMetadata> {
        metadata::extract(patch)
    }

    fn validate(patch: &[u8]) -> Result<()> {
        validate::validate(patch)
    }
}
```

### Features Crate (`crates/features/`)

Optional features like validation and RetroAchievements support.

**Structure:**
```
features/src/{feature_name}/
├── mod.rs          # Public API
├── types.rs        # Type definitions
├── trait_def.rs    # Trait definitions
└── {impl}.rs       # Implementations
```

### CLI Crate (`crates/cli/`)

Minimal command-line interface with single purpose: applying patches.

```
cli/src/
├── main.rs         # Minimal entry point (30 lines)
├── commands/
│   ├── mod.rs      # Module exports (5 lines)
│   └── apply.rs    # Apply patches with validation (143 lines)
└── utils/
    ├── mod.rs      # Module exports (9 lines)
    ├── parsing.rs  # Format parsing (41 lines)
    ├── hex.rs      # Hex encoding (6 lines)
    └── validation.rs # CRC32 utilities (17 lines)
```

**CLI Interface:**
```bash
rompatch <ROM> <PATCH> [OUTPUT]
```

- No subcommands - just positional arguments
- Output defaults to `{rom_dir}/patched/{rom}.patched.{ext}`
- Always shows CRC32 checksums for validation
- Auto-detects patch format from header

## Adding a New Patch Format

1. **Create directory structure:**
```bash
mkdir -p crates/formats/src/{format}/
mkdir -p crates/formats/tests/{format}/
```

2. **Implement modules:**
   - `constants.rs` - Magic bytes, limits
   - `apply.rs` - Application logic
   - `create.rs` - Creation logic
   - `metadata.rs` - Metadata extraction
   - `validate.rs` - Validation
   - `io.rs` - I/O helpers (if needed)
   - `mod.rs` - Public API

3. **Write tests:**
   - `tests/{format}/apply_tests.rs`
   - `tests/{format}/create_tests.rs`
   - `tests/{format}/metadata_tests.rs`
   - `tests/{format}/validate_tests.rs`

4. **Update `formats/src/lib.rs`:**
```rust
#[cfg(feature = "{format}")]
pub mod {format};
```

5. **Update `formats/Cargo.toml`:**
```toml
[features]
{format} = []
```

6. **Run CI checks:**
```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

## Performance Considerations

- Use `#[inline]` for hot path functions
- Pre-allocate vectors when size is known
- Avoid unnecessary allocations
- Use slices instead of owned data where possible

```rust
//  Good
fn create(original: &[u8], modified: &[u8]) -> Result<Vec<u8>> {
    let mut patch = Vec::with_capacity(estimate_size(original, modified));
    // ...
}

//  Bad
fn create(original: &[u8], modified: &[u8]) -> Result<Vec<u8>> {
    let mut patch = Vec::new();  // Will reallocate multiple times
    // ...
}
```

## Safety

### Transactional Operations

Always clone data before modification when safety is required:

```rust
//  Safe - clone before modify
pub fn apply(rom_path: PathBuf, patch_path: PathBuf, output: PathBuf) -> Result<()> {
    let original = fs::read(&rom_path)?;
    let mut patched = original.clone();  // Clone for rollback

    patcher.apply(&mut patched, &patch)?;

    // Write to temp then atomic rename
    let temp = output.with_extension("tmp");
    fs::write(&temp, &patched)?;
    fs::rename(&temp, &output)?;

    Ok(())
}
```

## CI/CD Requirements

All code MUST pass:

1. **Format check:** `cargo fmt --check`
2. **Linting:** `cargo clippy -- -D warnings`
3. **Tests:** `cargo test`
4. **Benchmarks:** `cargo bench` (for performance-critical changes)

## Rust Version Policy

- **Minimum:** Rust 1.91
- **Edition:** 2024
- Use latest stable Rust features
- Update MSRV only when necessary

## Dependencies

Keep dependencies minimal:
- `anyhow` - Error handling
- `clap` - CLI parsing
- Format-specific dependencies only when absolutely necessary

## Summary

**Golden Rules:**
1. One responsibility per module
2. Tests in separate `tests/` directory
3. Document all public APIs
4. Follow the established structure
5. Pass all CI checks before committing
