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

    println!("\n=== Blockchain State ===");
    for (i, block) in blockchain.blocks.iter().enumerate() {
        println!("Block {}: Hash: {} (Nonce: {})", i, block.hash, block.nonce);
    }

    println!("\n=== Validation ===");
    println!("Is Blockchain valid? {}", blockchain.is_valid());
    
    // Probar diferentes dificultades
    println!("\n=== Test Harder Difficulty ===");
    blockchain.set_difficulty(5);
    blockchain.add_block("High difficulty transaction".to_string());
}