#[inline]
pub fn parse(contents: &str) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str::<serde_json::Value>(contents)
}

#[inline]
pub fn save(path: &std::path::Path, document: &serde_json::Value) -> anyhow::Result<()> {
    let serialized = serde_json::to_string_pretty(&document)?;

    std::fs::write(path, format!("{}\n", serialized.trim()))?;

    Ok(())
}
