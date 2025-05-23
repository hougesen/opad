use crate::parsers::toml;

use super::run_update_lock_file_command;

#[inline]
pub fn set_version(path: &std::path::Path, version: &str) -> Result<bool, crate::error::Error> {
    let contents = std::fs::read_to_string(path)?;

    let mut document = toml::parse(&contents)?;

    let mut modified = false;

    if let Some(package_raw) = document.get_mut("project") {
        if let Some(package_table) = package_raw.as_table_like_mut() {
            let should_modify = package_table
                .get("version")
                .is_some_and(|outer| outer.as_str().is_some_and(|inner| inner != version));

            if should_modify {
                package_table.insert(
                    "version",
                    toml_edit::Item::Value(toml_edit::Value::String(toml_edit::Formatted::new(
                        version.into(),
                    ))),
                );
                modified = true;
            }
        }
    }

    if modified {
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
