use colored::Colorize;

fn main() {
    log::debug!("running cargo-program in debug mode");
    if let Err(e) = cargo_program::run() {
        eprintln!(
            "{}{}",
            "error".bright_red().bold(),
            format!(": {}", e).bold()
        );
        log::error!("{}", e);
        e.chain().skip(1).for_each(|cause| {
            eprintln!(
                "{}      {}",
                "|".bright_red().bold(),
                format!("{}", cause).bright_red().bold()
            )
        });
        std::process::exit(1);
    }
}
