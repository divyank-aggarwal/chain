use crate::types::transaction::Transaction;
use rs_merkle::{algorithms::Sha256, MerkleTree};

pub fn get_merkel_root(txs: &Vec<Transaction>) -> [u8; 32] {
    let leaves = txs.iter().map(|x| x.tx_id).collect::<Vec<[u8; 32]>>();
    let merkel_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
    let merkel_root = merkel_tree.root().ok_or("How sad").unwrap();
    merkel_root
}
