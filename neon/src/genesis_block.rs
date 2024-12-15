use crate::{block::{Block, BlockInnerData}, history::generate_slot};

pub fn get_genesis_block() -> Block {
    Block {
        author: "DsaSdnhyKdJAToyeBi8eZZuGrEJTAQ1ddtGoym2ArAw5".to_string(),
        signature: vec![104,25,191,204,208,73,19,234,72,93,83,19,190,136,206,13,25,165,154,218,160,24,157,242,26,28,140,142,176,143,119,4,211,103,12,146,151,69,160,17,244,231,240,232,2,215,19,163,49,39,233,166,44,96,97,223,220,19,242,81,84,19,224,15],
        inner: BlockInnerData {
            height: 0,
            history: generate_slot("UeckerAllen-MichaelJames".to_string())
        }
    }
}
