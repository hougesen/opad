#[inline]
pub fn uv_enabled(dir: &std::path::Path) -> bool {
    dir.join("uv.lock").exists()
}

#[inline]
pub fn uv_update_lock_file_command() -> std::process::Command {
    let mut cmd = std::process::Command::new("uv");
    cmd.arg("lock");
    cmd
}
