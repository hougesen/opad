mod cargo;
mod npm;

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

    // cabal
    Cabal,

    // stack
    Stack,
}

impl PackageManager {
    #[inline]
    fn maybe_from_path(path: &std::path::Path) -> Option<Self> {
        if let Some(file_name) = path.file_name().and_then(|inner| inner.to_str()) {
            let pm = match file_name {
                "Cargo.toml" => Some(Self::CargoToml),
                "Cargo.lock" => Some(Self::CargoLock),

                "package.json" => Some(Self::PackageJson),
                "package-lock.json" => Some(Self::PackageLockJson),

                "pnpm-lock.yaml" => Some(Self::PnpmLockYaml),

                "yarn.lock" => Some(Self::YarnLock),

                "package.yaml" => Some(Self::Stack),

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

    #[inline]
    pub const fn is_enabled(self) -> bool {
        match self {
            Self::CargoToml => true,
            Self::CargoLock => false,

            Self::PackageJson => true,
            Self::PackageLockJson => false,

            Self::PnpmLockYaml => false,

            Self::YarnLock => false,

            Self::Cabal => false,

            Self::Stack => false,
        }
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
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.path.cmp(&other.path))
    }
}

impl Ord for PackageManagerFile {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.path.cmp(&other.path)
    }
}

impl std::fmt::Display for PackageManagerFile {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            PackageManager::CargoLock => Ok(false),

            PackageManager::PackageJson => npm::set_package_json_version(&self.path, version),
            PackageManager::PackageLockJson => Ok(false),

            PackageManager::PnpmLockYaml => Ok(false),

            PackageManager::YarnLock => Ok(false),

            PackageManager::Cabal => Ok(false),

            PackageManager::Stack => Ok(false),
        }
    }
}
