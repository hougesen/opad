mod cargo;

#[derive(Debug, Clone, Copy)]
pub enum PackageManager {
    // cargo
    CargoToml,
    CargoLock,

    // npm
    PackageJson,
    PackageLockJson,

    // pnpm
    PnpmLockYaml,

    // yarn
    YarnLock,
}

impl PackageManager {
    #[inline]
    fn maybe_from_file_name(file_name: &str) -> Option<Self> {
        match file_name {
            "Cargo.toml" => Some(Self::CargoToml),
            "Cargo.lock" => Some(Self::CargoLock),

            "package.json" => Some(Self::PackageJson),
            "package-lock.json" => Some(Self::PackageLockJson),

            "pnpm-lock.yaml" => Some(Self::PnpmLockYaml),

            "yarn.lock" => Some(Self::YarnLock),

            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PackageManagerFile {
    pub package_manager: PackageManager,

    pub path: std::path::PathBuf,
}

impl PackageManagerFile {
    #[inline]
    pub fn maybe_from_path(path: &std::path::Path) -> Option<Self> {
        path.file_name()
            .and_then(|file_name| file_name.to_str())
            .and_then(PackageManager::maybe_from_file_name)
            .map(|package_manager| PackageManagerFile {
                package_manager,
                path: path.to_path_buf(),
            })
    }

    #[inline]
    pub fn set_package_version(&self, version: &str) -> anyhow::Result<bool> {
        match self.package_manager {
            PackageManager::CargoToml => cargo::set_cargo_toml_version(&self.path, version),
            PackageManager::CargoLock => Ok(false),

            PackageManager::PackageJson => Ok(false),
            PackageManager::PackageLockJson => Ok(false),

            PackageManager::PnpmLockYaml => Ok(false),

            PackageManager::YarnLock => Ok(false),
        }
    }
}
