use std::process::{self, Command};

use git2::Repository;

use config::Config;

mod azure_table;
mod config;
mod diff;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    // Read in 1 required argument -- the name of a rule to execute
    let rule_name = std::env::args()
        .nth(1)
        .ok_or("Must provide a rule")?;

    // Read in config
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let config = Config::load()?;
    let rule = config.get(&rule_name).ok_or("Rule not found inside lmfa0.toml")?;

    // Determine our last successful run for this rule
    let base_tree = config.base(&rule_name, &repo)?;

    // See if any changes have happened in our "root" since last change
    let diff = repo.diff_tree_to_workdir_with_index(Some(&base_tree), None)?;
    if diff::rule_triggered(&rule.root, diff) {
        eprintln!("At least one path triggered");
        let pieces: Vec<_> = rule.command.split_whitespace().collect();
        if let ([bin], args) = pieces.split_at(1) {
            let status = Command::new(bin).args(args).status()?;
            if status.success() {
                config.store(&rule_name, &repo)?;
            }
            if let Some(code) = status.code() {
                process::exit(code);
            }
        } else {
            eprintln!("No command specified for {}", &rule_name);
        }
    } else {
        eprintln!("No changes");
    }
    Ok(())
}
