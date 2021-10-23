use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{AppSettings, Parser};
use pwasm_utils::parity_wasm::{self, elements};
use pwasm_utils::rules::{InstructionType, Metering, Set as RulesSet};

use super::BuildCommand;
use crate::common;
use crate::error::CrateError;
use crate::runner::{off_chain, on_chain};

const REGULAR_FEE: u32 = 1000;
const DEFAULT_NODE_ADDRESS: &str = "127.0.0.1:9933";

/// Run a Gear program off-chain or using local node
#[derive(Clone, Debug, Parser)]
#[clap(global_setting=AppSettings::DisableVersionFlag)]
pub(crate) struct RunCommand {
    /// Build artifacts in release mode, with optimizations
    #[clap(long)]
    pub(crate) release: bool,
    /// Path to Cargo.toml
    #[clap(long, default_value = "Cargo.toml")]
    pub(crate) manifest_path: PathBuf,
    /// Off-chain: Use `wasmer` runtime for running WASM code [default]
    #[clap(long)]
    wasmer: bool,
    /// On-chain: Local node's address and port
    #[clap(long)]
    node: Option<Option<String>>,
}

impl RunCommand {
    pub fn run(&self) -> Result<()> {
        let build: BuildCommand = self.clone().into();
        let info = build.run()?;
        let program_path = &info.optimized_wasm;
        let relative_path = program_path.strip_prefix(env::current_dir()?)?;

        common::print("Running", &format!("`{}`", relative_path.to_string_lossy()));

        if let Some(ref node) = self.node {
            let address = node
                .as_ref()
                .map(String::as_str)
                .unwrap_or(DEFAULT_NODE_ADDRESS);
            self.run_on_chain(program_path, address)
        } else {
            self.run_off_chain(program_path)
        }
    }

    fn run_off_chain(&self, path: &Path) -> Result<()> {
        let module = parity_wasm::deserialize_file(path).context("unable to read the WASM file")?;

        // We forbid memory grow and floating point
        let rules = RulesSet::new(
            REGULAR_FEE,
            vec![(InstructionType::GrowMemory, Metering::Forbidden)]
                .into_iter()
                .collect(),
        )
        .with_forbidden_floats();

        let instrumented_module = pwasm_utils::inject_gas_counter(module, &rules, "env")
            .map_err(|_| CrateError::UnableToInjectGasCounter(path.to_path_buf()))?;
        let code = elements::serialize(instrumented_module)
            .context("unable to serialize the program code")?;
        off_chain::run_program(code)
    }

    fn run_on_chain(&self, path: &Path, address: &str) -> Result<()> {
        let code = fs::read(path).context("unable to read the WASM file")?;
        on_chain::run_program(code, address)
    }
}
