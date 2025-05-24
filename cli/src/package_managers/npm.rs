use super::run_update_lock_file_command;
use crate::parsers::json;

#[derive(Debug)]
pub enum PackageJsonError {
    DocumentNotAnObject,
    InvalidVersionFieldDataType,
    MissingVersionField,
}

impl core::error::Error for PackageJsonError {}

impl core::fmt::Display for PackageJsonError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DocumentNotAnObject => write!(f, "Document is not an object"),
            Self::InvalidVersionFieldDataType => write!(f, "\"version\" field is not a string"),
            Self::MissingVersionField => write!(f, "\"version\" field not found"),
        }
    }
}

#[inline]
pub fn set_package_json_version(
    path: &std::path::Path,
    version: &str,
) -> Result<bool, crate::error::Error> {
    let contents = std::fs::read_to_string(path)?;

    let mut document = json::parse(&contents)?;

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

    if modified {
        root.insert(
            "version".to_owned(),
            serde_json::Value::String(version.into()),
        );

        json::save(path, &document)?;
    }

    Ok(modified)
}

#[inline]
fn bun_enabled(dir: &std::path::Path) -> bool {
    dir.join("bun.lock").exists() || dir.join("bun.lockb").exists()
}

#[inline]
fn yarn_enabled(dir: &std::path::Path) -> bool {
    dir.join("yarn.lock").exists()
}

#[inline]
fn pnpm_enabled(dir: &std::path::Path) -> bool {
    dir.join("pnpm-lock.yaml").exists() || dir.join("pnpm-lock.yml").exists()
}

#[inline]
fn npm_update_lock_file_command() -> std::process::Command {
    let mut cmd = std::process::Command::new("npm");
    cmd.arg("install");
    cmd
}

#[inline]
fn bun_update_lock_file_command() -> std::process::Command {
    let mut cmd = std::process::Command::new("bun");
    cmd.arg("install");
    cmd
}

#[inline]
fn pnpm_update_lock_file_command() -> std::process::Command {
    let mut cmd = std::process::Command::new("pnpm");
    cmd.arg("install");
    cmd
}

#[inline]
pub fn update_lock_files(dir: &std::path::Path) -> std::io::Result<bool> {
    if pnpm_enabled(dir) {
        run_update_lock_file_command(pnpm_update_lock_file_command(), dir)
    } else if bun_enabled(dir) {
        run_update_lock_file_command(bun_update_lock_file_command(), dir)
    } else if yarn_enabled(dir) {
        return Ok(true);
    } else {
        run_update_lock_file_command(npm_update_lock_file_command(), dir)
    }
}

#[cfg(test)]
mod test_set_package_json_version {
    use super::set_package_json_version;

    #[test]
    fn it_should_modify_version() -> Result<(), crate::error::Error> {
        let version = "1.2.3";

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

        let dir = tempfile::tempdir()?;

        let path = dir.path().join("package.json");

        std::fs::write(&path, input)?;

        {
            let modified = set_package_json_version(&path, version)?;

            assert!(modified);

            let output = std::fs::read_to_string(&path)?;

            assert_eq!(output, expected_output);
        };

        // Validate we do not modify file if version is the same
        {
            let modified = set_package_json_version(&path, version)?;

            assert!(!modified);

            let output = std::fs::read_to_string(&path)?;

            assert_eq!(output, expected_output);
        }

        Ok(())
    }
}
