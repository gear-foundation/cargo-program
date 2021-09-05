fn main() {
    log::debug!("running cargo-program in debug mode");
    if let Err(e) = cargo_program::run() {
        log::error!("{}", e);
        e.chain().skip(1).for_each(|cause| log::error!("{}", cause));
        std::process::exit(1);
    }
}
