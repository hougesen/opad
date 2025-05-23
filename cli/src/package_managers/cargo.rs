use crate::parsers::toml;

#[inline]
fn set_package_version(package_table: &mut dyn toml_edit::TableLike, version: &str) -> bool {
    if package_table
        .get("version")
        .is_some_and(|outer| outer.as_str().is_some_and(|inner| inner != version))
    {
        package_table.insert(
            "version",
            toml_edit::Item::Value(toml_edit::Value::String(toml_edit::Formatted::new(
                version.into(),
            ))),
        );
        true
    } else {
        false
    }
}

#[inline]
pub fn set_cargo_toml_version(path: &std::path::Path, version: &str) -> anyhow::Result<bool> {
    let contents = std::fs::read_to_string(path)?;

    let mut document = toml::parse(&contents)?;

    let mut modified = false;

    if let Some(package_raw) = document.get_mut("package") {
        if let Some(package_table) = package_raw.as_table_like_mut() {
            modified |= set_package_version(package_table, version);
        }
    }

    if let Some(workspace) = document.get_mut("workspace") {
        if let Some(workspace_table) = workspace.as_table_like_mut() {
            if let Some(package) = workspace_table.get_mut("package") {
                if let Some(package_table) = package.as_table_like_mut() {
                    modified |= set_package_version(package_table, version);
                }
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
    std::process::Command::new("cargo")
        .arg("check")
        .current_dir(path)
        .spawn()?
        .wait()
        .map(|exit_code| exit_code.success())
}

#[cfg(test)]
mod test_set_cargo_toml_version {
    use crate::package_managers::cargo::set_cargo_toml_version;

    #[test]
    fn it_should_modify_version() -> anyhow::Result<()> {
        let version = "1.2.3";

        let input = r#"[package]
version = "0.0.0"
edition = "2024"
homepage = "https://github.com/hougesen/crosspmv?tab=readme-ov-file"
authors = ["Mads Hougesen <mads@mhouge.dk>"]
license = "MIT"
repository = "https://github.com/hougesen/crosspmv"
documentation = "https://github.com/hougesen/crosspmv#readme"

[dependencies]
anyhow = "1.0.98"
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

        let dir = tempfile::tempdir()?;

        let path = dir.path().join("Cargo.toml");

        std::fs::write(&path, input)?;

        {
            let modified = set_cargo_toml_version(&path, version)?;

            assert!(modified);

            let output = std::fs::read_to_string(&path)?;

            assert_eq!(output, expected_output);
        };

        // Validate we do not modify file if version is the same
        {
            let modified = set_cargo_toml_version(&path, version)?;

            assert!(!modified);

            let output = std::fs::read_to_string(&path)?;

            assert_eq!(output, expected_output);
        }

        Ok(())
    }

    #[test]
    fn it_should_modify_version_workspace() -> anyhow::Result<()> {
        let version = "1.2.3";

        let input = r#"[workspace]
members = ["cli"]
resolver = "3"

[workspace.package]
version = "0.0.0"
edition = "2024"
homepage = "https://github.com/hougesen/crosspmv?tab=readme-ov-file"
authors = ["Mads Hougesen <mads@mhouge.dk>"]
license = "MIT"
repository = "https://github.com/hougesen/crosspmv"
documentation = "https://github.com/hougesen/crosspmv#readme"

[workspace.dependencies]
anyhow = "1.0.98"
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

        let dir = tempfile::tempdir()?;

        let path = dir.path().join("Cargo.toml");

        std::fs::write(&path, input)?;

        {
            let modified = set_cargo_toml_version(&path, version)?;

            assert!(modified);

            let output = std::fs::read_to_string(&path)?;

            assert_eq!(output, expected_output);
        };

        // Validate we do not modify file if version is the same
        {
            let modified = set_cargo_toml_version(&path, version)?;

            assert!(!modified);

            let output = std::fs::read_to_string(&path)?;

            assert_eq!(output, expected_output);
        }

        Ok(())
    }
}
