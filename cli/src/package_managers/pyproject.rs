use crate::parsers::toml;

#[inline]
pub fn set_version(path: &std::path::Path, version: &str) -> anyhow::Result<bool> {
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
pub fn update_lock_files(path: &std::path::Path) -> std::io::Result<bool> {
    if path.join("uv.lock").exists() {
        return std::process::Command::new("uv")
            .arg("lock")
            .current_dir(path)
            .spawn()?
            .wait()
            .map(|exit_code| exit_code.success());
    }

    if path.join("requirements.lock").exists() || path.join("requirements-dev.lock").exists() {
        return std::process::Command::new("rye")
            .arg("lock")
            .current_dir(path)
            .spawn()?
            .wait()
            .map(|exit_code| exit_code.success());
    }

    if path.join("poetry.lock").exists() {
        // TODO: update poetry lock file?
        // NOTE: does poetry.lock even include version of package?
    }

    // should this be false?
    Ok(true)
}

#[cfg(test)]
mod test_set_version {
    use super::set_version;

    #[test]
    fn it_should_modify_version() -> anyhow::Result<()> {
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
