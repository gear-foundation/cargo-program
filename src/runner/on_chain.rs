use anyhow::Result;

use crate::error::CrateError;

pub fn run_program(_code: Vec<u8>, _address: &str) -> Result<()> {
    Err(CrateError::UnimplementedCommand.into())
}
