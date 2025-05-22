#[inline]
pub fn set_version(path: &std::path::Path, version: &str) -> anyhow::Result<bool> {
    let contents = std::fs::read_to_string(path)?;
    println!("conents {contents}");

    let mut document = contents.parse::<toml_edit::DocumentMut>()?;

    let mut modified = false;

    let should_modify = document
        .get("version")
        .is_some_and(|outer| outer.as_str().is_some_and(|inner| inner != version));

    if should_modify {
        document.insert(
            "version",
            toml_edit::Item::Value(toml_edit::Value::String(toml_edit::Formatted::new(
                version.to_string(),
            ))),
        );
        modified = true;
    }

    if modified {
        std::fs::write(path, document.to_string())?;
    }

    Ok(modified)
}

#[inline]
pub const fn update_lock_files(_path: &std::path::Path) -> anyhow::Result<bool> {
    // NOTE: manifest.toml does not include the package version?

    Ok(true)
}

#[cfg(test)]
mod test_set_version {
    use super::set_version;

    #[test]
    fn it_should_modify_version() -> anyhow::Result<()> {
        let version = "1.2.3";

        let input = r#"name = "sgleam_demo"
version = "0.0.0"

# Fill out these fields if you intend to generate HTML documentation or publish
# your project to the Hex package manager.
#
# description = ""
# licences = ["Apache-2.0"]
# repository = { type = "github", user = "", repo = "" }
# links = [{ title = "Website", href = "" }]
#
# For a full reference of all the available options, you can have a look at
# https://gleam.run/writing-gleam/gleam-toml/.

[dependencies]
gleam_stdlib = ">= 0.44.0 and < 2.0.0"

[dev-dependencies]
gleeunit = ">= 1.0.0 and < 2.0.0"
"#;

        let new_version_line = format!("version = \"{version}\"");

        let expected_output = input.replacen("version = \"0.0.0\"", &new_version_line, 1);

        assert!(expected_output.contains(&new_version_line));

        let dir = tempfile::tempdir()?;

        let path = dir.path().join("gleam.toml");

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
