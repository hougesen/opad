use std::env::current_dir;

use fs::find_package_manager_files;

mod fs;
mod package_managers;

#[inline]
fn _main() -> anyhow::Result<()> {
    let files = find_package_manager_files(current_dir()?.as_path());

    let version = "0.0.0";

    let results = files
        .into_iter()
        .map(|pm| {
            let result = pm.set_package_version(version);
            (pm, result)
        })
        .collect::<Vec<_>>();

    println!("results: {results:#?}");

    Ok(())
}

fn main() {
    if let Err(error) = _main() {
        eprintln!("{error}");
        std::process::exit(0)
    }
}
