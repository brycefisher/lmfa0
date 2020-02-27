use std::path::Path;

use git2::{Repository,BranchType, DiffFile};

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

    let branch = repo.find_branch("master", BranchType::Local)?;
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
