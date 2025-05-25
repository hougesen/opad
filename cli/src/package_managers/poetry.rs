#[inline]
pub fn poetry_enabled(dir: &std::path::Path) -> bool {
    dir.join("poetry.lock").exists()
}
