use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ValidatorConfig {
    pub id: String,
    pub seed_phrase: String
}
