use super::transaction::Transaction;
use crate::utils::{byte::demo, merkel::get_merkel_root};
use rand::{distributions::Standard, Rng};
use sha2::{Digest, Sha256};

pub struct Block {
    pub number: u64,
    pub nonce: u64,
    pub difficulty: u64,
    pub root: [u8; 32],
    pub miner_pubkey: [u8; 33],
    pub header: [u8; 32],
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
}

const BLOCK_LIMIT: usize = 1000;

impl Block {
    pub fn is_block_valid(&self) -> bool {
        // Total transactions check
        if self.transactions.len() > BLOCK_LIMIT {
            return false;
        }

        // merkel root check
        let root = get_merkel_root(&self.transactions);
        if root != self.root {
            return false;
        }

        //TODO: check valid block number

        //TODO: timestamp check to see that block is within 2hrs of prev block

        //TODO: Check valid nonce

        // Check valid header
        // header = hash (root, pub_key, prev_block_header, nonce, difficulty, timestamp

        //TODO: actual logic for previous block header
        let prev_block_header = rand::thread_rng()
            .sample_iter(Standard)
            .take(32)
            .collect::<Vec<u8>>();
        let prev_block_header: [u8; 32] = demo(prev_block_header);
        let header_to_check = self.get_header_hash(&prev_block_header);

        /* Commented because actual logic for prev header is not there */
        // if prev_block_header != header_to_check {
        //     return false;
        // }

        true
    }
    fn get_header_hash(&self, prev_block_header: &[u8; 32]) -> [u8; 32] {
        let mut bytes_array: Vec<u8> = vec![];
        bytes_array.extend(self.root.iter());
        bytes_array.extend(self.miner_pubkey.iter());
        bytes_array.extend(prev_block_header.iter());
        bytes_array.extend(self.nonce.to_be_bytes().iter());
        bytes_array.extend(self.difficulty.to_be_bytes().iter());
        bytes_array.extend(self.timestamp.to_be_bytes().iter());
        let mut hasher = Sha256::new();
        hasher.update(&bytes_array);
        let hash = hasher
            .finalize()
            .as_slice()
            .try_into()
            .expect("Wrong Length");
        hash
    }
}
