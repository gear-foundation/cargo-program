use colored::Colorize;
use env_logger::{Builder, Env};

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug"))
        //.format_timestamp(None)
        .format_target(false)
        .init();

    if let Err(e) = cargo_program::run() {
        eprintln!(
            "{}{}",
            "error".bright_red().bold(),
            format!(": {}", e).bold()
        );
        e.chain()
            .skip(1)
            .for_each(|cause| eprintln!("{}", format!("|      {}", cause).bright_red().bold()));
        std::process::exit(1);
    }
}
