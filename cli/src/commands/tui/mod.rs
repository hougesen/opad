use crate::{
    cli::Cli,
    fs::{find_package_manager_files, setup_walker},
    logging::{log_error, log_info, log_success, log_warn},
};

#[inline]
pub fn run_command(args: &Cli) -> Result<(), crate::error::Error> {
    let dir = std::env::current_dir()?;

    let walker = setup_walker(&dir, args.check_gitignored_files, args.check_hidden_files);

    let files = find_package_manager_files(walker, &dir);

    if files.is_empty() {
        log_warn("No supported package managers found");

        return Ok(());
    }

    let selected = inquire::MultiSelect::new("Which files do you wish to update?", files)
        .with_vim_mode(true)
        .prompt()?;

    if selected.is_empty() {
        log_warn("No files selected");

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

        if let Err(error) = s.set_package_version(&version) {
            log_error(&format!("{error}"));
        } else {
            log_success(&format!("{} has been updated", s.path.display()));

            let should_update_lock_files =
                inquire::Confirm::new("Do you wish to update the lock files (experimental)")
                    .with_default(true)
                    .prompt_skippable()?;

            if should_update_lock_files.unwrap_or_default() {
                log_info(&format!(
                    "Updating lock files connected to {}",
                    s.path.display()
                ));

                loop {
                    if s.update_lock_files()? {
                        log_success("Lock files has been updated");
                        break;
                    }

                    log_error("Error updating lock files");

                    let retry = inquire::Confirm::new("Do you wish to retry?")
                        .with_default(false)
                        .prompt_skippable()?;

                    if !retry.unwrap_or_default() {
                        break;
                    }
                }
            }
        }

        println!();
    }

    Ok(())
}
