use clap::{Args, Parser, Subcommand};

const HELP_TEMPLATE: &str = "\
{before-help}{name} {version}
{about-with-newline}{author-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, propagate_version = true, help_template = HELP_TEMPLATE)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Update package versions
    Run(InteractiveCommandArguments),

    /// Generate shell completions
    Completions(ShellCompletionCommandArguments),
}

#[derive(Args, Debug, Default)]
pub struct InteractiveCommandArguments {
    #[arg(long, default_value_t = false)]
    pub check_hidden_files: bool,

    #[arg(long, default_value_t = false)]
    pub check_gitignored_files: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Shell {
    /// Bourne Again `SHell` (bash).
    Bash,

    /// Elvish shell (elvish).
    Elvish,

    /// Friendly Interactive `SHell` (fish).
    Fish,

    /// `Nushell` (nushell).
    Nushell,

    /// `PowerShell` (powershell).
    Powershell,

    /// Z `SHell` (zsh).
    Zsh,
}

impl clap::ValueEnum for Shell {
    #[inline]
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Bash,
            Self::Elvish,
            Self::Fish,
            Self::Nushell,
            Self::Powershell,
            Self::Zsh,
        ]
    }

    #[inline]
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Bash => clap::builder::PossibleValue::new("bash"),
            Self::Elvish => clap::builder::PossibleValue::new("elvish"),
            Self::Fish => clap::builder::PossibleValue::new("fish"),
            Self::Nushell => clap::builder::PossibleValue::new("nushell"),
            Self::Powershell => clap::builder::PossibleValue::new("powershell"),
            Self::Zsh => clap::builder::PossibleValue::new("zsh"),
        })
    }
}

#[derive(Args, Debug)]
pub struct ShellCompletionCommandArguments {
    #[arg()]
    pub shell: Shell,
}
