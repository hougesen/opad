use super::{
    bun, deno,
    npm::{self, PackageJsonError, set_package_json_version},
    pnpm, run_update_lock_file_command, yarn,
};
use crate::parsers::json;

#[derive(Debug)]
pub enum LernaJsonError {
    ParseJson(Box<serde_json::Error>),
}

impl core::error::Error for LernaJsonError {}

impl core::fmt::Display for LernaJsonError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ParseJson(error) => error.fmt(f),
        }
    }
}

#[inline]
pub fn set_lerna_json_version(
    contents: String,
    version: &str,
) -> Result<(bool, String), PackageJsonError> {
    set_package_json_version(contents, version)
}

#[inline]
pub fn update_lock_files(dir: &std::path::Path) -> Result<bool, crate::error::Error> {
    let lerna_json_file = dir.join("lerna.json");

    let contents = std::fs::read_to_string(lerna_json_file)?;

    let parsed =
        json::parse(&contents).map_err(|error| LernaJsonError::ParseJson(Box::new(error)))?;

    let npm_client = parsed
        .get("npmClient")
        .and_then(|field| field.as_str())
        .unwrap_or("npm");

    let command = match npm_client {
        "pnpm" => Some(pnpm::pnpm_update_lock_file_command()),
        "npm" => Some(npm::npm_update_lock_file_command()),
        "bun" => Some(bun::bun_update_lock_file_command()),
        "deno" => Some(deno::deno_update_lock_file_command()),
        "yarn" => Some(yarn::yarn_update_lock_file_command()),
        _ => None,
    };

    let result = if let Some(command) = command {
        run_update_lock_file_command(command, dir)?
    } else {
        npm::update_lock_files(dir)?
    };

    Ok(result)
}

#[cfg(test)]
mod test_set_lerna_json_version {
    use super::set_lerna_json_version;

    const INPUT: &str = r#"{
  "version": "0.0.0",
  "packages": [
    "packages/*"
  ],
  "npmClient": "yarn",
  "command": {
    "version": {
      "syncWorkspaceLock": true
    }
  }
}
"#;

    #[test]
    fn it_should_modify_version() {
        let version = format!(
            "{}.{}.{}",
            rand::random::<u16>(),
            rand::random::<u16>(),
            rand::random::<u16>()
        );

        let new_version_line = format!("\"version\": \"{version}\"");

        let expected_output = INPUT.replacen("\"version\": \"0.0.0\"", &new_version_line, 1);

        assert!(expected_output.contains(&new_version_line));

        let (modified, output) =
            set_lerna_json_version(INPUT.to_string(), &version).expect("it not to raise");

        assert!(modified);

        assert_eq!(output, expected_output);

        // Validate we do not modify file if version is the same
        {
            let (modified, output) =
                set_lerna_json_version(output, &version).expect("it not to raise");

            assert!(!modified);

            assert_eq!(output, expected_output);
        }
    }
}
