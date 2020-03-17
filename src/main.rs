// TODOs
//  5 - record a ref in a file listed in Config
//  6 - Make the location of the repo configurable

use std::process::{Command, Stdio};

use git2::Repository;

use config::Config;

mod config;
mod diff;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let rule_name = std::env::args().nth(1).ok_or("Must provide a rule")?;

    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    // Read in config
    let config = Config::load()?;
    let rule = config.get(&rule_name).ok_or("Rule not found inside lmfa0.toml")?;

    // TODO - figure out how to support nonbranch refs
    let base_tree = config.base(".lmfao".as_ref(), &repo)?;

    let diff = repo.diff_tree_to_workdir_with_index(Some(&base_tree), None)?;
    if diff::rule_triggered(&rule.root, diff) {
        println!("At least one path triggered");
        let pieces: Vec<_> = rule.command.split_whitespace().collect();
        if let ([bin], args) = pieces.split_at(1) {
            Command::new(bin)
                .args(args)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()?;
        } else {
            println!("No command specified for {}", &rule_name);
        }
    } else {
        println!("No changes");
    }
    Ok(())
}
