use anyhow::Result;
use clap::{AppSettings, Parser};

use crate::error::CrateError;

/// Execute unit and integration tests
#[derive(Debug, Parser)]
#[clap(global_setting=AppSettings::DisableVersionFlag)]
pub(crate) struct TestCommand {
    /// Build artifacts in release mode, with optimizations
    #[clap(long)]
    release: bool,
}

impl TestCommand {
    pub fn run(&self) -> Result<()> {
        Err(CrateError::UnimplementedCommand.into())
    }
}
