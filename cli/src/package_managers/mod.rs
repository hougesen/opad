pub use cargo::CargoTomlError;
pub use crystal::ShardYmlError;
pub use npm::PackageJsonError;
pub use pubspec::PubspecYamlError;
pub use pyproject::PyprojectTomlError;

mod cargo;
mod crystal;
mod deno;
mod elm;
mod gleam;
mod npm;
mod pubspec;
mod pyproject;

#[inline]
fn run_update_lock_file_command(
    mut command: std::process::Command,
    dir: &std::path::Path,
) -> std::io::Result<bool> {
    command
        .current_dir(dir)
        .spawn()?
        .wait()
        .map(|exit_code| exit_code.success())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PackageManager {
    CargoToml,
    DenoJson,
    ElmJson,
    GleamToml,
    PackageJson,
    PubspecYaml,
    PyProjectToml,
    ShardYml,
}

impl PackageManager {
    #[inline]
    fn maybe_from_path(path: &std::path::Path) -> Option<Self> {
        path.file_name()
            .and_then(|inner| inner.to_str())
            .and_then(|file_name| match file_name {
                "Cargo.toml" => Some(Self::CargoToml),
                "deno.json" | "deno.json5" | "deno.jsonc" => Some(Self::DenoJson),
                "elm.json" | "elm.json5" | "elm.jsonc" => Some(Self::ElmJson),
                "gleam.toml" => Some(Self::GleamToml),
                "package.json" | "package.json5" | "package.jsonc" => Some(Self::PackageJson),
                "pubspec.yaml" | "pubspec.yml" => Some(Self::PubspecYaml),
                "pyproject.toml" => Some(Self::PyProjectToml),
                "shard.yaml" | "shard.yml" => Some(Self::ShardYml),

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
    pub fn set_package_version(&self, version: &str) -> Result<bool, crate::error::Error> {
        match self.package_manager {
            PackageManager::CargoToml => cargo::set_cargo_toml_version(&self.path, version),
            PackageManager::DenoJson => deno::set_deno_json_version(&self.path, version),
            PackageManager::ElmJson => elm::set_elm_json_version(&self.path, version),
            PackageManager::GleamToml => gleam::set_gleam_toml_version(&self.path, version),
            PackageManager::PackageJson => npm::set_package_json_version(&self.path, version),
            PackageManager::PubspecYaml => pubspec::set_pubspec_version(&self.path, version),
            PackageManager::PyProjectToml => pyproject::set_version(&self.path, version),
            PackageManager::ShardYml => crystal::set_shard_yml_version(&self.path, version),
        }
    }

    #[inline]
    pub fn update_lock_files(&self) -> std::io::Result<bool> {
        let canon = self.path.canonicalize()?;

        let dir = canon.parent().unwrap_or(&self.path);

        let success = match self.package_manager {
            PackageManager::CargoToml => cargo::update_lock_files(dir)?,
            PackageManager::DenoJson => deno::update_lock_files(dir)?,
            PackageManager::ElmJson => elm::update_lock_files(dir),
            PackageManager::GleamToml => gleam::update_lock_files(dir),
            PackageManager::PackageJson => npm::update_lock_files(dir)?,
            PackageManager::PubspecYaml => pubspec::update_lock_files(dir),
            PackageManager::PyProjectToml => pyproject::update_lock_files(dir)?,
            PackageManager::ShardYml => crystal::update_lock_files(dir),
        };

        Ok(success)
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
            (PackageManager::ElmJson, std::path::Path::new("elm.json")),
            (
                PackageManager::GleamToml,
                std::path::Path::new("gleam.toml"),
            ),
            (
                PackageManager::PackageJson,
                std::path::Path::new("package.json"),
            ),
            (
                PackageManager::PubspecYaml,
                std::path::Path::new("pubspec.yaml"),
            ),
            (
                PackageManager::PyProjectToml,
                std::path::Path::new("pyproject.toml"),
            ),
            (PackageManager::ShardYml, std::path::Path::new("shard.yml")),
        ];

        for (package_manager, path) in expected_results {
            let result = PackageManager::maybe_from_path(path).unwrap();

            assert_eq!(package_manager, result);
        }
    }
}
