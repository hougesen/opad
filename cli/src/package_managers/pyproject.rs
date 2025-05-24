use super::run_update_lock_file_command;
use crate::parsers::toml;

#[derive(Debug)]
pub enum PyprojectTomlError {
    MissingProjectField,
    InvalidProjectFieldDataType,
    InvalidVersionFieldDataType,
    MissingVersionField,
}

impl core::error::Error for PyprojectTomlError {}

impl core::fmt::Display for PyprojectTomlError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingProjectField => write!(f, "\"project\" field not found"),
            Self::InvalidProjectFieldDataType => write!(f, "\"project\" field is not a table"),
            Self::MissingVersionField => write!(f, "\"project.version\" field not found"),
            Self::InvalidVersionFieldDataType => {
                write!(f, "\"project.version\" field is not a string")
            }
        }
    }
}

#[inline]
pub fn set_version(path: &std::path::Path, version: &str) -> Result<bool, crate::error::Error> {
    let contents = std::fs::read_to_string(path)?;

    let mut document = toml::parse(&contents)?;

    let package_raw = document
        .get_mut("project")
        .ok_or(PyprojectTomlError::MissingProjectField)?;

    let package_table = package_raw
        .as_table_like_mut()
        .ok_or(PyprojectTomlError::InvalidProjectFieldDataType)?;

    let version_key = package_table
        .get("version")
        .ok_or(PyprojectTomlError::MissingVersionField)?;

    let version_key_str = version_key
        .as_str()
        .ok_or(PyprojectTomlError::InvalidVersionFieldDataType)?;

    let modified = version_key_str != version;

    if modified {
        package_table.insert(
            "version",
            toml_edit::Item::Value(toml_edit::Value::String(toml_edit::Formatted::new(
                version.into(),
            ))),
        );

        toml::save(path, &document)?;
    }

    Ok(modified)
}

#[inline]
fn uv_enabled(dir: &std::path::Path) -> bool {
    dir.join("uv.lock").exists()
}

#[inline]
fn uv_update_lock_file_command() -> std::process::Command {
    let mut cmd = std::process::Command::new("uv");
    cmd.arg("lock");
    cmd
}

#[inline]
fn rye_update_lock_file_command() -> std::process::Command {
    let mut cmd = std::process::Command::new("rye");
    cmd.arg("lock");
    cmd
}

#[inline]
fn rye_enabled(dir: &std::path::Path) -> bool {
    dir.join("requirements.lock").exists() || dir.join("requirements-dev.lock").exists()
}

#[inline]
fn poetry_enabled(dir: &std::path::Path) -> bool {
    dir.join("poetry.lock").exists()
}

#[inline]
pub fn update_lock_files(dir: &std::path::Path) -> std::io::Result<bool> {
    if uv_enabled(dir) {
        run_update_lock_file_command(uv_update_lock_file_command(), dir)
    } else if rye_enabled(dir) {
        run_update_lock_file_command(rye_update_lock_file_command(), dir)
    } else if poetry_enabled(dir) {
        // TODO: do something?
        // NOTE: does poetry.lock even include version of package?
        Ok(true)
    } else {
        Ok(true)
    }
}

#[cfg(test)]
mod test_set_version {
    use super::set_version;

    #[test]
    fn it_should_modify_version() -> Result<(), crate::error::Error> {
        let version = "1.2.3";

        let input = "[project]
version = \"0.0.0\"
name = \"uv-demo\"
description = \"Add your description here\"
readme = \"README.md\"
requires-python = \">=3.13\"
dependencies = []
";

        let new_version_line = format!("[project]\nversion = \"{version}\"");

        let expected_output =
            input.replacen("[project]\nversion = \"0.0.0\"", &new_version_line, 1);

        assert!(expected_output.contains(&new_version_line));

        let dir = tempfile::tempdir()?;

        let path = dir.path().join("pyproject.toml");

        std::fs::write(&path, input)?;

        {
            let modified = set_version(&path, version)?;

            assert!(modified);

            let output = std::fs::read_to_string(&path)?;

            assert_eq!(output, expected_output);
        };

        // Validate we do not modify file if version is the same
        {
            let modified = set_version(&path, version)?;

            assert!(!modified);

            let output = std::fs::read_to_string(&path)?;

            assert_eq!(output, expected_output);
        }

        Ok(())
    }
}
