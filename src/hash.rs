use crate::block::Block;
use sha2::{Digest, Sha256};
pub fn calculate_hash(block: &Block) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!(
        "{}{}{}{}{}",
        block.index, block.previous_hash, block.timestamp, block.data, block.nonce
    ));
    format!("{:x}", hasher.finalize())
}
