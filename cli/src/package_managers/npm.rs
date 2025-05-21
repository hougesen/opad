#[inline]
pub fn set_package_json_version(path: &std::path::Path, version: &str) -> anyhow::Result<bool> {
    let mut modified = false;

    let contents = std::fs::read_to_string(path)?;

    let mut parsed = serde_json::from_str::<serde_json::Value>(&contents)?;

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
