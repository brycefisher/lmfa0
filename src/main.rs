// TODOs
//  1 - parse args for path
//  2 - parse args for action
//  5 - record a ref in a file listed in Config
//  6 - Make the location of the repo configurable

use std::path::Path;

use git2::{Repository, DiffFile};

use config::Config;

mod config;

fn diff_file_in(ancestor: impl AsRef<Path>, diff_file: &DiffFile) -> bool {
    match diff_file.path() {
        None => false,
        Some(file_path) => file_path.starts_with(ancestor)
    }
}

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    // Read in config
    let config = Config::load()?;

    // TODO - figure out how to support nonbranch refs
    let base_tree = config.base(".lmfao".as_ref(), &repo)?;

    let diff = repo.diff_tree_to_workdir_with_index(Some(&base_tree), None)?;

    let ancestor = "Cargo.toml";
    for diff_delta in diff.deltas() {
        if diff_file_in(ancestor, &diff_delta.old_file()) {
            println!("At least one change in monitored path");
            return Ok(());
        }

        if diff_file_in(ancestor, &diff_delta.new_file()) {
            println!("At least one change in monitored path");
            return Ok(());
        }
    }

    println!("No changes");
    Ok(())
}
