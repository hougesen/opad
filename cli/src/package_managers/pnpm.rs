#[inline]
pub fn pnpm_enabled(dir: &std::path::Path) -> bool {
    dir.join("pnpm-lock.yaml").exists()
        || dir.join("pnpm-lock.yml").exists()
        || dir.join("pnpm-workspace.yaml").exists()
        || dir.join("pnpm-workspace.yml").exists()
}

#[inline]
pub fn pnpm_update_lock_file_command() -> std::process::Command {
    let mut cmd = std::process::Command::new("pnpm");
    cmd.arg("install");
    cmd
}
