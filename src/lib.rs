mod commands;
mod common;
mod error;
mod output_info;
mod runner;

use anyhow::Result;
use clap::Parser;

use crate::commands::*;

/// Utility to simplify Gear programs development
#[derive(Debug, Parser)]
#[clap(
    bin_name = "cargo",
    version = clap::crate_version!(),
)]
struct Opts {
    #[clap(subcommand)]
    command: Util,
}

#[derive(Debug, Parser)]
enum Util {
    #[clap(
        name = "program",
        version = clap::crate_version!(),
    )]
    Program(ProgramUtil),
}

/// Utility to simplify Gear programs development
#[derive(Debug, Parser)]
struct ProgramUtil {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
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
