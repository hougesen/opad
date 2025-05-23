use clap::Parser;

use crate::cli::{Cli, Commands, InteractiveCommandArguments};

mod completions;
mod tui;

#[inline]
pub fn execute_command() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(command) => match command {
            Commands::Run(args) => tui::run_command(&args)?,
            Commands::Completions(args) => completions::run_command(&args, &mut std::io::stdout())?,
        },
        None => tui::run_command(&InteractiveCommandArguments::default())?,
    }

    Ok(())
}
