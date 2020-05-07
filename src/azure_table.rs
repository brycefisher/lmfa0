use std::convert::TryInto;
use git2::Oid;

use crate::Result;

pub trait StorageTable {}

pub struct Client<'c> {
    branch: String,
    job: String,
    table: &'c dyn StorageTable,
}

impl<'c> Client<'c> {
    /// Creates a StorageTableClient scoped to a particular branch name and job.
    pub fn new(branch: impl Into<String>, job: impl Into<String>, table: impl TryInto<&'c dyn StorageTable, Error=crate::Error>) -> Result<Client<'c>> {
        /*
        let AzureTableConfig { account, sas, table } = config.azure_table_config()?;
        let client = TableClient::azure_sas(&account, &sas)?;
        let cloud_table = CloudTable::new(client, table);
        */
        Ok(Client {
            branch: branch.into(),
            job: job.into(),
            table: table.try_into()?,
        })
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
}
