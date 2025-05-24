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
pub fn save(input: &str) -> String {
    format!("{}\n", input.trim())
}

#[inline]
pub fn replace_node(input: &str, node: &MarkedScalarNode, replace_with: &str) -> String {
    let mut output = String::new();

    if let Some(start) = node.span().start() {
        for (index, line) in input.lines().enumerate() {
            if index + 1 == start.line() {
                let mut temp = line.to_string();

                let start_index = start.column() - 1;

                let end_index = start_index + node.len();

                temp.replace_range(start_index..end_index, replace_with);

                output.push_str(&temp);
            } else {
                output.push_str(line);
            }

            output.push('\n');
        }
    }

    output
}

#[cfg(test)]
mod test_parse {
    use crate::parsers::yaml::replace_node;

    const INPUT: &str = r#"# this is a comment

version:    9.1.2    #mads
name: Mads Hougesen
"#;

    #[test]
    fn it_should_support_comments() -> Result<(), crate::error::Error> {
        let mut document = super::parse(INPUT)?;

        let mut output = String::new();

        if let Some(map) = document.as_mapping_mut() {
            if let Some(version_node) = map.get_node("version") {
                if let Some(scalar) = version_node.as_scalar() {
                    output = replace_node(INPUT, scalar, "1.2.3");
                }
            }
        }

        let expected_output = r#"# this is a comment

version:    1.2.3    #mads
name: Mads Hougesen
"#;

        assert_eq!(output, expected_output);

        Ok(())
    }
}
