use std::path::Path;

use git2::{Diff, DiffFile};

fn diff_file_in(ancestor: impl AsRef<Path>, diff_file: &DiffFile) -> bool {
    match diff_file.path() {
        None => false,
        Some(file_path) => file_path.starts_with(ancestor)
    }
}

pub fn rule_triggered(path: &Path, diff: Diff) -> bool {
    for diff_delta in diff.deltas() {
        if diff_file_in(&path, &diff_delta.old_file()) {
            return true;
        }
        if diff_file_in(&path, &diff_delta.new_file()) {
            return true;
        }
    }
    false
}
