mod error;

use anyhow::Result;
use clap::Clap;

/// Utility to simplify Gear programs development
#[derive(Clap, Debug)]
#[clap(
    bin_name = "cargo",
    version = clap::crate_version!(),
)]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clap, Debug)]
enum Command {
    #[clap(
        name = "program",
        version = clap::crate_version!(),
    )]
    Program(ProgramCommand),
}

/// Utility to simplify Gear programs development
#[derive(Clap, Debug)]
struct ProgramCommand {
    #[clap(subcommand)]
    sub_command: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    New(NewSubCommand),
}

/// Create a new Gear program
#[derive(Clap, Debug)]
struct NewSubCommand {
    /// Project name
    name: String,
}

pub fn run() -> Result<()> {
    let opts = Opts::parse();
    dbg!(opts);
    Ok(())
}
