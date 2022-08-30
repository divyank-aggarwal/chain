use k256::ecdsa::{
    self,
    signature::{Signature, Verifier},
    VerifyingKey,
};
#[derive(Clone)]
pub struct Transaction {
    pub tx_id: [u8; 32],
    pub message: String,
    pub pub_key: [u8; 33],
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
        // tx_id = hash(message, pub_key, nonce)

        //TODO: nonce verification

        return true;
    }
}
