use std::path::PathBuf;

use anyhow::Result;
use clap::{AppSettings, Clap};
use colored::Colorize;

use crate::common;
use crate::crate_info::CrateInfo;

/// Compile a Gear program
#[derive(Clap, Debug)]
#[clap(global_setting=AppSettings::DisableVersionFlag)]
pub(crate) struct BuildCommand {
    /// Path to the manifest file (`Cargo.toml`)
    #[clap(long, default_value = "Cargo.toml")]
    pub(crate) manifest_path: PathBuf,
    /// Build artifacts in release mode, with optimizations
    #[clap(long)]
    pub(crate) release: bool,
}

impl BuildCommand {
    pub fn run(&self) -> Result<()> {
        // TODO: Check whether rustc has been installed and has the appropriate version (1.54+)
        // TODO: Check whether WASM target has been added
        let mut args = vec![
            "build",
            "--target",
            "wasm32-unknown-unknown",
            "--manifest-path",
        ];
        let manifest_path = self.manifest_path.to_string_lossy();
        args.push(&manifest_path);
        if self.release {
            args.push("--release");
        }
        common::run_cargo(args)?;

        let info = CrateInfo::from_command(self)?;
        println!(
            "{} {}",
            "      Output".green().bold(),
            info.output_wasm.to_string_lossy()
        );

        Ok(())
    }
}
