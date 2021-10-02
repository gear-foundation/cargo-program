use std::path::PathBuf;

use anyhow::{Context, Result};
use cargo_metadata::{Metadata, MetadataCommand, Package};

use crate::commands::BuildCommand;
use crate::error::CrateError;

pub(crate) struct CrateInfo {
    pub(crate) output_wasm: PathBuf,
}

impl CrateInfo {
    pub fn from_command(cmd: &BuildCommand) -> Result<Self> {
        anyhow::ensure!(
            cmd.manifest_path.exists(),
            CrateError::InvalidManifestPath(cmd.manifest_path.clone())
        );

        let mut meta_cmd = MetadataCommand::new();
        let metadata = meta_cmd
            .manifest_path(&cmd.manifest_path)
            .exec()
            .context("unable to invoke `cargo metadata`")?;

        let root_package = Self::root_package(&metadata).ok_or(CrateError::RootPackageNotFound)?;
        let package_name = root_package.name.replace("-", "_");
        let lib_name = root_package
            .targets
            .iter()
            .find(|target| target.kind.iter().any(|t| t == "cdylib"))
            .ok_or(CrateError::LibNameNotFound)?
            .name
            .replace("-", "_");

        let mut target_dir: PathBuf = metadata.target_directory.into();
        let manifest_path = cmd.manifest_path.canonicalize()?;
        let manifest_dir = manifest_path
            .parent()
            .ok_or_else(|| CrateError::InvalidManifestPath(manifest_path.clone()))?;
        let workspace_root = metadata.workspace_root.canonicalize()?;
        if manifest_dir != workspace_root {
            target_dir.push(&package_name);
        }
        target_dir.push("wasm32-unknown-unknown");
        target_dir.push(if cmd.release { "release" } else { "debug" });

        let mut output_wasm = target_dir.join(&lib_name);
        output_wasm.set_extension("wasm");

        anyhow::ensure!(
            output_wasm.exists(),
            CrateError::OutputNotFound(output_wasm)
        );

        Ok(Self { output_wasm })
    }

    fn root_package(metadata: &Metadata) -> Option<&Package> {
        let root_id = metadata.resolve.as_ref()?.root.as_ref()?;
        metadata
            .packages
            .iter()
            .find(|package| package.id == *root_id)
    }
}
