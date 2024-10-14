#[derive(Debug, Clone)]
pub struct Block {
    pub index: usize,
    pub hash: String,
    pub previous_hash: Option<String>,
    pub data: String,
    pub timestamp: u64,
}

pub struct Blockchain {
    chain: Vec<Block>,
}

impl Block {
    pub fn new(
        index: usize,
        hash: String,
        previous_hash: Option<String>,
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

impl Blockchain {
    pub fn new() -> Self {
        let mut chain = Vec::new();
        chain.push(Self::create_genesis_block());
        Blockchain { chain }
    }
}
