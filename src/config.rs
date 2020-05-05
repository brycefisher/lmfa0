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
    rules: HashMap<String, Rule>,
    azure_table: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            default_branch: "master".into(),
            rules: HashMap::new(),
            azure_table: HashMap::new(),
        }
    }
}

#[derive(Debug, Deserialize, Default, PartialEq)]
pub struct AzureTableConfig {
    pub table: String,
    pub account: String,
    pub sas: String,
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

    pub fn azure_table_config(&self) -> Result<AzureTableConfig> {
        let sas = std::env::var("LMFA0_SAS")?;
        let table = self.azure_table.get("table")
            .ok_or("No table inside [azure_table]")?;
        let account = self.azure_table.get("account")
            .ok_or("No account inside [azure_table]")?;
        Ok(AzureTableConfig {
            sas,
            table: table.clone(),
            account: account.clone(),
        })
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

#[cfg(test)]
mod test {
    use super::*;
    use std::env;

    #[test]
    fn azure_table_config() {
        // Given
        env::set_var("LMFA0_SAS", "?sv=...");
        let config = Config::from_str(r##"
        [azure_table]
        account = "azure-storage-acct1"
        table = "lmfa0"
        "##).unwrap();

        // When
        let output = config.azure_table_config().unwrap();

        assert_eq!(
            output,
            AzureTableConfig {
                account: "azure-storage-acct1".into(),
                table: "lmfa0".into(),
                sas: "?sv=...".into(),
            }
        );
    }
}
