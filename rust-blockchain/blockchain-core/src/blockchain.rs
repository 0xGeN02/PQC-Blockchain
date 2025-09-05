use crate::block::Block;
use crate::pow::ProofOfWork;

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        genesis_block.mine_block(1); // Genesis block with difficulty 1
        Blockchain {
            blocks: vec![genesis_block ], 
            difficulty: 4
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_hash = self.blocks.last().unwrap().hash.clone();
        let mut new_block = Block::new(self.blocks.len() as u64, data, previous_hash);
        
        println!("Mining block {}...", new_block.index);
        let start = std::time::Instant::now();

        new_block.mine_block(self.difficulty);

        let duration = start.elapsed();
        println!("Block mined: {} in {:?}", new_block.hash, duration);

        self.blocks.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];
            if current.previous_hash != previous.hash {
                return false;
            }
            if current.hash != current.calculate_hash() {
                return false;
            }
            let pow = ProofOfWork::new(current.clone(), self.difficulty);
            if !pow.validate() {
                return false;
            }
        }
        true
    }

    pub fn set_difficulty(&mut self, difficulty: usize) {
        self.difficulty = difficulty;
    }
}