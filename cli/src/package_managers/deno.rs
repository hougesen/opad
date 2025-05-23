use super::npm;

/// `deno.json` follows the same format as `package.json` (?)
#[inline]
pub fn set_deno_json_version(path: &std::path::Path, version: &str) -> anyhow::Result<bool> {
    npm::set_package_json_version(path, version)
}

#[inline]
pub fn update_lock_files(path: &std::path::Path) -> std::io::Result<bool> {
    std::process::Command::new("deno")
        .arg("install")
        .current_dir(path)
        .spawn()?
        .wait()
        .map(|exit_code| exit_code.success())
}
