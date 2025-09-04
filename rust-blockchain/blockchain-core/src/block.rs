use sha2::{Sha256, Digest};
use chrono::Utc;

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
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}{}", index, timestamp, &data, &previous_hash));
        let hash = format!("{:x}", hasher.finalize());
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
            nonce: 0,
        }
    }
}