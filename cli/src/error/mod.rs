#[derive(Debug)]
pub enum Error {
    Inquire(inquire::InquireError),
    Io(std::io::Error),
    MarkedYaml(Box<marked_yaml::LoadError>),
    SerdeJson(Box<serde_json::Error>),
    TomlEdit(Box<toml_edit::TomlError>),
}

impl core::error::Error for Error {}

impl core::fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Inquire(error) => error.fmt(f),
            Self::Io(error) => error.fmt(f),
            Self::MarkedYaml(error) => error.fmt(f),
            Self::SerdeJson(error) => error.fmt(f),
            Self::TomlEdit(error) => error.fmt(f),
        }
    }
}

impl From<std::io::Error> for Error {
    #[inline]
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<inquire::InquireError> for Error {
    #[inline]
    fn from(value: inquire::InquireError) -> Self {
        Self::Inquire(value)
    }
}

impl From<marked_yaml::LoadError> for Error {
    #[inline]
    fn from(value: marked_yaml::LoadError) -> Self {
        Self::MarkedYaml(Box::new(value))
    }
}

impl From<toml_edit::TomlError> for Error {
    #[inline]
    fn from(value: toml_edit::TomlError) -> Self {
        Self::TomlEdit(Box::new(value))
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(Box::new(value))
    }
}
