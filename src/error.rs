use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrateError {
    #[error("this command hasn't been implemented yet")]
    UnimplementedCommand,

    #[error("destination `{0}` already exists")]
    DestinationExists(PathBuf),

    #[error("invalid manifest path `{0}`")]
    InvalidManifestPath(PathBuf),

    #[error("unable to find the root package in cargo metadata")]
    RootPackageNotFound,

    #[error("unable to find the library name in cargo metadata")]
    LibNameNotFound,

    #[error("unable to find the output WASM file `{0}`")]
    OutputNotFound(PathBuf),
}
