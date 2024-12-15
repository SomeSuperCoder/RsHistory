use borsh::{BorshDeserialize, BorshSerialize};
use crypto::KeyPair;
use serde::{Deserialize, Serialize};

use crate::history::ProofOfHistory;

#[derive(Debug, Default, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct Block {
    pub author: String,
    pub signature: Vec<u8>,
    pub inner: BlockInnerData
}

impl Block {
    pub fn new(history: ProofOfHistory, height: usize, key_pair: &mut KeyPair) -> Self {        
        let latest_hash = history.last().unwrap().hash.clone();

        let author = key_pair.get_address();
        let inner: BlockInnerData = BlockInnerData {
            height: height.clone(),
            history
        };

        let sign_taregt = format!("{}_{}_{}", author, height, latest_hash);
        let signature = key_pair.sign(sign_taregt.as_bytes());

        Self {
            author,
            signature,
            inner
        }
    }
}


#[derive(Debug, Default, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct BlockInnerData {
    pub height: usize,
    pub history: ProofOfHistory
}
