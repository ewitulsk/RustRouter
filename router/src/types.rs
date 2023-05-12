use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct NetworkReference<'a> {
    pub name: &'a str,
    pub http: &'a str,
    pub chain_id:  u64
}

#[derive(Clone)]
pub struct Network {
    pub name: String,
    pub http: String,
    pub chain_id: u64
}

