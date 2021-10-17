use anyhow::Result;
use clap::{AppSettings, Parser};

use crate::error::CrateError;

/// Login to the Gear backend
#[derive(Debug, Parser)]
#[clap(global_setting=AppSettings::DisableVersionFlag)]
pub(crate) struct LoginCommand {
    /// User's token
    token: String,
}

impl LoginCommand {
    pub fn run(&self) -> Result<()> {
        Err(CrateError::UnimplementedCommand.into())
    }
}
