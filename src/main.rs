mod db;
mod errors;
mod redis_utils;
mod types;
mod utils;

use db::connection::{create_connection_string, DbState};
use db::utils::get_user_nonce;
use k256::ecdsa::{signature::Signer, Signature, SigningKey};
use k256::ecdsa::{signature::Verifier, VerifyingKey};
use rand::{
    distributions::{Alphanumeric, Standard},
    Rng,
};
use rand_core::OsRng;
use redis_utils::{connection, redis_tx};
use sqlx::postgres::PgPoolOptions;
use types::block::Block;
use types::transaction::Transaction;
use utils::byte::demo;
use utils::merkel;

#[actix_web::main]
async fn main() {
    many_tests();
    let conn_url = create_connection_string();
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&conn_url)
        .await
        .expect("Please pass");
    let db_pool = DbState { db_pool };
    let test_key: [u8; 1] = [39];
    println!("{}", hex::encode(test_key));
    let get_nonce = get_user_nonce(&test_key, &db_pool.db_pool).await;
    match get_nonce {
        Ok(x) => println!("{}", x),
        Err(y) => {
            println!("{:?}", y);
        }
    }
    match db::utils::insert_user_nonce(&test_key, 5, &db_pool.db_pool).await {
        Ok(_) => {}
        Err(y) => {
            println!("{:?}", y);
        }
    }

    let test_key2: [u8; 3] = [41, 56, 21];
    match db::utils::insert_user_nonce(&test_key2, 2, &db_pool.db_pool).await {
        Ok(_) => {}
        Err(y) => {
            println!("{:?}", y);
        }
    }
    sqlx::migrate!("./src/db/migrations")
        .run(&db_pool.db_pool)
        .await
        .expect("Migrations did not run!")
}

fn many_tests() {
    // transaction test
    let mut txs: Vec<Transaction> = vec![];
    let mut con = connection::connect();
    connection::test_redis(&mut con);
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

        //redis test
        redis_tx::insert_transaction(&txs[ctr], &mut con);
        let get_back = redis_tx::get_transaction(&tx_id, &mut con);
        println!("{:?}", get_back);
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
    let random_key = rand::thread_rng()
        .sample_iter(Standard)
        .take(32)
        .collect::<Vec<u8>>();
    let random_key: [u8; 32] = demo(random_key);
    redis_tx::get_transaction(&random_key, &mut con);
}
