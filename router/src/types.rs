use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Network<'a> {
    pub name: &'a str,
    pub http: &'a str,
    pub chain_id:  u64
}
