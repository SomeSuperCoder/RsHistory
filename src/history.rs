use serde::{Serialize, Deserialize};

use crate::hash::compute_hash;

#[derive(Debug)]
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

#[derive(Debug, Serialize, Deserialize)]
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
