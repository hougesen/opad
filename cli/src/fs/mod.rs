use crate::package_managers::PackageManagerFile;

#[inline]
pub fn setup_walker(
    path: &std::path::Path,
    check_gitignored_files: bool,
    check_hidden_files: bool,
) -> ignore::Walk {
    ignore::WalkBuilder::new(path)
        .git_ignore(!check_gitignored_files)
        .hidden(!check_hidden_files)
        .build()
}

#[inline]
pub fn find_package_manager_files(
    walker: ignore::Walk,
    path: &std::path::Path,
) -> Vec<PackageManagerFile> {
    let mut files = walker
        .flatten()
        .filter_map(|entry| {
            let inner = entry
                .path()
                .strip_prefix(path)
                .unwrap_or_else(|_| entry.path());

            PackageManagerFile::maybe_from_path(inner)
        })
        .collect::<Vec<_>>();

    files.sort_unstable();

    files
}
