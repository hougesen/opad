#[inline]
pub fn yarn_enabled(dir: &std::path::Path) -> bool {
    dir.join("yarn.lock").exists()
}

#[inline]
pub fn yarn_update_lock_file_command() -> std::process::Command {
    let mut cmd = std::process::Command::new("yarn");
    cmd.arg("install");
    cmd
}
