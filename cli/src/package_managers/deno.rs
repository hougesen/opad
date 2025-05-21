use super::npm;

/// `deno.json` follows the same format as `package.json` (?)
#[inline]
pub fn set_deno_json_version(path: &std::path::Path, version: &str) -> anyhow::Result<bool> {
    npm::set_package_json_version(path, version)
}

#[inline]
pub fn update_lock_files(_path: &std::path::Path) -> anyhow::Result<bool> {
    todo!()
}
