use std::process::{self, Command};

use anyhow::Result;

pub(crate) fn run_cargo(args: Vec<&str>) -> Result<()> {
    let mut cargo = Command::new("cargo");
    cargo.arg("+nightly");
    cargo.args(args);

    let status = cargo.status()?;
    if !status.success() {
        process::exit(1);
    }
    Ok(())
}
