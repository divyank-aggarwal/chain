use crate::types::transaction::Transaction;
use hex;
use redis::Commands;

pub fn insert_transaction(tx: &Transaction, con: &mut redis::Connection) {
    let json_data = serde_json::to_string(tx).expect("Failed to parse transaction into json");
    let tx_id = hex::encode(&tx.tx_id);
    let _: () = con
        .set(tx_id, json_data)
        .expect("Failed to set transaction in db");
}

pub fn get_transaction(tx_id: &[u8; 32], con: &mut redis::Connection) -> Option<Transaction> {
    let tx_id = hex::encode(tx_id);
    match con.get::<String, String>(tx_id) {
        Ok(x) => {
            let tx: Transaction =
                serde_json::from_str(&x).expect("Failed to parse redit output into tx");
            return Some(tx);
        }
        Err(_) => None,
    }
}
