use std::env;
use std::path::PathBuf;

use anyhow::Result;
use clap::{AppSettings, Clap};
use colored::Colorize;

use crate::error::CrateError;

use super::BuildCommand;

/// Run a Gear program off-chain or using local node
#[derive(Clap, Clone, Debug)]
#[clap(global_setting=AppSettings::DisableVersionFlag)]
pub(crate) struct RunCommand {
    /// Build artifacts in release mode, with optimizations
    #[clap(long)]
    pub(crate) release: bool,
    /// Path to Cargo.toml
    #[clap(long, default_value = "Cargo.toml")]
    pub(crate) manifest_path: PathBuf,
    /// Use `wasmtime` runtime for running WASM code (default)
    #[clap(long)]
    wasmtime: bool,
}

impl RunCommand {
    pub fn run(&self) -> Result<()> {
        let build: BuildCommand = self.clone().into();
        let info = build.run()?;
        let relative_path = info.optimized_wasm.strip_prefix(env::current_dir()?)?;
        println!(
            "{:>12} `{}`",
            "Running".green().bold(),
            relative_path.to_string_lossy(),
        );
        Err(CrateError::UnimplementedCommand.into())
    }
}
