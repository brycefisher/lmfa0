use git2::Oid;

use crate::Result;

/// For dependency injection for the "real" backend client into `Client`
pub trait StorageTable {
    /// Read the git sha ("Oid") from table storage.
    fn get(&self, partition_key: &str, row_key: &str) -> Result<Option<String>>;

    /// Update or insert the provided git sha ("Oid") into table storage.
    fn upsert(&self, partition_key: &str, row_key: &str, oid: String) -> Result<()>;
}

/// High level client between lmfa0 and Azure Table Storage backend
pub struct Client<'c> {
    /// Used as row key in Table Storage
    job: String,
    /// "real" client used to interact with Azure
    table: &'c dyn StorageTable,
}

impl<'c> Client<'c> {
    /// Creates a StorageTableClient scoped to a particular branch name and job.
    pub fn new(job: impl Into<String>, table: &'c dyn StorageTable) -> Result<Client<'c>> {
        /*
        let AzureTableConfig { account, sas, table } = config.azure_table_config()?;
        let client = TableClient::azure_sas(&account, &sas)?;
        let cloud_table = CloudTable::new(client, table);
        */
        Ok(Client {
            job: job.into(),
            table,
        })
    }

    /// Fetches the Git SHA from Azure Storage Table
    pub fn get(&self, branch: impl AsRef<str>) -> Result<Option<Oid>> {
        let oid = self.table
            .get(branch.as_ref(), &self.job)?
            .map(|x| Oid::from_str(x.as_ref()));
        match oid {
            Some(result) => Ok(Some(result?)),
            None => Ok(None),
        }
    }

    /// Updates the Git SHA from Azure Storage Table
    pub fn upsert(&self, branch: impl AsRef<str>, sha: Oid) -> Result<()> {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_none() {
        // Given
        struct StorageTableClient;
        impl StorageTable for StorageTableClient {
            fn get(&self, _: &str, _: &str) -> Result<Option<String>> { Ok(None) }
            fn upsert(&self, _: &str, _: &str, _: String) -> Result<()> { todo!(); }
        }
        let client = Client::new("docs", &StorageTableClient).unwrap();

        // When
        let out = client.get("master");

        // Then
        assert!(matches!(out, Ok(None)));
    }

    #[test]
    fn get_some() {
        // Given
        struct StorageTableClient;
        impl StorageTable for StorageTableClient {
            fn get(&self, _: &str, _: &str) -> Result<Option<String>> {
                Ok(Some("65b4f6f5df0603ed4d83d648837a68160b4f3719".into()))
            }
            fn upsert(&self, _: &str, _: &str, _: String) -> Result<()> { todo!(); }
        }
        let client = Client::new("docs", &StorageTableClient).unwrap();

        // When
        let out = client.get("master");

        // Then
        assert!(matches!(out, Ok(Some(_))));
    }
}
