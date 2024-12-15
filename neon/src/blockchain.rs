use crate::{block::Block, genesis_block::get_genesis_block};

#[derive(Default)]
pub struct Blockchain {
    pub blocks: Vec<Block>
}

impl Blockchain {
    pub fn init() -> Self {
        Self {
            blocks: vec![get_genesis_block()]
        }
    }

    pub fn get_latest_block(&self) -> Block {
        let index = self.blocks.len()-1;

        self.blocks.get(index).expect("Fatal error: failed getting latest block from chain").clone()
    }
}
