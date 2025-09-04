use crate::block::Block;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        Blockchain {blocks: vec![genesis_block ]}
    }

    pub fn add_block(&mut self, data: String) {
        let previous_hash = self.blocks.last().unwrap().hash.clone();
        let new_block = Block::new(self.blocks.len() as u64, data, previous_hash);
        self.blocks.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];
            if current.previous_hash != previous.hash {
                return false;
            }
            let mut hasher = Sha256::new();
            hasher.update(format!("{}{}{}{}", current.index, current.timestamp, &current.data, &current.previous_hash));
            let hash = format!("{:x}", hasher.finalize());
            if current.hash != hash {
                return false;
            }
        }
        true
    }
}