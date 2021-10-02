use colored::Colorize;

fn main() {
    env_logger::builder().format_timestamp(None).init();
    log::debug!("running cargo-program in debug mode");

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
