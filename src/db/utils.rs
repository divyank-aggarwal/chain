use crate::types::block::Block;
use crate::utils::byte;
use crate::{
    errors::types::ChainErrors,
    types::{db::UserNonce, transaction::Transaction},
};
use sqlx::PgPool;
use std::convert::TryInto;

pub async fn get_user_nonce(pub_key: &[u8], con: &PgPool) -> Result<u32, ChainErrors> {
    match sqlx::query_as!(
        UserNonce,
        "Select nonce from users where pub_key = $1",
        pub_key
    )
    .fetch_one(con)
    .await
    {
        Ok(x) => Ok(x.nonce.try_into().unwrap()),
        Err(y) => match y {
            sqlx::Error::RowNotFound => Ok(0),
            y => Err(ChainErrors::DatabaseOther(format!("{:?}", y))),
        },
    }
}

pub async fn insert_user_nonce(
    pub_key: &[u8],
    nonce: u32,
    con: &PgPool,
) -> Result<(), ChainErrors> {
    if let Ok(nonce) = TryInto::<i32>::try_into(nonce) {
        match nonce {
            1 => match sqlx::query!(
                "Insert into users (pub_key, nonce) values ($1,$2)",
                pub_key,
                1
            )
            .execute(con)
            .await
            {
                Ok(_) => Ok(()),
                Err(y) => Err(ChainErrors::DatabaseOther(format!("{:?}", y))),
            },
            _ => match sqlx::query!(
                "Update users set nonce = $1 where pub_key = $2",
                nonce,
                pub_key
            )
            .execute(con)
            .await
            {
                Ok(_) => Ok(()),
                Err(y) => Err(ChainErrors::DatabaseOther(format!("{:?}", y))),
            },
        }
    } else {
        Err(ChainErrors::ConversionError(String::from(
            "Unable to convert nonce into u32",
        )))
    }
}

pub async fn get_transaction_from_db(
    tx_id: &[u8],
    con: &PgPool,
) -> Result<Transaction, ChainErrors> {
    let tx = sqlx::query!(
        "Select tx_id, message, pub_key, signature, nonce from transactions where tx_id = $1",
        tx_id
    )
    .fetch_one(con)
    .await;

    match tx {
        Err(y) => Err(ChainErrors::DatabaseOther(format!("{:?}", y))),
        Ok(x) => Ok(Transaction {
            tx_id: byte::demo(x.tx_id),
            message: x.message,
            pub_key: byte::demo(x.pub_key),
            signature: byte::demo(x.signature),
            nonce: TryInto::<u32>::try_into(x.nonce).unwrap(),
        }),
    }
}

pub async fn insert_transaction_into_db(tx: &Transaction, con: &PgPool) -> Result<(), ChainErrors> {
    if let Ok(nonce) = TryInto::<i32>::try_into(tx.nonce) {
        match sqlx::query!("Insert into transactions (tx_id,message,pub_key,signature,nonce) values ($1,$2,$3,$4,$5)",
&tx.tx_id[..],tx.message,&tx.pub_key[..],&tx.signature[..],nonce).execute(con).await {
    Ok(_) => Ok(()),
    Err(y) => Err(ChainErrors::DatabaseOther(format!("{:?}",y)))

}
    } else {
        Err(ChainErrors::ConversionError(String::from(
            "Unable to convert nonce into i32",
        )))
    }
}

pub async fn get_blockhash_from_db(block_no: u64, con: &PgPool) -> Result<[u8; 32], ChainErrors> {
    if let Ok(no) = TryInto::<i32>::try_into(block_no) {
        let row = sqlx::query!("Select header from blocks where block_no = $1", no)
            .fetch_one(con)
            .await;
        match row {
            Ok(x) => Ok(byte::demo(x.header)),
            Err(y) => Err(ChainErrors::DatabaseOther(format!("{:?}", y))),
        }
    } else {
        Err(ChainErrors::ConversionError(String::from(
            "Unable to convert block number into i32",
        )))
    }
}

pub async fn insert_block_into_db(block: &Block, con: &PgPool) -> Result<(), ChainErrors> {
    let mut tx = con.begin().await?;
    let user_vec: Vec<Vec<u8>> = block
        .transactions
        .iter()
        .map(|x| x.pub_key[..].to_vec())
        .collect();
    let user_vec = user_vec.as_slice();

    let nonce_vec: Vec<i32> = block
        .transactions
        .iter()
        .map(|x| TryInto::<i32>::try_into(x.nonce).unwrap())
        .collect();
    let nonce_vec = nonce_vec.as_slice();

    sqlx::query!("Insert into users (pub_key,nonce) VALUES (UNNEST($1::bytea[]), UNNEST($2::int[])) on conflict(pub_key) do update set nonce = excluded.nonce",user_vec,nonce_vec)
    .execute(&mut tx).await?;

    let tx_id_vec = block
        .transactions
        .iter()
        .map(|x| x.tx_id[..].to_vec())
        .collect::<Vec<Vec<u8>>>();
    let tx_id_vec = tx_id_vec.as_slice();

    let message_vec = block
        .transactions
        .iter()
        .map(|x| x.message.clone())
        .collect::<Vec<String>>();
    let message_vec = message_vec.as_slice();

    let signature_vec = block
        .transactions
        .iter()
        .map(|x| x.signature[..].to_vec())
        .collect::<Vec<Vec<u8>>>();
    let signature_vec = signature_vec.as_slice();

    sqlx::query!(
        "Insert into transactions (tx_id,message,pub_key,signature,nonce) VALUES"
            + "( UNNEST($1::bytea[]), "
            + "UNNEST($2::text[]), "
            + "UNNEST($3::bytea[]), "
            + "UNNEST($4::bytea[]), "
            + "UNNEST($5::int[])) ",
        tx_id_vec,
        message_vec,
        user_vec,
        signature_vec,
        nonce_vec
    )
    .execute(&mut tx)
    .await?;

    let block_no = TryInto::<i32>::try_into(block.number).or_else(|_| {
        Err(ChainErrors::ConversionError(String::from(
            "Could not parse block number into i32",
        )))
    })?;

    let nonce = TryInto::<i64>::try_into(block.nonce).or_else(|_| {
        Err(ChainErrors::ConversionError(String::from(
            "Could not parse block nonce into i64",
        )))
    })?;

    let difficulty = TryInto::<i64>::try_into(block.difficulty).or_else(|_| {
        Err(ChainErrors::ConversionError(String::from(
            "Could not parse block difficulty into i64",
        )))
    })?;

    let timestamp = TryInto::<i64>::try_into(block.timestamp).or_else(|_| {
        Err(ChainErrors::ConversionError(String::from(
            "Could not parse block timestamp into i64",
        )))
    })?;

    sqlx::query!(
        "Insert into blocks (block_no,nonce,difficulty,root,miner_pubkey,header,timestamp) VALUES"
            + "($1,$2,$3,$4,$5,$6,$7)",
        block_no,
        nonce,
        difficulty,
        &block.root[..],
        &block.miner_pubkey[..],
        &block.header[..],
        timestamp
    )
    .execute(&mut tx)
    .await?;

    let size: i32 = block.transactions.len() as i32;
    let seq: Vec<i32> = (1..(size + 1)).collect();
    let seq = seq.as_slice();
    let block_no_vec: Vec<i32> = (1..(size + 1)).map(|_| block_no).collect();
    let block_no_vec = block_no_vec.as_slice();

    sqlx::query!(
        "Insert into block_transaction (block_no, tx_id, seq) VALUES"
            + "( UNNEST($1::int[]), "
            + "UNNEST($2::bytea[]), "
            + "UNNEST($3::int[]))",
        block_no_vec,
        tx_id_vec,
        seq
    )
    .execute(&mut tx)
    .await?;
    tx.commit().await?;
    Ok(())
}
