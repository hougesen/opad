use logging::log_error;

mod cli;
mod commands;
mod error;
mod fs;
mod logging;
mod package_managers;
mod parsers;

fn main() {
    if let Err(error) = commands::execute_command() {
        log_error(&format!("{error}"));

        std::process::exit(1)
    }
}
