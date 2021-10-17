use anyhow::Result;
use clap::{AppSettings, Parser};

use crate::error::CrateError;

/// Upload a Gear program to the chain
#[derive(Debug, Parser)]
#[clap(global_setting=AppSettings::DisableVersionFlag)]
pub(crate) struct PublishCommand {
    /// Token to use when uploading
    #[clap(long)]
    token: Option<String>,
}

impl PublishCommand {
    pub fn run(&self) -> Result<()> {
        Err(CrateError::UnimplementedCommand.into())
    }
}
