#[derive(Debug, Clone)]
pub struct Block {
    pub index: usize,
    pub hash: String,
    pub previous_hash: String,
    pub data: String,
    pub timestamp: u64,
}

impl Block {
    pub fn new(
        index: usize,
        hash: String,
        previous_hash: String,
        data: String,
        timestamp: u64,
    ) -> Self {
        Self {
            index,
            hash,
            previous_hash,
            data,
            timestamp,
        }
    }
}
