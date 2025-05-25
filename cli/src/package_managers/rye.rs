#[inline]
pub fn rye_update_lock_file_command() -> std::process::Command {
    let mut cmd = std::process::Command::new("rye");
    cmd.arg("lock");
    cmd
}

#[inline]
pub fn rye_enabled(dir: &std::path::Path) -> bool {
    dir.join("requirements.lock").exists() || dir.join("requirements-dev.lock").exists()
}
