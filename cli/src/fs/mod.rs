use crate::package_managers::PackageManagerFile;

#[inline]
fn setup_walker(path: &std::path::Path) -> ignore::Walk {
    ignore::WalkBuilder::new(path)
        .git_ignore(true)
        .hidden(true)
        .build()
}

#[inline]
pub fn find_package_manager_files(path: &std::path::Path) -> Vec<PackageManagerFile> {
    let mut files = setup_walker(path)
        .flatten()
        .filter_map(|entry| {
            let inner = entry
                .path()
                .strip_prefix(path)
                .unwrap_or_else(|_| entry.path());

            PackageManagerFile::maybe_from_path(inner)
        })
        .filter(|p| p.package_manager.is_enabled())
        .collect::<Vec<_>>();

    files.sort_unstable();

    files
}
