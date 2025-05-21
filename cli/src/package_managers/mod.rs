mod cargo;
mod deno;
mod npm;
mod pyproject;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PackageManager {
    CargoToml,

    DenoJson,

    PackageJson,

    PyProject,
}

impl PackageManager {
    #[inline]
    fn maybe_from_path(path: &std::path::Path) -> Option<Self> {
        path.file_name()
            .and_then(|inner| inner.to_str())
            .and_then(|file_name| match file_name {
                "Cargo.toml" => Some(Self::CargoToml),

                "deno.json" => Some(Self::DenoJson),

                "package.json" => Some(Self::PackageJson),

                "pyproject.toml" => Some(Self::PyProject),

                _ => None,
            })
    }
}

#[derive(Debug, Clone)]
pub struct PackageManagerFile {
    pub package_manager: PackageManager,

    pub path: std::path::PathBuf,
}

impl PartialEq for PackageManagerFile {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for PackageManagerFile {}

impl PartialOrd for PackageManagerFile {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.path.cmp(&other.path))
    }
}

impl Ord for PackageManagerFile {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.path.cmp(&other.path)
    }
}

impl core::fmt::Display for PackageManagerFile {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

impl PackageManagerFile {
    #[inline]
    pub fn maybe_from_path(path: &std::path::Path) -> Option<Self> {
        PackageManager::maybe_from_path(path).map(|package_manager| Self {
            package_manager,
            path: path.to_path_buf(),
        })
    }

    #[inline]
    pub fn set_package_version(&self, version: &str) -> anyhow::Result<bool> {
        match self.package_manager {
            PackageManager::CargoToml => cargo::set_cargo_toml_version(&self.path, version),

            PackageManager::DenoJson => deno::set_deno_json_version(&self.path, version),

            PackageManager::PackageJson => npm::set_package_json_version(&self.path, version),

            PackageManager::PyProject => pyproject::set_version(&self.path, version),
        }
    }
}

#[cfg(test)]
mod test_package_manager {
    use super::PackageManager;

    #[test]
    fn it_should_correctly_determine_package_manager_system() {
        let expected_results = [
            (
                PackageManager::CargoToml,
                std::path::Path::new("../Cargo.toml"),
            ),
            (PackageManager::DenoJson, std::path::Path::new("deno.json")),
            (
                PackageManager::PackageJson,
                std::path::Path::new("package.json"),
            ),
            (
                PackageManager::PyProject,
                std::path::Path::new("pyproject.toml"),
            ),
        ];

        for (package_manager, path) in expected_results {
            let result = PackageManager::maybe_from_path(path).unwrap();

            assert_eq!(package_manager, result);
        }
    }
}
