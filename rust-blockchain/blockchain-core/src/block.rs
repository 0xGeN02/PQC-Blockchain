use sha2::{Sha256, Digest};
use chrono::Utc;
use crate::pow::ProofOfWork;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(), // Calculated via PoW
            nonce: 0,
        }
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let mut pow = ProofOfWork::new(self.clone(), difficulty);
        let (nonce, hash) = pow.mine_block();
        self.nonce = nonce;
        self.hash = hash;
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}{}{}", 
            self.index, 
            self.timestamp, 
            &self.data, 
            &self.previous_hash, 
            self.nonce
        ));
        format!("{:x}", hasher.finalize())
    }
}