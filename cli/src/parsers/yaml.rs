use marked_yaml::{LoaderOptions, types::MarkedScalarNode};

#[inline]
pub fn parse(input: &str) -> Result<marked_yaml::Node, marked_yaml::LoadError> {
    marked_yaml::parse_yaml_with_options(
        0,
        input,
        LoaderOptions::default()
            .lowercase_keys(false)
            .error_on_duplicate_keys(false),
    )
}

#[inline]
pub fn serialize(input: &str) -> String {
    format!("{}\n", input.trim())
}

#[derive(Debug)]
pub enum NodeReplaceError {
    SpanStartMissing,
}

impl core::error::Error for NodeReplaceError {}

impl core::fmt::Display for NodeReplaceError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SpanStartMissing => write!(f, "Internal: value node span is missing"),
        }
    }
}

#[inline]
pub fn replace_node_value_in_input(
    input: &str,
    node: &MarkedScalarNode,
    replace_with: &str,
) -> Result<String, NodeReplaceError> {
    let start = node
        .span()
        .start()
        .ok_or(NodeReplaceError::SpanStartMissing)?;

    let chars_before = input.chars().take(start.character());

    let chars_after = input
        .chars()
        .skip(start.character())
        .skip(node.chars().count());

    let output = chars_before
        .chain(replace_with.chars())
        .chain(chars_after)
        .collect();

    Ok(output)
}

#[cfg(test)]
mod test_parse {
    use crate::parsers::yaml::replace_node_value_in_input;

    const INPUT: &str = r#"# this is a comment

version:    9.1.2    #mads
name: Mads Hougesen
"#;

    #[test]
    fn it_should_support_comments() {
        let mut document = super::parse(INPUT).expect("it to parse");

        let map = document.as_mapping_mut().expect("document to be a map");

        let version_node = map
            .get_node("version")
            .expect("document to have a version field");

        let scalar = version_node.as_scalar().expect("node to be a scalar");

        let output =
            replace_node_value_in_input(INPUT, scalar, "1.2.3").expect("input to be replace");

        let expected_output = r#"# this is a comment

version:    1.2.3    #mads
name: Mads Hougesen
"#;

        assert_eq!(output, expected_output);
    }

    #[test]
    fn it_should_not_replace_line_endings_crlf() {
        let input = INPUT.lines().collect::<Vec<_>>().join("\r\n");

        let mut document = super::parse(&input).expect("it to parse");

        let map = document.as_mapping_mut().expect("document to be a map");

        let version_node = map
            .get_node("version")
            .expect("document to have a version field");

        let scalar = version_node.as_scalar().expect("node to be a scalar");

        let output =
            replace_node_value_in_input(&input, scalar, "1.2.3").expect("input to be replace");

        let expected_output =
            "# this is a comment\r\n\r\nversion:    1.2.3    #mads\r\nname: Mads Hougesen";

        assert_eq!(output, expected_output);
    }
}
