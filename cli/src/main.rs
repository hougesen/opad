use crossterm::style::Stylize;

mod cli;
mod commands;
mod error;
mod fs;
mod package_managers;
mod parsers;

fn main() {
    if let Err(error) = commands::execute_command() {
        eprintln!("{}", format!("{error}").red().bold());

        std::process::exit(1)
    }
}
