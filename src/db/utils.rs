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
            "Unable to convert nonce into u32",
        )))
    }
}
