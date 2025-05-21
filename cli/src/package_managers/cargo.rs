#[inline]
fn set_package_version(package_table: &mut dyn toml_edit::TableLike, version: &str) -> bool {
    if package_table
        .get("version")
        .is_some_and(|outer| outer.as_str().is_some_and(|inner| inner != version))
    {
        package_table.insert(
            "version",
            toml_edit::Item::Value(toml_edit::Value::String(toml_edit::Formatted::new(
                version.to_string(),
            ))),
        );
        true
    } else {
        false
    }
}

#[inline]
pub fn set_cargo_toml_version(path: &std::path::Path, version: &str) -> anyhow::Result<bool> {
    let contents = std::fs::read_to_string(path)?;

    let mut document = contents.parse::<toml_edit::DocumentMut>()?;

    let mut modified = false;

    if let Some(package_raw) = document.get_mut("package") {
        if let Some(package_table) = package_raw.as_table_like_mut() {
            modified |= set_package_version(package_table, version);
        }
    }

    if let Some(workspace) = document.get_mut("workspace") {
        if let Some(workspace_table) = workspace.as_table_like_mut() {
            if let Some(package) = workspace_table.get_mut("package") {
                if let Some(package_table) = package.as_table_like_mut() {
                    modified |= set_package_version(package_table, version);
                }
            }
        }
    }

    if modified {
        std::fs::write(path, document.to_string())?;
    }

    Ok(modified)
}
