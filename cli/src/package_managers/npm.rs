use super::{bun, pnpm, run_update_lock_file_command, yarn};
use crate::parsers::json;

#[derive(Debug)]
pub enum PackageJsonError {
    DocumentNotAnObject,
    InvalidVersionFieldDataType,
    MissingVersionField,
    ParseJson(Box<serde_json::Error>),
    SerializeJson(Box<serde_json::Error>),
}

impl core::error::Error for PackageJsonError {}

impl core::fmt::Display for PackageJsonError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DocumentNotAnObject => write!(f, "Document is not an object"),
            Self::InvalidVersionFieldDataType => write!(f, "\"version\" field is not a string"),
            Self::MissingVersionField => write!(f, "\"version\" field not found"),
            Self::ParseJson(error) | Self::SerializeJson(error) => error.fmt(f),
        }
    }
}

#[inline]
pub fn set_package_json_version(
    contents: String,
    version: &str,
) -> Result<(bool, String), PackageJsonError> {
    let mut document =
        json::parse(&contents).map_err(|error| PackageJsonError::ParseJson(Box::new(error)))?;

    let root = document
        .as_object_mut()
        .ok_or(PackageJsonError::DocumentNotAnObject)?;

    let version_key = root
        .get("version")
        .ok_or(PackageJsonError::MissingVersionField)?;

    let version_key_str = version_key
        .as_str()
        .ok_or(PackageJsonError::InvalidVersionFieldDataType)?;

    let modified = version_key_str != version;

    let output = if modified {
        root.insert(
            "version".to_owned(),
            serde_json::Value::String(version.into()),
        );

        json::serialize(&document)
            .map_err(|error| PackageJsonError::SerializeJson(Box::new(error)))?
    } else {
        contents
    };

    Ok((modified, output))
}

#[inline]
pub fn npm_update_lock_file_command() -> std::process::Command {
    let mut cmd = std::process::Command::new("npm");
    cmd.arg("install");
    cmd
}

#[inline]
pub fn update_lock_files(dir: &std::path::Path) -> std::io::Result<bool> {
    let command = if pnpm::pnpm_enabled(dir) {
        pnpm::pnpm_update_lock_file_command()
    } else if bun::bun_enabled(dir) {
        bun::bun_update_lock_file_command()
    } else if yarn::yarn_enabled(dir) {
        yarn::yarn_update_lock_file_command()
    } else {
        npm_update_lock_file_command()
    };

    run_update_lock_file_command(command, dir)
}

#[cfg(test)]
mod test_set_package_json_version {
    use super::{PackageJsonError, set_package_json_version};
    use crate::package_managers::error::PackageManagerError;

    #[test]
    fn it_should_modify_version() {
        let version = format!(
            "{}.{}.{}",
            rand::random::<u16>(),
            rand::random::<u16>(),
            rand::random::<u16>()
        );

        let input = "{
  \"name\": \"npm\",
  \"version\": \"0.0.0\",
  \"main\": \"index.js\",
  \"keywords\": [],
  \"author\": \"\",
  \"license\": \"ISC\",
  \"description\": \"\"
}
";

        let new_version_line = format!("\"version\": \"{version}\"");

        let expected_output = input.replacen("\"version\": \"0.0.0\"", &new_version_line, 1);

        assert!(expected_output.contains(&new_version_line));

        let (modified, output) =
            set_package_json_version(input.to_string(), &version).expect("it not to raise");

        assert!(modified);

        assert_eq!(output, expected_output);

        // Validate we do not modify file if version is the same
        {
            let (modified, output) =
                set_package_json_version(output, &version).expect("it not to raise");

            assert!(!modified);

            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn it_should_require_version_field() {
        let input = "{ \"name\": \"Mads\" }";

        let result = set_package_json_version(input.to_string(), "5.1.23")
            .expect_err("it to return an error");

        assert!(matches!(result, PackageJsonError::MissingVersionField));

        assert!(
            crate::error::Error::from(PackageManagerError::from(result))
                .to_string()
                .contains("\"version\"")
        );
    }

    #[test]
    fn version_field_should_be_a_string() {
        let input = "{ \"version\": {} }";

        let result = set_package_json_version(input.to_string(), "5.1.23")
            .expect_err("it to return an error");

        assert!(matches!(
            result,
            PackageJsonError::InvalidVersionFieldDataType
        ));

        assert!(
            crate::error::Error::from(PackageManagerError::from(result))
                .to_string()
                .contains("\"version\"")
        );
    }
}
