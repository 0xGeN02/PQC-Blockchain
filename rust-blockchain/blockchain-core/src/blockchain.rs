use crate::block::Block;
use crate::pow::ProofOfWork;

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string(), 2);
        genesis_block.mine_block(); 
        Blockchain {
            blocks: vec![genesis_block ], 
            difficulty: 4
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_hash = self.blocks.last().unwrap().hash.clone();
        let current_difficulty = self.calculate_next_difficulty();
        let mut new_block = Block::new(
            self.blocks.len() as u64, 
            data, 
            previous_hash,
            current_difficulty
        );
        
        println!("Mining block {}...", new_block.index);
        let start = std::time::Instant::now();

        new_block.mine_block();

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


    fn calculate_next_difficulty(&self) -> usize {
        // Simple example: increase difficulty every 2 blocks
        if self.blocks.len() % 2 == 0 && self.blocks.len() > 0 {
            let last_difficulty = self.blocks.last().unwrap().difficulty;
            std::cmp::min(last_difficulty + 1, 6) // Cap at difficulty 6
        } else {
            self.difficulty
        }
    }

    pub fn get_difficulty_stats(&self) -> (usize, usize, f64) {
        let difficulties: Vec<usize> = self.blocks.iter().map(|b| b.difficulty).collect();
        let min_diff = *difficulties.iter().min().unwrap_or(&0);
        let max_diff = *difficulties.iter().max().unwrap_or(&0);
        let avg_diff = difficulties.iter().sum::<usize>() as f64 / difficulties.len() as f64;
        (min_diff, max_diff, avg_diff)
    }
}