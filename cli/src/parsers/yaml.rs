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
    StartLineNotFound,
}

impl core::error::Error for NodeReplaceError {}

impl core::fmt::Display for NodeReplaceError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SpanStartMissing => write!(f, "Internal: value node span is missing"),
            Self::StartLineNotFound => write!(f, "Internal: value node span start line not found"),
        }
    }
}

#[inline]
pub fn replace_node_value_in_input(
    input: &str,
    node: &MarkedScalarNode,
    replace_with: &str,
) -> Result<String, NodeReplaceError> {
    let mut output = String::new();

    let start = node
        .span()
        .start()
        .ok_or(NodeReplaceError::SpanStartMissing)?;

    let start_line = start.line() - 1;

    let mut value_replaced = false;

    for (index, line) in input.lines().enumerate() {
        if index == start_line {
            let start_index = start.column() - 1;

            let end_index = start_index + node.len();

            let mut line_string = line.to_string();

            line_string.replace_range(start_index..end_index, replace_with);

            output.push_str(&line_string);

            value_replaced = true;
        } else {
            output.push_str(line);
        }

        output.push('\n');
    }

    if value_replaced {
        Ok(output)
    } else {
        Err(NodeReplaceError::StartLineNotFound)
    }
}

#[cfg(test)]
mod test_parse {
    use crate::parsers::yaml::replace_node_value_in_input;

    const INPUT: &str = r#"# this is a comment

version:    9.1.2    #mads
name: Mads Hougesen
"#;

    #[test]
    fn it_should_support_comments() -> Result<(), marked_yaml::LoadError> {
        let mut document = super::parse(INPUT)?;

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

        Ok(())
    }
}
