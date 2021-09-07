use anyhow::Result;
use clap::{AppSettings, Clap};

use crate::error::CrateError;

/// Run a Gear program off-chain or using local node
#[derive(Clap, Debug)]
#[clap(global_setting=AppSettings::DisableVersionFlag)]
pub(crate) struct RunCommand {
    /// Build artifacts in release mode, with optimizations
    #[clap(long)]
    release: bool,
}

impl RunCommand {
    pub fn run(&self) -> Result<()> {
        Err(CrateError::UnimplementedCommand.into())
    }
}
