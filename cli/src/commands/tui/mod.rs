use std::env::current_dir;

use crossterm::style::Stylize;

use crate::fs::find_package_manager_files;

#[inline]
pub fn run_command() -> anyhow::Result<()> {
    let dir = current_dir()?;

    let files = find_package_manager_files(&dir);

    if files.is_empty() {
        println!("{}", "No supported package managers found".yellow().bold());

        return Ok(());
    }

    let selected = inquire::MultiSelect::new("Which files do you wish to update?", files)
        .with_vim_mode(true)
        .prompt()?;

    if selected.is_empty() {
        println!("{}", "No files selected".yellow().bold());

        return Ok(());
    }

    let version = inquire::Text::new("What do you wish to set the version to?").prompt()?;

    for s in selected {
        s.set_package_version(&version)?;
    }

    println!("{}", "Files has been updated".bold().green());

    Ok(())
}
