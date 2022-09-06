Create table if not exists blocks (
    block_no integer primary key,
    nonce bigint not null,
    difficulty bigint not null,
    root bytea not null,
    miner_pubkey bytea references users(pub_key) not null,
    header bytea not null,
    timestamp bigint not null
)