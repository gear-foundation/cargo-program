use anyhow::Result;

use crate::error::CrateError;

pub fn run_program(_code: Vec<u8>) -> Result<()> {
    Err(CrateError::UnimplementedCommand.into())
}
