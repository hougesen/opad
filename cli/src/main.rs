use crossterm::style::Stylize;

mod commands;
mod fs;
mod package_managers;
mod parsers;

fn main() {
    if let Err(error) = commands::tui::run_command() {
        eprintln!("{}", format!("{error}").red().bold());

        std::process::exit(1)
    }
}
