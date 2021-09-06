use std::{env, fs};

use anyhow::{ensure, Result};
use clap::Clap;

use crate::error::CrateError;

/// Create a new Gear program
#[derive(Clap, Debug)]
pub(crate) struct NewCommand {
    /// Project name
    name: String,
    /// Create async program
    #[clap(name = "async", short, long)]
    async_flag: bool,
}

impl NewCommand {
    pub fn run(&self) -> Result<()> {
        let dest_dir = env::current_dir()?.join(&self.name);
        ensure!(!dest_dir.exists(), CrateError::DestinationExists(dest_dir));
        fs::create_dir(&dest_dir)?;

        fs::write(dest_dir.join("Cargo.toml"), self.replace_name(include_bytes!("../templates/new/Cargo.toml"))?)?;
        Ok(())
    }

    fn replace_name(&self, source: &[u8]) -> Result<String> {
        Ok(String::from_utf8(source.to_vec())?.replace("{{name}}", &self.name))
    }
}
