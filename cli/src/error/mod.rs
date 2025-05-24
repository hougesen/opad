use crate::package_managers::error::PackageManagerError;

#[derive(Debug)]
pub enum Error {
    Inquire(inquire::InquireError),
    Io(std::io::Error),
    PackageManager(PackageManagerError),
}

impl core::error::Error for Error {}

impl core::fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Inquire(error) => error.fmt(f),
            Self::Io(error) => error.fmt(f),
            Self::PackageManager(error) => error.fmt(f),
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

impl From<PackageManagerError> for Error {
    fn from(value: PackageManagerError) -> Self {
        Self::PackageManager(value)
    }
}
