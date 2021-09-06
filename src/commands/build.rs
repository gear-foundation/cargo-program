use std::process::{self, Command};

use anyhow::Result;
use clap::Clap;

/// Compile a Gear program
#[derive(Clap, Debug)]
pub(crate) struct BuildCommand {
    /// Build artifacts in release mode, with optimizations
    #[clap(long)]
    release: bool,
}

impl BuildCommand {
    pub fn run(&self) -> Result<()> {
        // TODO: Check whether rustc has been installed and has the appropriate version (1.54+)
        // TODO: Check whether WASM target has been added
        let mut cargo_build = Command::new("cargo");
        cargo_build.arg("+nightly");
        cargo_build.arg("build");
        cargo_build.arg("--target");
        cargo_build.arg("wasm32-unknown-unknown");
        if self.release {
            cargo_build.arg("--release");
        }

        let status = cargo_build.status()?;
        if !status.success() {
            process::exit(1);
        }
        Ok(())
    }
}
