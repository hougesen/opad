use crossterm::style::Stylize;

use crate::{
    cli::Cli,
    fs::{find_package_manager_files, setup_walker},
};

#[inline]
pub fn run_command(args: &Cli) -> anyhow::Result<()> {
    let dir = std::env::current_dir()?;

    let walker = setup_walker(&dir, args.check_gitignored_files, args.check_hidden_files);

    let files = find_package_manager_files(walker, &dir);

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

    let mut version = String::new();

    for s in &selected {
        version = inquire::Text::new(&format!(
            "{}: What do you wish to set the version to?",
            s.path.display()
        ))
        .with_validator(inquire::validator::MinLengthValidator::new(1))
        .with_initial_value(&version)
        .prompt()?;

        s.set_package_version(&version)?;
    }

    println!("{}", "Files has been updated".bold().green());

    let should_update_lock_files =
        inquire::Confirm::new("Do you wish to update the lock files? (experimental)")
            .with_default(true)
            .prompt_skippable()?;

    if should_update_lock_files.unwrap_or_default() {
        for f in selected {
            println!(
                "{}",
                format!("Updating lockfiles connected to {}", f.path.display())
                    .blue()
                    .bold()
            );

            loop {
                if f.update_lock_files()? {
                    break;
                }

                eprintln!("{}", "Error updating lockfiles".bold().red());

                let retry = inquire::Confirm::new("Do you wish to retry?")
                    .with_default(false)
                    .prompt_skippable()?;

                if !retry.unwrap_or_default() {
                    break;
                }
            }
        }
    }

    Ok(())
}
