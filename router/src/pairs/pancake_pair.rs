use super::{Descriptor, Pair};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PancakePair {
    pub base: Pair,
    pub unique_field: u64
}

// impl Descriptor for PancakePair {
//     fn to_json(&self) -> String {
//         String::new()
//     }
// }