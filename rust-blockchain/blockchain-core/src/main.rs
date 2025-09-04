mod block;
mod blockchain;
mod pow;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("Alice pays Bob 10 coins".to_string());
    blockchain.add_block("Bob pays Charlie 5 coins".to_string());

    println!("{:#?}", blockchain);
    println!("Blockchain valid? {}", blockchain.is_valid());
}