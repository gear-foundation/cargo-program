use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrateError {
    #[error("this command hasn't been implemented yet")]
    UnimplementedCommand,

    #[error("destination `{0}` already exists")]
    DestinationExists(PathBuf),
}
