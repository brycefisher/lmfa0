// TODOs
//  1 - parse args for path
//  2 - parse args for action
//  5 - record a ref in a file listed in Config
//  6 - Make the location of the repo configurable

use std::fs;
use std::path::{Path, PathBuf};

use git2::{Repository, Oid, BranchType, DiffFile, Tree};
use serde::Deserialize;

fn diff_file_in(ancestor: impl AsRef<Path>, diff_file: &DiffFile) -> bool {
    match diff_file.path() {
        None => false,
        Some(file_path) => file_path.starts_with(ancestor)
    }
}

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[serde(default)]
#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    default_branch: String,
    ref_file: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            default_branch: "master".into(),
            ref_file: PathBuf::from(".lmfa0-ref"),
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

    pub fn base<'r>(&self, repo: &'r Repository) -> Result<Tree<'r>> {
        match fs::read_to_string(&self.ref_file) {
            Ok(base_ref) => {
                let oid = Oid::from_str(&base_ref)?;
                let commit = repo.find_commit(oid)?;
                Ok(commit.tree()?)
            }
            Err(_) => {
                let branch = repo.find_branch(&self.default_branch, BranchType::Local)?;
                let r#ref = branch.into_reference();
                Ok(r#ref.peel_to_tree()?)
            }
        }
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
    let base_tree = config.base(&repo)?;

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
