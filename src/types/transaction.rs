use k256::ecdsa::{
    self,
    signature::{Signature, Verifier},
    VerifyingKey,
};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use sha2::{Digest, Sha256};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub tx_id: [u8; 32],
    pub message: String,
    #[serde(with = "BigArray")]
    pub pub_key: [u8; 33],
    #[serde(with = "BigArray")]
    pub signature: [u8; 64],
    pub nonce: u32,
}

impl Transaction {
    pub fn is_transaction_valid(&self) -> bool {
        // signature verification
        let verify_key = match VerifyingKey::from_sec1_bytes(&self.pub_key) {
            Ok(x) => x,
            Err(e) => {
                //TODO: handle error
                return false;
            }
        };
        let signature: ecdsa::Signature = Signature::from_bytes(&self.signature).unwrap();
        let msg = self.message.as_bytes();
        if !verify_key.verify(msg, &signature).is_ok() {
            return false;
        }

        //TODO: tx_id verification
        let tx_id_to_check = self.tx_id_hash();
        if tx_id_to_check != self.tx_id {
            return false;
        }

        //TODO: nonce verification

        return true;
    }

    fn tx_id_hash(&self) -> [u8; 32] {
        let mut bytes_array: Vec<u8> = vec![];
        bytes_array.extend(self.message.as_bytes().iter());
        bytes_array.extend(self.pub_key.iter());
        bytes_array.extend(self.nonce.to_be_bytes().iter());
        bytes_array.extend(self.signature.iter());
        let mut hasher = Sha256::new();
        hasher.update(bytes_array);
        let hash = hasher
            .finalize()
            .as_slice()
            .try_into()
            .expect("Length is not 32");
        hash
    }
}
