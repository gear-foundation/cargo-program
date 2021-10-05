use std::{env, fs};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{AppSettings, Clap};
use colored::Colorize;
use pwasm_utils::parity_wasm;

use crate::common;
use crate::output_info::OutputInfo;
use crate::error::CrateError;

/// Compile a Gear program
#[derive(Clap, Debug)]
#[clap(global_setting=AppSettings::DisableVersionFlag)]
pub(crate) struct BuildCommand {
    /// Build artifacts in release mode, with optimizations
    #[clap(long)]
    pub(crate) release: bool,
    /// Path to Cargo.toml
    #[clap(long, default_value = "Cargo.toml")]
    pub(crate) manifest_path: PathBuf,
}

impl BuildCommand {
    pub fn run(&self) -> Result<OutputInfo> {
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

        let info = OutputInfo::from_command(self)?;
        self.print_file_info("Output", &info.output_wasm)?;

        self.optimize(&info)?;
        self.print_file_info("Optimized", &info.optimized_wasm)?;

        self.optimize_meta(&info)?;
        self.print_file_info("Metadata", &info.metadata_wasm)?;

        Ok(info)
    }

    fn optimize(&self, info: &OutputInfo) -> Result<()> {
        let mut module = parity_wasm::deserialize_file(&info.output_wasm)
            .context("unable to read the output WASM")?;
        let exports = vec!["init", "handle", "handle_reply"];
        pwasm_utils::optimize(&mut module, exports)
            .map_err(|_| CrateError::UnableToOptimize(info.output_wasm.clone()))?;
        parity_wasm::serialize_to_file(&info.optimized_wasm, module)
            .context("unable to write the optimized WASM")
    }

    fn optimize_meta(&self, info: &OutputInfo) -> Result<()> {
        let mut module = parity_wasm::deserialize_file(&info.output_wasm)
            .context("unable to read the output WASM")?;
        let exports = vec![
            "meta_init_input",
            "meta_init_output",
            "meta_input",
            "meta_output",
            "meta_registry",
            "meta_title",
        ];
        pwasm_utils::optimize(&mut module, exports)
            .map_err(|_| CrateError::UnableToProduceMetadata(info.output_wasm.clone()))?;
        parity_wasm::serialize_to_file(&info.metadata_wasm, module)
            .context("unable to write the metadata WASM")
    }

    fn print_file_info(&self, label: &str, path: &Path) -> Result<()> {
        let metadata = fs::metadata(path).context("unable to get a file info")?;
        let size = match metadata.len() {
            bytes if bytes < 1024 => format!("{} bytes", bytes),
            to_kib if to_kib < 1024 * 1024 => format!("{} KiB", to_kib / 1024),
            to_mib => format!("{} MiB", to_mib / 1024 / 1024),
        };
        let relative_path = path.strip_prefix(env::current_dir()?)?;
        println!(
            "{:>12} `{}` ({})",
            label.green().bold(),
            relative_path.to_string_lossy(),
            size,
        );
        Ok(())
    }
}
