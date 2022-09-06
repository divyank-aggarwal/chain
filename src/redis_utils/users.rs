use crate::errors::types::ChainErrors;
use hex;
use redis::{self, Commands, ErrorKind, RedisError};

pub fn insert_user_nonce(
    user_key: [u8; 33],
    nonce: u32,
    con: &mut redis::Connection,
) -> Result<(), ChainErrors> {
    let user_key = hex::encode(user_key);
    match con.set(user_key, nonce) {
        Ok(()) => Ok(()),
        Err(y) => Err(ChainErrors::RedisOther(format!("{:?}", y))),
    }
}

pub fn get_user_nonce(user_key: [u8; 33], con: &mut redis::Connection) -> Result<u32, ChainErrors> {
    let user_key = hex::encode(user_key);
    match con.get::<String, u32>(user_key) {
        Ok(x) => Ok(x),
        Err(y) => match y.kind() {
            ErrorKind::TypeError => Err(ChainErrors::RedisNotFound),
            _ => Err(ChainErrors::RedisOther(format!("{:?}", y))),
        },
    }
}
