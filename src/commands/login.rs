use anyhow::Result;
use clap::{AppSettings, Clap};

use crate::error::CrateError;

/// Login to the Gear backend
#[derive(Clap, Debug)]
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
