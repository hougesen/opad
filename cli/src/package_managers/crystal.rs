use crate::parsers::yaml;

#[derive(Debug)]
pub enum ShardYmlError {
    InvalidDocument,
    InvalidVersionFieldDataType,
    MissingVersionField,
    ParseYml(Box<marked_yaml::LoadError>),
}

impl core::error::Error for ShardYmlError {}

impl core::fmt::Display for ShardYmlError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidDocument => write!(f, "Document is not to parseable"),
            Self::InvalidVersionFieldDataType => write!(f, "\"version\" field is not a string"),
            Self::MissingVersionField => write!(f, "\"version\" field not found"),
            Self::ParseYml(error) => error.fmt(f),
        }
    }
}

#[inline]
pub fn set_shard_yml_version(
    input: String,
    version: &str,
) -> Result<(bool, String), ShardYmlError> {
    let document = yaml::parse(&input).map_err(|error| ShardYmlError::ParseYml(Box::new(error)))?;

    let map = document
        .as_mapping()
        .ok_or(ShardYmlError::InvalidDocument)?;

    let version_node = map
        .get_node("version")
        .ok_or(ShardYmlError::MissingVersionField)?;

    let scalar = version_node
        .as_scalar()
        .ok_or(ShardYmlError::InvalidVersionFieldDataType)?;

    let output = yaml::replace_node(&input, scalar, version);

    let modified = output != input;

    let output = if modified {
        yaml::serialize(&output)
    } else {
        input
    };

    Ok((modified, output))
}

#[inline]
pub const fn update_lock_files(_path: &std::path::Path) -> bool {
    true
}

#[cfg(test)]
mod test_set_shard_yml_version {
    const INPUT: &str = r#"name: crystal-demo
version:          0.1.0

authors:
                    - Mads Hougesen <mads@mhouge.dk>

crystal:   ">= 1.15.1"

license:     MIT
"#;

    #[test]
    fn it_should_update_version() -> Result<(), super::ShardYmlError> {
        let version = "2025.05.23+1722";

        let new_version_line = format!("version:          {version}");

        let expected_output = INPUT.replace("version:          0.1.0", &new_version_line);

        assert!(expected_output.contains(&new_version_line));

        let (modified, output) = super::set_shard_yml_version(INPUT.to_string(), version)?;

        assert!(modified);

        assert_eq!(output, expected_output);

        Ok(())
    }

    #[test]
    fn it_support_multiline_strings() -> Result<(), super::ShardYmlError> {
        let input = INPUT.replace("version:          0.1.0", "version:\n          0.1.0");

        let version = "2025.05.23+1722";

        let new_version_line = format!("version:\n          {version}");

        let expected_output = input.replace("version:\n          0.1.0", &new_version_line);

        assert!(expected_output.contains(&new_version_line));

        let (modified, output) = super::set_shard_yml_version(input, version)?;

        assert!(modified);

        assert_eq!(output, expected_output);

        Ok(())
    }
}
