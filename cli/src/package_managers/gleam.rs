use crate::parsers::toml;

#[derive(Debug)]
pub enum GleamTomlError {
    InvalidVersionFieldDataType,
    MissingVersionField,
    ParseToml(Box<toml_edit::TomlError>),
}

impl core::error::Error for GleamTomlError {}

impl core::fmt::Display for GleamTomlError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidVersionFieldDataType => write!(f, "\"version\" field is not a string"),
            Self::MissingVersionField => write!(f, "\"version\" field not found"),
            Self::ParseToml(error) => error.fmt(f),
        }
    }
}

#[inline]
pub fn set_gleam_toml_version(
    contents: String,
    version: &str,
) -> Result<(bool, String), GleamTomlError> {
    let mut document =
        toml::parse(&contents).map_err(|error| GleamTomlError::ParseToml(Box::new(error)))?;

    let version_key = document
        .get("version")
        .ok_or(GleamTomlError::MissingVersionField)?;

    let version_key_str = version_key
        .as_str()
        .ok_or(GleamTomlError::InvalidVersionFieldDataType)?;

    let modified = version_key_str != version;

    let output = if modified {
        document.insert(
            "version",
            toml_edit::Item::Value(toml_edit::Value::String(toml_edit::Formatted::new(
                version.into(),
            ))),
        );

        toml::serialize(&document)
    } else {
        contents
    };

    Ok((modified, output))
}

#[inline]
pub const fn update_lock_files(_dir: &std::path::Path) -> bool {
    // NOTE: manifest.toml does not include the package version?

    true
}

#[cfg(test)]
mod test_set_gleam_toml_version {
    use crate::package_managers::error::PackageManagerError;

    #[test]
    fn it_should_modify_version() {
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

        let (modified, output) =
            super::set_gleam_toml_version(input.to_string(), version).expect("it not to raise");

        assert!(modified);

        assert_eq!(output, expected_output);

        // Validate we do not modify file if version is the same
        {
            let (modified, output) =
                super::set_gleam_toml_version(output, version).expect("it not to raise");

            assert!(!modified);

            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn it_should_require_version_field() {
        let input = "";

        let result = super::set_gleam_toml_version(input.to_string(), "1.23.4")
            .expect_err("it should return an error");

        assert!(matches!(result, super::GleamTomlError::MissingVersionField));

        assert!(result.to_string().contains("\"version\""));

        PackageManagerError::from(result).test_up_casting();
    }

    #[test]
    fn version_field_should_be_string() {
        let input = "[version]\nkey = \"value\"\n";

        let result = super::set_gleam_toml_version(input.to_string(), "1.23.4")
            .expect_err("it should return an error");

        assert!(matches!(
            result,
            super::GleamTomlError::InvalidVersionFieldDataType
        ));

        assert!(result.to_string().contains("\"version\""));

        PackageManagerError::from(result).test_up_casting();
    }
}
