mod cabal;
mod cargo;
mod npm;
mod pyproject;
mod stack;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PackageManager {
    // cargo
    CargoToml,

    // npm
    PackageJson,

    // cabal
    Cabal,

    // stack
    Stack,

    // python
    PyProject,
}

impl PackageManager {
    #[inline]
    fn maybe_from_path(path: &std::path::Path) -> Option<Self> {
        if let Some(file_name) = path.file_name().and_then(|inner| inner.to_str()) {
            let pm = match file_name {
                "Cargo.toml" => Some(Self::CargoToml),

                "package.json" => Some(Self::PackageJson),

                "package.yaml" => Some(Self::Stack),

                "pyproject.toml" => Some(Self::PyProject),

                _ => None,
            };

            if let Some(pm) = pm {
                return Some(pm);
            }
        }

        if let Some(ext) = path.extension().and_then(|inner| inner.to_str()) {
            let pm = match ext {
                "cabal" => Some(Self::Cabal),

                _ => None,
            };

            if let Some(pm) = pm {
                return Some(pm);
            }
        }

        None
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

            PackageManager::PackageJson => npm::set_package_json_version(&self.path, version),

            PackageManager::Cabal => cabal::set_version(&self.path, version),

            PackageManager::Stack => stack::set_version(&self.path, version),

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
            (
                PackageManager::Stack,
                std::path::Path::new("cli/package.yaml"),
            ),
            (
                PackageManager::PackageJson,
                std::path::Path::new("package.json"),
            ),
            (
                PackageManager::Cabal,
                std::path::Path::new("../../crosspmv.cabal"),
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
