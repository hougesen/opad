pub fn set_cargo_toml_version(path: &std::path::Path, version: &str) -> anyhow::Result<bool> {
    let mut modified = false;

    let contents = std::fs::read_to_string(path)?;

    let mut parsed = contents.parse::<toml_edit::DocumentMut>()?;

    if let Some(package_raw) = parsed.get_mut("package") {
        if let Some(package_table) = package_raw.as_table_mut() {
            if package_table.get("version").is_some_and(|v| v.is_str()) {
                package_table.insert(
                    "version",
                    toml_edit::Item::Value(toml_edit::Value::String(toml_edit::Formatted::new(
                        version.to_string(),
                    ))),
                );
                modified = true;
            }
        }
    }

    std::fs::write(path, parsed.to_string())?;

    Ok(modified)
}
