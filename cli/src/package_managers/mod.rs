mod cargo;

#[derive(Debug)]
pub enum PackageManagerFile {
    // cargo
    CargoToml(std::path::PathBuf),
    CargoLock(std::path::PathBuf),

    // npm
    PackageJson(std::path::PathBuf),
    PackageLockJson(std::path::PathBuf),

    // pnpm
    PnpmLockYaml(std::path::PathBuf),

    // yarn
    YarnLock(std::path::PathBuf),
}

impl PackageManagerFile {
    #[inline]
    pub fn maybe_from_path(path: &std::path::Path) -> Option<PackageManagerFile> {
        if let Some(file_name) = path
            .file_name()
            .map(|file_name| file_name.to_str())
            .flatten()
        {
            match file_name {
                "Cargo.toml" => Some(Self::CargoToml(path.to_path_buf())),
                "Cargo.lock" => Some(Self::CargoLock(path.to_path_buf())),

                "package.json" => Some(Self::PackageJson(path.to_path_buf())),
                "package-lock.json" => Some(Self::PackageLockJson(path.to_path_buf())),

                "pnpm-lock.yaml" => Some(Self::PnpmLockYaml(path.to_path_buf())),

                "yarn.lock" => Some(Self::YarnLock(path.to_path_buf())),

                _ => None,
            }
        } else {
            None
        }
    }

    #[inline]
    pub fn set_package_version(&self, version: &str) -> anyhow::Result<bool> {
        match self {
            Self::CargoToml(path) => cargo::set_cargo_toml_version(path, &version),
            Self::CargoLock(_path) => Ok(false),

            Self::PackageJson(_path) => Ok(false),
            Self::PackageLockJson(_path) => Ok(false),

            Self::PnpmLockYaml(_path) => Ok(false),

            Self::YarnLock(_path) => Ok(false),
        }
    }
}
