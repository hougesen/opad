use super::npm::{self, PackageJsonError};

/// `elm.json` follows the same format as `package.json` (?)
#[inline]
pub fn set_elm_json_version(
    contents: String,
    version: &str,
) -> Result<(bool, String), PackageJsonError> {
    npm::set_package_json_version(contents, version)
}

#[inline]
pub const fn update_lock_files(_dir: &std::path::Path) -> bool {
    true
}

#[cfg(test)]
mod test_set_elm_json_version {
    use super::set_elm_json_version;

    const INPUT: &str = r#"{
  "type": "package",
  "name": "hougesen/opad",
  "summary": "",
  "license": "MIT",
  "version": "0.0.0",
  "exposed-modules": [],
  "elm-version": "0.19.0 <= v < 0.20.0",
  "dependencies": {
    "elm/core": "1.0.0 <= v < 2.0.0",
    "elm/json": "1.0.0 <= v < 2.0.0"
  },
  "test-dependencies": {}
}
"#;

    #[test]
    fn it_should_set_version() {
        let version = "45.1.92";
        let version_str = format!("\"version\": \"{version}\"");

        let expected_output = INPUT.replace("\"version\": \"0.0.0\"", &version_str);
        assert!(expected_output.contains(&version_str));

        let (modified, output) =
            set_elm_json_version(INPUT.to_string(), version).expect("it not to raise");

        assert!(modified);
        assert_eq!(output, expected_output);

        // Validate we do not modify file if version is the same
        {
            let (modified, output) =
                set_elm_json_version(output, version).expect("it not to raise");

            assert!(!modified);

            assert_eq!(output, expected_output);
        }
    }
}
