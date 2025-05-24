use super::{
    npm::{self, PackageJsonError},
    run_update_lock_file_command,
};

/// `deno.json` follows the same format as `package.json` (?)
#[inline]
pub fn set_deno_json_version(
    contents: String,
    version: &str,
) -> Result<(bool, String), PackageJsonError> {
    npm::set_package_json_version(contents, version)
}

#[inline]
fn deno_update_lock_file_command() -> std::process::Command {
    let mut cmd = std::process::Command::new("deno");
    cmd.arg("install");
    cmd
}

#[inline]
pub fn update_lock_files(dir: &std::path::Path) -> std::io::Result<bool> {
    run_update_lock_file_command(deno_update_lock_file_command(), dir)
}

#[cfg(test)]
mod test_set_deno_json_version {
    use super::set_deno_json_version;

    const INPUT: &str = r#"{
  "name": "my-lib",
  "version": "0.0.0",
  "exports": "./mod.ts",
  "tasks": {
    "dev": "deno test --watch mod.ts"
  },
  "imports": {
    "@std/assert": "jsr:@std/assert@1"
  }
}
"#;

    #[test]
    fn it_should_set_version() {
        let version = "1.555.2";
        let version_str = format!("\"version\": \"{version}\"");

        let expected_output = INPUT.replace("\"version\": \"0.0.0\"", &version_str);
        assert!(expected_output.contains(&version_str));

        let (modified, output) =
            set_deno_json_version(INPUT.to_string(), version).expect("it not to raise");

        assert!(modified);
        assert_eq!(output, expected_output);

        // Validate we do not modify file if version is the same
        {
            let (modified, output) =
                set_deno_json_version(output, version).expect("it not to raise");

            assert!(!modified);

            assert_eq!(output, expected_output);
        }
    }
}
