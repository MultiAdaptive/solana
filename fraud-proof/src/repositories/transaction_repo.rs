use crate::common::node_error::NodeError;
use crate::models::transaction_model::TransactionRow;
use postgres::Client;

pub struct TransactionRepo<'a> {
    pub one: &'a mut Client,
}

impl<'a> TransactionRepo<'a> {
    pub fn range(&mut self, from_slot: i64, to_slot: i64) -> Result<Vec<TransactionRow>, NodeError> {
        let conn = &mut self.one;

        let tx_stmt =
            "SELECT slot, legacy_message, signatures FROM transaction WHERE slot >= $1 AND slot <= $2 ORDER BY slot ASC, write_version ASC";
        let tx_results = conn.query(tx_stmt, &[&from_slot, &to_slot]).unwrap();

        let rows = tx_results.into_iter().map(|r| {
            TransactionRow {
                slot: r.get(0),
                legacy_message: r.get(1),
                signatures: r.get(2),
            }
        }).collect();


        Ok(rows)
    }

    pub fn show(&mut self, slot: i64) -> Result<Vec<TransactionRow>, NodeError> {
        let conn = &mut self.one;

        let tx_stmt =
            "SELECT slot, legacy_message, signatures FROM transaction WHERE slot = $1 ORDER BY write_version ASC";
        let tx_results = conn.query(tx_stmt, &[&slot]).unwrap();

        let rows = tx_results.into_iter().map(|r| {
            TransactionRow {
                slot: r.get(0),
                legacy_message: r.get(1),
                signatures: r.get(2),
            }
        }).collect();

        Ok(rows)
    }
}
