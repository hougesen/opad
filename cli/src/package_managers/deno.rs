use super::npm;

/// `deno.json` follows the same format as `package.json` (?)
#[inline]
pub fn set_deno_json_version(path: &std::path::Path, version: &str) -> anyhow::Result<bool> {
    npm::set_package_json_version(path, version)
}

#[inline]
pub fn update_lock_files(path: &std::path::Path) -> anyhow::Result<bool> {
    let exit_code = std::process::Command::new("deno")
        .arg("install")
        .current_dir(path)
        .spawn()?
        .wait()?;

    Ok(exit_code.success())
}
