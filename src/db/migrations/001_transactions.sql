Create table if not exists trasactions (
    tx_id bytea primary key,
    message text not null,
    pub_key bytea references users(pub_key) not null,
    signature bytea not null,
    nonce integer not null
)