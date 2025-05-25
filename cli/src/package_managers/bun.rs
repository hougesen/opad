#[inline]
pub fn bun_enabled(dir: &std::path::Path) -> bool {
    dir.join("bun.lock").exists() || dir.join("bun.lockb").exists()
}

#[inline]
pub fn bun_update_lock_file_command() -> std::process::Command {
    let mut cmd = std::process::Command::new("bun");
    cmd.arg("install");
    cmd
}
