use super::run_update_lock_file_command;
use crate::parsers::toml;

#[derive(Debug)]
pub enum CargoTomlError {
    InvalidPackageFieldDataType { workspace: bool },
    InvalidVersionFieldDataType { workspace: bool },
    InvalidWorkspaceFieldDataType,
    MissingPackageField { workspace: bool },
    MissingVersionField { workspace: bool },
    ParseToml(Box<toml_edit::TomlError>),
}

impl core::error::Error for CargoTomlError {}

impl core::fmt::Display for CargoTomlError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ParseToml(error) => error.fmt(f),
            Self::InvalidVersionFieldDataType { workspace } => {
                let field = if *workspace {
                    "\"workspace.package.version\""
                } else {
                    "\"package.version\""
                };

                write!(f, "{field} field is not a string")
            }
            Self::InvalidPackageFieldDataType { workspace } => {
                let field = if *workspace {
                    "\"workspace.package\""
                } else {
                    "\"package\""
                };

                write!(f, "{field} field is not a table")
            }
            Self::MissingVersionField { workspace } => {
                let field = if *workspace {
                    "\"workspace.package.version\""
                } else {
                    "\"package.version\""
                };

                write!(f, "{field} field not found")
            }
            Self::MissingPackageField { workspace } => {
                let field = if *workspace {
                    "\"workspace.package\""
                } else {
                    "\"package\""
                };

                write!(f, "{field} field not found")
            }
            Self::InvalidWorkspaceFieldDataType => write!(f, "\"workspace\" is not a table"),
        }
    }
}

#[inline]
fn set_package_version(
    package_table: &mut dyn toml_edit::TableLike,
    version: &str,
    workspace: bool,
) -> Result<bool, CargoTomlError> {
    let version_key = package_table
        .get("version")
        .ok_or(CargoTomlError::MissingVersionField { workspace })?;

    let version_key_str = version_key
        .as_str()
        .ok_or(CargoTomlError::InvalidVersionFieldDataType { workspace })?;

    let modified = version_key_str != version;

    if modified {
        package_table.insert(
            "version",
            toml_edit::Item::Value(toml_edit::Value::String(toml_edit::Formatted::new(
                version.into(),
            ))),
        );
    }

    Ok(modified)
}

#[inline]
pub fn set_cargo_toml_version(
    contents: String,
    version: &str,
) -> Result<(bool, String), CargoTomlError> {
    let mut document =
        toml::parse(&contents).map_err(|error| CargoTomlError::ParseToml(Box::new(error)))?;

    let mut modified = false;

    if let Some(workspace) = document.get_mut("workspace") {
        let workspace_table = workspace
            .as_table_like_mut()
            .ok_or(CargoTomlError::InvalidWorkspaceFieldDataType)?;

        let package = workspace_table
            .get_mut("package")
            .ok_or(CargoTomlError::MissingPackageField { workspace: true })?;

        let package_table = package
            .as_table_like_mut()
            .ok_or(CargoTomlError::InvalidPackageFieldDataType { workspace: true })?;

        modified |= set_package_version(package_table, version, true)?;
    } else if let Some(package_raw) = document.get_mut("package") {
        let package_table = package_raw
            .as_table_like_mut()
            .ok_or(CargoTomlError::InvalidPackageFieldDataType { workspace: false })?;

        modified |= set_package_version(package_table, version, false)?;
    } else {
        return Err(CargoTomlError::MissingPackageField { workspace: false });
    }

    let output = if modified {
        toml::serialize(&document)
    } else {
        contents
    };

    Ok((modified, output))
}

#[inline]
fn cargo_update_lock_file_command() -> std::process::Command {
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("check");
    cmd
}

#[inline]
pub fn update_lock_files(dir: &std::path::Path) -> std::io::Result<bool> {
    run_update_lock_file_command(cargo_update_lock_file_command(), dir)
}

#[cfg(test)]
mod test_set_cargo_toml_version {
    #[test]
    fn it_should_modify_version() -> Result<(), super::CargoTomlError> {
        let version = "1.2.3";

        let input = r#"[package]
version = "0.0.0"
edition = "2024"
homepage = "https://github.com/hougesen/opad?tab=readme-ov-file"
authors = ["Mads Hougesen <mads@mhouge.dk>"]
license = "MIT"
repository = "https://github.com/hougesen/opad"
documentation = "https://github.com/hougesen/opad#readme"

[dependencies]
crossterm = "0.29.0"
ignore = "0.4.23"
inquire = "0.7.5"
serde_json = { version = "1.0.140", features = ["preserve_order"] }
tempfile = "3.20.0"
toml_edit = "0.22.26"
"#;

        let new_version_line = format!("[package]\nversion = \"{version}\"");

        let expected_output =
            input.replacen("[package]\nversion = \"0.0.0\"", &new_version_line, 1);

        assert!(expected_output.contains(&new_version_line));

        let (modified, output) = super::set_cargo_toml_version(input.to_string(), version)?;

        assert!(modified);

        assert_eq!(output, expected_output);

        // Validate we do not modify file if version is the same
        {
            let (modified, output) = super::set_cargo_toml_version(output, version)?;

            assert!(!modified);

            assert_eq!(output, expected_output);
        }

        Ok(())
    }

    #[test]
    fn it_should_modify_version_workspace() -> Result<(), super::CargoTomlError> {
        let version = "1.2.3";

        let input = r#"[workspace]
members = ["cli"]
resolver = "3"

[workspace.package]
version = "0.0.0"
edition = "2024"
homepage = "https://github.com/hougesen/opad?tab=readme-ov-file"
authors = ["Mads Hougesen <mads@mhouge.dk>"]
license = "MIT"
repository = "https://github.com/hougesen/opad"
documentation = "https://github.com/hougesen/opad#readme"

[workspace.dependencies]
crossterm = "0.29.0"
ignore = "0.4.23"
inquire = "0.7.5"
serde_json = { version = "1.0.140", features = ["preserve_order"] }
tempfile = "3.20.0"
toml_edit = "0.22.26"
"#;

        let new_version_line = format!("[workspace.package]\nversion = \"{version}\"");

        let expected_output = input.replacen(
            "[workspace.package]\nversion = \"0.0.0\"",
            &new_version_line,
            1,
        );

        assert!(expected_output.contains(&new_version_line));

        let (modified, output) = super::set_cargo_toml_version(input.to_string(), version)?;

        assert!(modified);

        assert_eq!(output, expected_output);

        // Validate we do not modify file if version is the same
        {
            let (modified, output) = super::set_cargo_toml_version(output, version)?;

            assert!(!modified);

            assert_eq!(output, expected_output);
        }

        Ok(())
    }
}
