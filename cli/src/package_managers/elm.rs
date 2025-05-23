use super::npm;

/// `elm.json` follows the same format as `package.json` (?)
#[inline]
pub fn set_elm_json_version(
    path: &std::path::Path,
    version: &str,
) -> Result<bool, crate::error::Error> {
    npm::set_package_json_version(path, version)
}

#[inline]
pub const fn update_lock_files(_dir: &std::path::Path) -> bool {
    true
}
