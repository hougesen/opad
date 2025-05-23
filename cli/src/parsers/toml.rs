#[inline]
pub fn parse(contents: &str) -> Result<toml_edit::DocumentMut, toml_edit::TomlError> {
    contents.parse::<toml_edit::DocumentMut>()
}

#[inline]
pub fn save(path: &std::path::Path, document: &toml_edit::DocumentMut) -> std::io::Result<()> {
    let serialized = document.to_string();

    std::fs::write(path, format!("{}\n", serialized.trim()))
}

#[cfg(test)]
mod test_parse {
    #[test]
    fn it_should_keep_comments() -> Result<(), toml_edit::TomlError> {
        let input = "# this is a comment
key = \"value\"
";

        let parsed = super::parse(input)?;

        assert_eq!(input, parsed.to_string());

        Ok(())
    }
}
