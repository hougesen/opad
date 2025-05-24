use super::npm::{PackageJsonError, set_package_json_version};

#[inline]
pub fn set_lerna_json_version(
    contents: String,
    version: &str,
) -> Result<(bool, String), PackageJsonError> {
    set_package_json_version(contents, version)
}

#[inline]
pub const fn update_lock_files(_dir: &std::path::Path) -> bool {
    // TODO: ?
    true
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
