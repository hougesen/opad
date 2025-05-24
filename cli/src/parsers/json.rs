#[inline]
pub fn parse(contents: &str) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str::<serde_json::Value>(contents)
}

#[inline]
pub fn save(document: &serde_json::Value) -> Result<String, serde_json::Error> {
    let serialized = serde_json::to_string_pretty(&document)?;

    Ok(format!("{}\n", serialized.trim()))
}
