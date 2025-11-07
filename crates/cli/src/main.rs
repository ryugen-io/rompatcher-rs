//! ROM Patcher CLI
//!
//! A minimal CLI for applying ROM patches with automatic validation.

use anyhow::Result;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

mod commands;
mod utils;

use rom_patcher_cli::OnlyMode as OnlyModeLib;

/// Operation mode for --only flag
#[derive(ValueEnum, Clone, Debug)]
enum OnlyMode {
    /// Only verify checksums without applying patch
    Verify,
}

impl From<OnlyMode> for OnlyModeLib {
    fn from(mode: OnlyMode) -> Self {
        match mode {
            OnlyMode::Verify => OnlyModeLib::Verify,
        }
    }
}

/// ROM Patcher - Apply patches to ROM files
#[derive(Parser, Debug)]
#[command(name = "rompatchrs")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the ROM file
    rom: PathBuf,

    /// Path to the patch file
    patch: PathBuf,

    /// Output path (optional, defaults to {rom_dir}/patched/{rom}.patched.{ext})
    output: Option<PathBuf>,

    /// Verify source/target checksums (slower, safer)
    #[arg(long)]
    verify: bool,

    /// Only perform specific operation without applying patch
    #[arg(long, value_enum)]
    only: Option<OnlyMode>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let only_mode = cli.only.map(|m| m.into());
    commands::apply::execute(cli.rom, cli.patch, cli.output, cli.verify, only_mode)
}
