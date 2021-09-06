use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrateError {
    #[error("destination `{0}` already exists")]
    DestinationExists(PathBuf),
}
