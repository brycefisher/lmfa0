use git2::Oid;

use crate::Result;

#[derive(Debug, PartialEq)]
pub struct TableClient {
    branch: String,
    job: String,
}

impl TableClient {
    /// Creates a TableClient scoped to a particular branch name and job.
    pub fn new(branch: impl Into<String>, job: impl Into<String>) -> TableClient {
        TableClient {
            branch: branch.into(),
            job: job.into()
        }
    }

    /// Fetches the Git SHA from Azure Storage Table
    pub fn get(&self) -> Result<Oid> {
        todo!();
    }

    /// Updates the Git SHA from Azure Storage Table
    pub fn save(&self, sha: Oid) -> Result<()> {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_client() {
        assert_eq!(
            TableClient { branch: "master".into(), job: "test".into() },
            TableClient::new("master", "test")
        );
    }
}
