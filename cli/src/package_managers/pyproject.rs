use super::{poetry, run_update_lock_file_command, rye, uv};
use crate::parsers::toml;

#[derive(Debug)]
pub enum PyprojectTomlError {
    InvalidProjectFieldDataType,
    InvalidProjectVersionFieldDataType,
    MissingProjectField,
    MissingProjectVersionField,
    ParseToml(Box<toml_edit::TomlError>),
}

impl core::error::Error for PyprojectTomlError {}

impl core::fmt::Display for PyprojectTomlError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidProjectFieldDataType => write!(f, "\"project\" field is not a table"),
            Self::InvalidProjectVersionFieldDataType => {
                write!(f, "\"project.version\" field is not a string")
            }
            Self::MissingProjectField => write!(f, "\"project\" field not found"),
            Self::MissingProjectVersionField => write!(f, "\"project.version\" field not found"),
            Self::ParseToml(error) => error.fmt(f),
        }
    }
}

#[inline]
pub fn set_pyproject_version(
    contents: String,
    version: &str,
) -> Result<(bool, String), PyprojectTomlError> {
    let mut document =
        toml::parse(&contents).map_err(|error| PyprojectTomlError::ParseToml(Box::new(error)))?;

    let package_raw = document
        .get_mut("project")
        .ok_or(PyprojectTomlError::MissingProjectField)?;

    let package_table = package_raw
        .as_table_like_mut()
        .ok_or(PyprojectTomlError::InvalidProjectFieldDataType)?;

    let version_key = package_table
        .get("version")
        .ok_or(PyprojectTomlError::MissingProjectVersionField)?;

    let version_key_str = version_key
        .as_str()
        .ok_or(PyprojectTomlError::InvalidProjectVersionFieldDataType)?;

    let modified = version_key_str != version;

    let output = if modified {
        package_table.insert(
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
pub fn update_lock_files(dir: &std::path::Path) -> std::io::Result<bool> {
    if uv::uv_enabled(dir) {
        run_update_lock_file_command(uv::uv_update_lock_file_command(), dir)
    } else if rye::rye_enabled(dir) {
        run_update_lock_file_command(rye::rye_update_lock_file_command(), dir)
    } else if poetry::poetry_enabled(dir) {
        // TODO: do something?
        // NOTE: does poetry.lock even include version of package?
        Ok(true)
    } else {
        Ok(true)
    }
}

#[cfg(test)]
mod test_set_pyproject_version {
    use super::set_pyproject_version;
    use crate::package_managers::{error::PackageManagerError, pyproject::PyprojectTomlError};

    #[test]
    fn it_should_modify_version() {
        let version = format!(
            "{}.{}.{}",
            rand::random::<u16>(),
            rand::random::<u16>(),
            rand::random::<u16>()
        );

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

        let (modified, output) =
            set_pyproject_version(input.to_string(), &version).expect("it to not raise");

        assert!(modified);

        assert_eq!(output, expected_output);

        // Validate we do not modify file if version is the same
        {
            let (modified, output) =
                set_pyproject_version(output, &version).expect("it not to raise");

            assert!(!modified);

            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn it_should_require_project_field() {
        let input = "\n";

        let result = set_pyproject_version(input.to_string(), "1.23.4")
            .expect_err("it should return an error");

        assert!(matches!(result, PyprojectTomlError::MissingProjectField));

        assert!(
            crate::error::Error::from(PackageManagerError::from(result))
                .to_string()
                .contains("\"project\"")
        );
    }

    #[test]
    fn project_field_should_be_a_table() {
        let input = "project = \"value\"\n";

        let result = set_pyproject_version(input.to_string(), "1.23.4")
            .expect_err("it should return an error");

        assert!(matches!(
            result,
            PyprojectTomlError::InvalidProjectFieldDataType
        ));

        assert!(
            crate::error::Error::from(PackageManagerError::from(result))
                .to_string()
                .contains("\"project\"")
        );
    }

    #[test]
    fn it_should_require_project_version_field() {
        let input = "[project]\nkey = \"value\"\n";

        let result = set_pyproject_version(input.to_string(), "1.23.4")
            .expect_err("it should return an error");

        assert!(matches!(
            result,
            PyprojectTomlError::MissingProjectVersionField
        ));

        assert!(
            crate::error::Error::from(PackageManagerError::from(result))
                .to_string()
                .contains("\"project.version\"")
        );
    }

    #[test]
    fn project_version_field_should_be_string() {
        let input = "[project.version]\nkey = \"value\"\n";

        let result = set_pyproject_version(input.to_string(), "1.23.4")
            .expect_err("it should return an error");

        assert!(matches!(
            result,
            PyprojectTomlError::InvalidProjectVersionFieldDataType
        ));

        assert!(
            crate::error::Error::from(PackageManagerError::from(result))
                .to_string()
                .contains("\"project.version\"")
        );
    }
}
