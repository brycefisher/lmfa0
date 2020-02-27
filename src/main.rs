use std::fs;
use std::path::Path;

use git2::{Repository,BranchType, DiffFile};
use serde::Deserialize;

fn diff_file_in(ancestor: impl AsRef<Path>, diff_file: &DiffFile) -> bool {
    match diff_file.path() {
        None => false,
        Some(file_path) => file_path.starts_with(ancestor)
    }
}

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    default_branch: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            default_branch: "master".into()
        }
    }
}

impl Config {
    pub fn load() -> Result<Config> {
        match fs::read_to_string("/etc/lmfa0/config.toml") {
            Ok(raw_toml) => toml::from_str(raw_toml.as_str()).map_err(From::from),
            Err(_) => Ok(Config::default()),
        }
    }

    pub fn base(&self) -> &str {
        self.default_branch.as_str()
    }
}

fn main() -> Result<()> {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    // Read in config
    let config = Config::load()?;

    // TODO - figure out how to support nonbranch refs
    let branch = repo.find_branch(config.base(), BranchType::Local)?;
    let r#ref = branch.into_reference();
    let tree = r#ref.peel_to_tree()?;

    let diff = repo.diff_tree_to_workdir_with_index(Some(&tree), None)?;

    let ancestor = "";
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

    Ok(())
}
