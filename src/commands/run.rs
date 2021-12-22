use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::{ensure, Context, Result};
use clap::{AppSettings, Parser};

use super::BuildCommand;
use crate::common;
use crate::error::CrateError;
use crate::runner;

const DEFAULT_NODE_ADDRESS: &str = "ws://127.0.0.1:9944";

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
    /// The Rhai-script to be run
    script: Option<String>,
    /// The node's address and port
    #[clap(long)]
    node: Option<String>,
}

impl RunCommand {
    pub fn run(&self) -> Result<()> {
        let script_path = self.find_script()?;

        // Build the program before running
        let build: BuildCommand = self.clone().into();
        let info = build.run()?;
        let code = fs::read(&info.optimized_wasm).context("unable to read the WASM file")?;
        runner::set_code(code);

        // Set node's URL
        let url = self.node.as_deref().unwrap_or(DEFAULT_NODE_ADDRESS);
        runner::set_node_url(url.to_string());

        // Run the script
        let relative_path = script_path
            .strip_prefix(env::current_dir()?)?
            .to_string_lossy();
        common::print("Running", &format!("`{}`", relative_path));
        runner::run(script_path)
    }

    fn find_script(&self) -> Result<PathBuf> {
        if let Some(script) = &self.script {
            let path = fs::canonicalize(script)?;
            if path.exists() {
                return Ok(path);
            }
        }
        let entries = fs::read_dir(env::current_dir()?)?;
        let mut paths = Vec::new();
        for entry in entries {
            let path = entry?.path();
            if let Some(ext) = path.extension() {
                if ext.to_ascii_lowercase() == "rhai" {
                    paths.push(path);
                }
            }
        }
        ensure!(paths.len() == 1, CrateError::ScriptNotFound);
        Ok(paths.remove(0))
    }
}
