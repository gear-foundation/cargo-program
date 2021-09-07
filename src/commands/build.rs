use anyhow::Result;
use clap::{AppSettings, Clap};

use crate::common;

/// Compile a Gear program
#[derive(Clap, Debug)]
#[clap(global_setting=AppSettings::DisableVersionFlag)]
pub(crate) struct BuildCommand {
    /// Build artifacts in release mode, with optimizations
    #[clap(long)]
    release: bool,
}

impl BuildCommand {
    pub fn run(&self) -> Result<()> {
        // TODO: Check whether rustc has been installed and has the appropriate version (1.54+)
        // TODO: Check whether WASM target has been added
        let mut args = vec!["build", "--target", "wasm32-unknown-unknown"];
        if self.release {
            args.push("--release");
        }
        common::run_cargo(args)
    }
}
