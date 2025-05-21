use anyhow::Ok;

#[inline]
pub fn set_package_json_version(path: &std::path::Path, version: &str) -> anyhow::Result<bool> {
    let contents = std::fs::read_to_string(path)?;

    let mut parsed = serde_json::from_str::<serde_json::Value>(&contents)?;

    let mut modified = false;

    if let Some(root) = parsed.as_object_mut() {
        if root
            .get("version")
            .is_some_and(|outer| outer.as_str().is_some_and(|inner| inner != version))
        {
            root.insert(
                "version".to_string(),
                serde_json::Value::String(version.to_string()),
            );

            modified = true;
        }
    }

    if modified {
        std::fs::write(
            path,
            format!("{}\n", serde_json::to_string_pretty(&parsed)?.trim()),
        )?;
    }

    Ok(modified)
}

#[inline]
pub fn update_lock_files(path: &std::path::Path) -> anyhow::Result<bool> {
    let exit_code = if path.join("pnpm-lock.yaml").exists() {
        let mut cmd = std::process::Command::new("pnpm");
        cmd.arg("install");
        cmd
    } else if path.join("bun.lockb").exists() {
        let mut cmd = std::process::Command::new("bun");
        cmd.arg("install");
        cmd
    } else if path.join("yarn.lock").exists() {
        let mut cmd = std::process::Command::new("yarn");
        cmd.arg("install");
        cmd
    } else {
        let mut cmd = std::process::Command::new("npm");
        cmd.arg("install");
        cmd
    }
    .current_dir(path)
    .spawn()?
    .wait()?;

    Ok(exit_code.success())
}

#[cfg(test)]
mod test_set_package_json_version {
    use super::set_package_json_version;

    #[test]
    fn it_should_modify_version() -> anyhow::Result<()> {
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

        let expected_output = format!(
            "{{
  \"name\": \"npm\",
  \"version\": \"{version}\",
  \"main\": \"index.js\",
  \"keywords\": [],
  \"author\": \"\",
  \"license\": \"ISC\",
  \"description\": \"\"
}}
"
        );

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
