use clap::Parser;

use crate::cli::Cli;

mod completions;
mod tui;

#[inline]
pub fn execute_command() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(shell) = cli.completions {
        completions::run_command(shell, &mut std::io::stdout())?;
    } else {
        tui::run_command(&cli)?;
    }

    Ok(())
}
