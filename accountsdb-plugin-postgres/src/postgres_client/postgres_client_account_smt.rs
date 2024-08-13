use {
    crate::{
        geyser_plugin_postgres::{GeyserPluginPostgresConfig, GeyserPluginPostgresError},
        postgres_client::SimplePostgresClient,
    },
    chrono::Utc,
    log::*,
    postgres::{Client, Statement},
    solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPluginError,
};

impl SimplePostgresClient {
    pub(crate) fn build_smt_tree_upsert_statement(client: &mut Client,
                                                  config: &GeyserPluginPostgresConfig,
    ) -> Result<Statement, GeyserPluginError> {
        let stmt =
            "INSERT INTO merkle_tree (slot, root_hash, updated_on) VALUES ($1, $2, $3) ON CONFLICT (slot) DO UPDATE SET root_hash = EXCLUDED.root_hash WHERE EXCLUDED.root_hash IS NOT NULL";

        let stmt = client.prepare(stmt);

        match stmt {
            Err(err) => {
                Err(GeyserPluginError::Custom(Box::new(GeyserPluginPostgresError::DataSchemaError {
                    msg: format!(
                        "Error in preparing for the merkle_tree update PostgreSQL database: ({}) host: {:?} user: {:?} config: {:?}",
                        err, config.host, config.user, config
                    ),
                })))
            }
            Ok(stmt) => Ok(stmt),
        }
    }

    pub(crate) fn update_merkle_tree_root(&mut self, slot: i64, root: String) -> Result<(), GeyserPluginError> {
        let client = self.client.get_mut().unwrap();

        let statement = &client.update_smt_tree_stmt;
        let updated_on = Utc::now().naive_utc();
        let result = client.client.execute(
            statement,
            &[
                &(slot),
                &root,
                &updated_on],
        );

        if let Err(err) = result {
            let msg = format!(
                "Failed to persist smt_proof to the PostgreSQL database. Error: {:?}",
                err);
            error!("{}", msg);
            return Err(GeyserPluginError::SMTUpdateError { msg });
        }

        Ok(())
    }
}
