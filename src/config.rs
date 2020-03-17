use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use git2::{Repository, Oid, BranchType, Tree};
use serde::Deserialize;

use crate::Result;

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub command: String,
    pub root: PathBuf,
}

#[serde(default)]
#[derive(Debug, Deserialize)]
pub struct Config {
    default_branch: String,
    rules: HashMap<String, Rule>
}

impl Default for Config {
    fn default() -> Self {
        Config {
            default_branch: "master".into(),
            rules: HashMap::new(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Config> {
        match fs::read_to_string("lmfa0.toml") {
            Ok(raw_toml) => toml::from_str(raw_toml.as_str()).map_err(From::from),
            Err(_) => Ok(Config::default()),
        }
    }

    pub fn base<'r>(&self, ref_file: &Path, repo: &'r Repository) -> Result<Tree<'r>> {
        match fs::read_to_string(ref_file) {
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

    pub fn get(&self, rule: &str) -> Option<&Rule> {
        self.rules.get(rule)
    }
}
