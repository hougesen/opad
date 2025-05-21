#[inline]
pub fn set_version(path: &std::path::Path, version: &str) -> anyhow::Result<bool> {
    let contents = std::fs::read_to_string(path)?;

    let mut document = contents.parse::<toml_edit::DocumentMut>()?;

    let mut modified = false;

    if let Some(package_raw) = document.get_mut("project") {
        if let Some(package_table) = package_raw.as_table_like_mut() {
            let should_modify = package_table
                .get("version")
                .is_some_and(|outer| outer.as_str().is_some_and(|inner| inner != version));

            if should_modify {
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

    if modified {
        std::fs::write(path, document.to_string())?;
    }

    Ok(modified)
}

#[inline]
pub fn update_lock_files(_path: &std::path::Path) -> anyhow::Result<bool> {
    todo!()
}
