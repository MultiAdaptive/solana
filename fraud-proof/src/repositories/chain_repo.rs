use diesel::prelude::*;
use diesel::row::NamedRow;
use diesel::RunQueryDsl;

use crate::common::node_error::NodeError;
use crate::entities::chain_entity::table_chain::column_slot;
use crate::entities::chain_entity::table_chain::dsl::table_chain;
use crate::models::chain_model::{ChainRecord, ChainRow};
use crate::utils::store_util::{PgConnectionPool, PooledPgConnection};
use crate::utils::uuid_util::generate_uuid;

pub struct ChainRepo {
    pub pool: Box<PgConnectionPool>,
}

impl ChainRepo {
    pub fn show(&self) -> Result<ChainRow, NodeError> {
        let conn: &mut PooledPgConnection = &mut self.pool.get()?;

        let results = table_chain
            .order(column_slot.desc())
            .limit(1)
            .load::<ChainRow>(conn)
            .expect("Error loading chain");

        if results.is_empty() {
            return Err(
                NodeError::new(generate_uuid(),
                               "Couldn't find query last slot from database".to_string(),
                )
            );
        }

        let row = results[0].clone();

        Ok(row)
    }

    pub fn upsert(&self, record: ChainRecord) -> Result<ChainRow, NodeError> {
        let conn: &mut PooledPgConnection = &mut self.pool.get()?;

        let row: ChainRow;

        let count: i64 = table_chain
            .count()
            .get_result(conn)
            .expect("Error count table chain");

        if count == 0 {
            row = diesel::insert_into(table_chain)
                .values(&record)
                .get_result::<ChainRow>(conn)?;
        } else {
            row = diesel::update(table_chain)
                .set((
                    column_slot.eq(record.column_slot),
                ))
                .get_result::<ChainRow>(conn)?;
        }

        Ok(row)
    }
}
