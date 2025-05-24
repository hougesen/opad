use super::{PackageJsonError, npm};

/// `elm.json` follows the same format as `package.json` (?)
#[inline]
pub fn set_elm_json_version(
    contents: String,
    version: &str,
) -> Result<(bool, String), PackageJsonError> {
    npm::set_package_json_version(contents, version)
}

#[inline]
pub const fn update_lock_files(_dir: &std::path::Path) -> bool {
    true
}
