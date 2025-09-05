mod block;
mod blockchain;
mod pow;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    
    println!("=== Blockchain (POW) ===");
    
    blockchain.add_block("Alice pays Bob 10 coins".to_string());
    blockchain.add_block("Bob pays Charlie 5 coins".to_string());
    blockchain.add_block("Charlie pays Dave 3 coins".to_string());
    blockchain.add_block("Dave pays Eve 2 coins".to_string());
    blockchain.add_block("Eve pays Frank 1 coin".to_string());

    println!("\n=== Blockchain State ===");
    for block in &blockchain.blocks {
        println!(
            "Block {}: Difficulty={}, Hash={}, Nonce={}", 
            block.index, 
            block.difficulty,
            &block.hash,
            block.nonce
        );
    }

    println!("\n=== Validation ===");
    println!("Is Blockchain valid? {}", blockchain.is_valid());

    println!("{:#?}", blockchain);

    let (min_diff, max_diff, avg_diff) = blockchain.get_difficulty_stats();
    println!("Difficulty stats: Min={}, Max={}, Avg={:.2}", min_diff, max_diff, avg_diff);
}