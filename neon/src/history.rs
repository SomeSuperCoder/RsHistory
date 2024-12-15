use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};

use crate::hash::compute_hash;

pub type ProofOfHistory = Vec<HistoryPart>;
pub const HASHES_PER_SLOT: usize = 350_000;

#[derive(Debug, Default, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct HistoryPart {
    pub hash: String,
    pub data: HistoryPartData
}

impl HistoryPart {
    pub fn new(data: HistoryPartData) -> Self {
        let string = serde_json::to_string(&data).unwrap();
        let hash = compute_hash(&string);

        Self {
            hash,
            data
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct HistoryPartData {
    pub prev_hash: String,
    pub content: String
}

impl HistoryPartData {
    pub fn new(prev_hash: String, content: String) -> Self {
        Self {
            prev_hash,
            content
        }
    }
}

pub fn generate_slot(base: String) -> ProofOfHistory {
    let mut history: ProofOfHistory = Vec::new();

    for i in 0..HASHES_PER_SLOT {
        let prev_hash;

        if i == 0 {
            prev_hash = base.clone();
        } else {
            prev_hash = history.get(history.len() - 1).unwrap().hash.clone();
        }

        let new_part = HistoryPart::new(
            HistoryPartData::new(
                prev_hash,
                String::new()
            )
        );

        history.push(new_part);
    }

    history
}
