use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

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
    pub fn from_str(raw_toml: impl AsRef<str>) -> Result<Config> {
        toml::from_str(raw_toml.as_ref()).map_err(From::from)
    }

    pub fn load() -> Result<Config> {
        match fs::read_to_string("lmfa0.toml") {
            Ok(raw_toml) => Config::from_str(raw_toml),
            Err(_) => Ok(Config::default()),
        }
    }

    pub fn base<'r>(&self, rule_name: &str, repo: &'r Repository) -> Result<Tree<'r>> {
        let ref_file = PathBuf::from(".lmfa0").join(rule_name);
        match fs::read_to_string(ref_file) {
            Ok(base_ref) => {
                let oid = Oid::from_str(&base_ref.trim())?;
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

    pub fn store<'r>(&self, rule: &str, repo: &'r Repository) -> Result<()> {
        let head = repo.head()?;
        let commit = head.peel_to_commit()?;
        let oid = commit.id();

        let path = PathBuf::from(".lmfa0").join(rule);
        fs::create_dir_all(".lmfa0")?;
        fs::write(path, format!("{}", oid)).map_err(From::from)
    }

    pub fn get(&self, rule: &str) -> Option<&Rule> {
        self.rules.get(rule)
    }
}
