use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::mpsc;

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

pub struct ChannelRegistrysToWatch {
    pub registrys_to_watch: Vec<String>
}

pub struct ChannelUpdateMetadata {
    pub new_metadata: Option<Vec<Value>>,
    pub channel_tx: Option<mpsc::Sender<ChannelRegistrysToWatch>>
}
