use clap::CommandFactory;

use crate::cli::{Cli, Shell};

#[inline]
pub fn run_command(shell: Shell, buffer: &mut impl std::io::Write) -> Result<(), std::io::Error> {
    let mut cmd = Cli::command();

    let cmd_name = cmd.get_name().to_string();

    match shell {
        Shell::Bash => {
            clap_complete::generate(clap_complete::Shell::Bash, &mut cmd, cmd_name, buffer);
        }
        Shell::Elvish => {
            clap_complete::generate(clap_complete::Shell::Elvish, &mut cmd, cmd_name, buffer);
        }
        Shell::Fish => {
            clap_complete::generate(clap_complete::Shell::Fish, &mut cmd, cmd_name, buffer);
        }
        Shell::Nushell => {
            clap_complete::generate(clap_complete_nushell::Nushell, &mut cmd, cmd_name, buffer);
        }
        Shell::Powershell => {
            clap_complete::generate(clap_complete::Shell::PowerShell, &mut cmd, cmd_name, buffer);
        }
        Shell::Zsh => {
            clap_complete::generate(clap_complete::Shell::Zsh, &mut cmd, cmd_name, buffer);
        }
    }

    buffer.flush()
}

#[cfg(test)]
mod test_run_command {
    use crate::cli::Shell;

    #[test]
    fn it_should_write_shell_completions() {
        let shells = [
            Shell::Bash,
            Shell::Elvish,
            Shell::Fish,
            Shell::Nushell,
            Shell::Powershell,
            Shell::Zsh,
        ];

        for shell in shells {
            let mut buffer = Vec::new();

            super::run_command(shell, &mut buffer).unwrap();

            assert!(!buffer.is_empty());
        }
    }
}
