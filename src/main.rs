mod types;
mod utils;

use k256::ecdsa::{signature::Signer, Signature, SigningKey};
use k256::ecdsa::{signature::Verifier, VerifyingKey};
use rand::{
    distributions::{Alphanumeric, Standard},
    Rng,
};
use rand_core::OsRng;
use redis::Commands;
use std::convert::TryInto;
use types::transaction::Transaction;
use utils::merkel;

use crate::types::block::Block;
use crate::utils::byte::demo;

fn main() {
    // transaction test
    let mut txs: Vec<Transaction> = vec![];
    for ctr in 0..7 {
        let tx_id = rand::thread_rng()
            .sample_iter(Standard)
            .take(32)
            .collect::<Vec<u8>>();
        let priv_key = SigningKey::random(&mut OsRng);
        let pub_key = priv_key.verifying_key().to_bytes();
        let message_str: String = rand::thread_rng()
            .sample_iter(Alphanumeric)
            .take(20)
            .map(char::from)
            .collect();
        let message_bytes = message_str.as_bytes();
        let signature: Signature = priv_key.sign(message_bytes);
        println!("{:?}", signature);
        let signature: [u8; 64] = demo(signature.to_vec());
        let tx_id = demo(tx_id);
        let pub_key = demo(pub_key.to_vec());
        txs.push(Transaction {
            tx_id,
            pub_key,
            nonce: rand::thread_rng().gen_range(0..32000),
            message: message_str,
            signature,
        });
        println!("tx {}: Validity: {}", ctr, txs[ctr].is_transaction_valid());
    }
    let root = merkel::get_merkel_root(&txs);
    println!("{:?}", root);

    //block test

    let miner_pub_key = rand::thread_rng()
        .sample_iter(Standard)
        .take(33)
        .collect::<Vec<u8>>();
    let miner_pub_key = demo(miner_pub_key);
    let header = rand::thread_rng()
        .sample_iter(Standard)
        .take(32)
        .collect::<Vec<u8>>();
    let header = demo(header);
    let block = Block {
        number: rand::thread_rng().gen_range(0..32000),
        nonce: rand::thread_rng().gen_range(0..32000),
        difficulty: rand::thread_rng().gen_range(0..32000),
        root,
        transactions: txs,
        timestamp: rand::thread_rng().gen_range(0..32000),
        miner_pubkey: miner_pub_key,
        header,
    };
    println!("{}", block.is_block_valid());

    let signing_key = SigningKey::random(&mut OsRng);

    println!("{:?}", signing_key.to_bytes());
    let message = b"Whatever Message";
    let signature: Signature = signing_key.sign(message);
    println!("{:?}", signature.to_string());
    println!("{}", signature.to_string().len());
    let verify_key = VerifyingKey::from(&signing_key);
    assert!(verify_key.verify(message, &signature).is_ok());
    println!("{:?}", test_redis());
}

fn test_redis() -> redis::RedisResult<isize> {
    let client = redis::Client::open("redis://127.0.0.1:6379")?;
    let mut con = client.get_connection()?;
    let _: () = con.set("my_key", 42)?;
    con.get("my_key")
}
