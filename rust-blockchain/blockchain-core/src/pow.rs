use sha2::{Sha256, Digest};
use crate::block::Block;

pub struct ProofOfWork {
    pub block: Block,
    pub difficulty: usize,
}
