use std::env;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{AppSettings, Clap};
use colored::Colorize;
use pwasm_utils::parity_wasm::{self, elements};
use pwasm_utils::rules::{InstructionType, Metering, Set as RulesSet};

use super::BuildCommand;
use crate::error::CrateError;

const REGULAR_FEE: u32 = 1000;

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
    /// Use `wasmer` runtime for running WASM code off-chain (default)
    #[clap(long)]
    wasmer: bool,
    /// Local node's address and port
    #[clap(long, default_value = "127.0.0.1:9944")]
    node: String,
}

impl RunCommand {
    pub fn run(&self) -> Result<()> {
        let build: BuildCommand = self.clone().into();
        let info = build.run()?;
        let program_path = &info.optimized_wasm;
        let relative_path = program_path.strip_prefix(env::current_dir()?)?;
        println!(
            "{:>12} `{}`",
            "Running".green().bold(),
            relative_path.to_string_lossy(),
        );
        let module = parity_wasm::deserialize_file(program_path)
            .context("unable to read the optimized WASM")?;

        // We forbid memory grow and floating point
        let rules = RulesSet::new(
            REGULAR_FEE,
            vec![(InstructionType::GrowMemory, Metering::Forbidden)]
                .into_iter()
                .collect(),
        )
        .with_forbidden_floats();

        let instrumented_module = pwasm_utils::inject_gas_counter(module, &rules, "env")
            .map_err(|_| CrateError::UnableToInjectGasCounter(program_path.clone()))?;
        let code = elements::serialize(instrumented_module)
            .context("unable to serialize the program code")?;
        self.run_off_chain(code)
    }

    fn run_off_chain(&self, _code: Vec<u8>) -> Result<()> {
        Err(CrateError::UnimplementedCommand.into())
    }
}
