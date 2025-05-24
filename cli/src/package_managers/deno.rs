use super::{
    npm::{self, PackageJsonError},
    run_update_lock_file_command,
};

/// `deno.json` follows the same format as `package.json` (?)
#[inline]
pub fn set_deno_json_version(
    contents: String,
    version: &str,
) -> Result<(bool, String), PackageJsonError> {
    npm::set_package_json_version(contents, version)
}

#[inline]
fn deno_update_lock_file_command() -> std::process::Command {
    let mut cmd = std::process::Command::new("deno");
    cmd.arg("install");
    cmd
}

#[inline]
pub fn update_lock_files(dir: &std::path::Path) -> std::io::Result<bool> {
    run_update_lock_file_command(deno_update_lock_file_command(), dir)
}
