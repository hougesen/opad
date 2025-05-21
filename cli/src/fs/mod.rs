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
    setup_walker(path)
        .flatten()
        .filter_map(|p| PackageManagerFile::maybe_from_path(p.path()))
        .collect()
}
