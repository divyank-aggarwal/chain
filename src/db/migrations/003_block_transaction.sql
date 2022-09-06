Create table if not exists block_transaction (
    id serial primary key,
    block_no integer references blocks(block_no) not null,
    tx_id bytea references transactions(tx_id) not null,
    seq integer not null
)