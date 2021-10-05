mod commands;
mod common;
mod error;
mod output_info;

use anyhow::Result;
use clap::Clap;

use crate::commands::*;

/// Utility to simplify Gear programs development
#[derive(Clap, Debug)]
#[clap(
    bin_name = "cargo",
    version = clap::crate_version!(),
)]
struct Opts {
    #[clap(subcommand)]
    command: Util,
}

#[derive(Clap, Debug)]
enum Util {
    #[clap(
        name = "program",
        version = clap::crate_version!(),
    )]
    Program(ProgramUtil),
}

/// Utility to simplify Gear programs development
#[derive(Clap, Debug)]
struct ProgramUtil {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clap, Debug)]
enum Command {
    New(NewCommand),
    Build(BuildCommand),
    Run(RunCommand),
    Test(TestCommand),
    Login(LoginCommand),
    Publish(PublishCommand),
}

pub fn run() -> Result<()> {
    let opts = Opts::parse();
    let Util::Program(ProgramUtil { command }) = opts.command;
    match command {
        Command::New(command) => command.run(),
        Command::Build(command) => command.run().and(Ok(())),
        Command::Run(command) => command.run(),
        Command::Test(command) => command.run(),
        Command::Login(command) => command.run(),
        Command::Publish(command) => command.run(),
    }
}
