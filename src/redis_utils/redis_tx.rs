use crate::errors::types::ChainErrors;
use crate::types::transaction::Transaction;
use hex;
use redis::{Commands, ErrorKind};

pub fn insert_transaction(
    tx: &Transaction,
    con: &mut redis::Connection,
) -> Result<(), ChainErrors> {
    let json_data = serde_json::to_string(tx).expect("Failed to parse transaction into json");
    let tx_id = hex::encode(&tx.tx_id);
    match con.set(tx_id, json_data) {
        Ok(()) => Ok(()),
        Err(y) => Err(ChainErrors::RedisOther(format!("{:?}", y))),
    }
}

pub fn get_transaction(
    tx_id: &[u8; 32],
    con: &mut redis::Connection,
) -> Result<Transaction, ChainErrors> {
    let tx_id = hex::encode(tx_id);
    match con.get::<String, String>(tx_id) {
        Ok(x) => {
            let tx: Transaction =
                serde_json::from_str(&x).expect("Failed to parse redit output into tx");
            return Ok(tx);
        }
        Err(y) => match y.kind() {
            ErrorKind::TypeError => Err(ChainErrors::RedisNotFound),
            _ => Err(ChainErrors::RedisOther(format!("{:?}", y))),
        },
    }
}
