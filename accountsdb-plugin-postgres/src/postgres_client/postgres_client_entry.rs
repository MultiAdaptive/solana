use {
    chrono::Utc,
    crate::{
        geyser_plugin_postgres::{GeyserPluginPostgresConfig, GeyserPluginPostgresError},
        postgres_client::{SimplePostgresClient, UpdateEntryRequest},
    },
    log::*,
    postgres::{Client, Statement},
    solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPluginError,
};
use solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaEntryInfoV2;

#[derive(Clone, Debug)]
pub struct DbEntryInfo {
    pub slot: i64,
    pub index: i64,
    pub num_hashes: i64,
    pub hash: Vec<u8>,
    pub executed_transaction_count: i64,
    pub starting_transaction_index: i64,
}


impl<'a> From<&ReplicaEntryInfoV2<'a>> for DbEntryInfo {
    fn from(entry_info: &ReplicaEntryInfoV2) -> Self {
        Self {
            slot: entry_info.slot as i64,
            index: entry_info.index as i64,
            num_hashes: entry_info.num_hashes as i64,
            hash: entry_info.hash.to_vec(),
            executed_transaction_count: entry_info.executed_transaction_count as i64,
            starting_transaction_index: entry_info.starting_transaction_index as i64,
        }
    }
}


impl SimplePostgresClient {
    pub(crate) fn build_entry_upsert_statement(
        client: &mut Client,
        config: &GeyserPluginPostgresConfig,
    ) -> Result<Statement, GeyserPluginError> {
        let stmt =
            "INSERT INTO entry (slot, entry_index, num_hashes, entry, executed_transaction_count, starting_transaction_index, updated_on) \
        VALUES ($1, $2, $3, $4, $5, $6, $7)";

        let stmt = client.prepare(stmt);

        match stmt {
            Err(err) => {
                Err(GeyserPluginError::Custom(Box::new(GeyserPluginPostgresError::DataSchemaError {
                    msg: format!(
                        "Error in preparing for the entry update PostgreSQL database: ({}) host: {:?} user: {:?} config: {:?}",
                        err, config.host, config.user, config
                    ),
                })))
            }
            Ok(stmt) => Ok(stmt),
        }
    }

    pub(crate) fn update_entry_impl(
        &mut self,
        entry_info: UpdateEntryRequest,
    ) -> Result<(), GeyserPluginError> {
        let client = self.client.get_mut().unwrap();
        let statement = &client.update_entry_stmt;
        let client = &mut client.client;

        let updated_on = Utc::now().naive_utc();

        let entry_info = entry_info.entry_info;

        let result = client.execute(
            statement,
            &[
                &(entry_info.slot),
                &(entry_info.index),
                &(entry_info.num_hashes),
                &(entry_info.hash),
                &(entry_info.executed_transaction_count),
                &(entry_info.starting_transaction_index),
                &updated_on,
            ],
        );
        if let Err(err) = result {
            let msg = format!(
                "Failed to persist entry to the PostgreSQL database. Error: {:?}",
                err
            );
            error!("{}", msg);
            return Err(GeyserPluginError::EntryUpdateError { msg });
        }

        Ok(())
    }
}
