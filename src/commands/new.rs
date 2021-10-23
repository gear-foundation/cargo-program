use std::{env, fs};

use anyhow::{ensure, Context, Result};
use clap::{AppSettings, Parser};

use crate::error::CrateError;

/// Create a new Gear program
#[derive(Debug, Parser)]
#[clap(global_setting=AppSettings::DisableVersionFlag)]
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
        fs::create_dir(&dest_dir).context("unable to create program directory")?;

        fs::write(
            dest_dir.join("Cargo.toml"),
            self.replace_fields(include_bytes!("templates/_Cargo.toml"))?,
        )?;

        let src_dir = dest_dir.join("src");
        fs::create_dir(&src_dir).context("unable to create `src` subdirectory")?;

        if self.async_flag {
            fs::write(
                src_dir.join("lib.rs"),
                include_bytes!("templates/_async_lib.rs"),
            )?;
        } else {
            fs::write(src_dir.join("lib.rs"), include_bytes!("templates/_lib.rs"))?;
        }
        Ok(())
    }

    fn replace_fields(&self, source: &[u8]) -> Result<String> {
        let source = String::from_utf8(source.to_vec())?;
        let result = source.replace("{{name}}", &self.name).replace(
            "{{async}}",
            if self.async_flag {
                "gstd-async = { git = \"https://github.com/gear-tech/gear.git\" }\n"
            } else {
                ""
            },
        );
        Ok(result)
    }
}
