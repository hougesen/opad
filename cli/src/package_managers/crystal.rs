use crate::parsers::yaml;

#[derive(Debug)]
pub enum ShardYmlError {
    InvalidDocument,
    InvalidVersionFieldDataType,
    MissingVersionField,
}

impl core::error::Error for ShardYmlError {}

impl core::fmt::Display for ShardYmlError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidDocument => write!(f, "Document is not to parseable"),
            Self::InvalidVersionFieldDataType => write!(f, "\"version\" field is not a string"),
            Self::MissingVersionField => write!(f, "\"version\" field not found"),
        }
    }
}

#[inline]
pub fn set_shard_yml_version(
    path: &std::path::Path,
    version: &str,
) -> Result<bool, crate::error::Error> {
    let mut contents = std::fs::read_to_string(path)?;

    let document = yaml::parse(&contents)?;

    let map = document
        .as_mapping()
        .ok_or(ShardYmlError::InvalidDocument)?;

    let version_node = map
        .get_node("version")
        .ok_or(ShardYmlError::MissingVersionField)?;

    let scalar = version_node
        .as_scalar()
        .ok_or(ShardYmlError::InvalidVersionFieldDataType)?;

    let output = yaml::replace_node(&contents, scalar, version);

    let modified = output != contents;

    contents = output;

    if modified {
        yaml::save(path, &contents)?;
    }

    Ok(modified)
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
    fn it_should_update_version() -> Result<(), crate::error::Error> {
        let version = "2025.05.23+1722";

        let new_version_line = format!("version:          {version}");

        let expected_output = INPUT.replace("version:          0.1.0", &new_version_line);

        assert!(expected_output.contains(&new_version_line));

        let dir = tempfile::tempdir()?;

        let path = dir.path().join("shard.yml");

        std::fs::write(&path, INPUT)?;

        let modified = super::set_shard_yml_version(&path, version)?;

        assert!(modified);

        let output = std::fs::read_to_string(&path)?;

        assert_eq!(output, expected_output);

        Ok(())
    }

    #[test]
    fn it_support_multiline_strings() -> Result<(), crate::error::Error> {
        let input = INPUT.replace("version:          0.1.0", "version:\n          0.1.0");

        let version = "2025.05.23+1722";

        let new_version_line = format!("version:\n          {version}");

        let expected_output = input.replace("version:\n          0.1.0", &new_version_line);

        assert!(expected_output.contains(&new_version_line));

        let dir = tempfile::tempdir()?;

        let path = dir.path().join("pubspec.yaml");

        std::fs::write(&path, &input)?;
        let modified = super::set_shard_yml_version(&path, version)?;

        assert!(modified);

        let output = std::fs::read_to_string(&path)?;

        assert_eq!(output, expected_output);

        Ok(())
    }
}
