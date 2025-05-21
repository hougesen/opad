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

    let version = inquire::Text::new("What do you wish to set the version to?")
        .with_validator(inquire::validator::MinLengthValidator::new(1))
        .prompt()?;

    let mut modified_files = Vec::new();

    for s in selected {
        if s.set_package_version(&version)? {
            modified_files.push(s);
        }
    }

    println!("{}", "Files has been updated".bold().green());

    let should_update_lock_files = inquire::Confirm::new("Do you wish to update the lock files?")
        .with_default(false)
        .prompt_skippable()?;

    if should_update_lock_files.unwrap_or_default() {
        for f in modified_files {
            println!(
                "{}",
                format!("Updating lockfiles connected to {}", f.path.display())
                    .blue()
                    .bold()
            );

            if !f.update_lock_files()? {
                eprintln!("{}", "Error updating lockfiles".bold().red());
            }
        }
    }

    Ok(())
}
