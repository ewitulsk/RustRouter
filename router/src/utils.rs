use std::{fs, hash::Hash};
use std::collections::HashMap;

use anyhow::{Result, anyhow, Ok};

use crate::types::Network;

pub fn get_network<'a>(name: String) -> Result<Network<'a>> {
    let data: String = fs::read_to_string("../networks.json").expect("Failed to read file");
    let networks: HashMap<String, Network> = serde_json::from_str(&data).unwrap();

    if !networks.contains_key(&name){
        return Err(anyhow!("Network does not exist"));
    }

    let networks_clone: HashMap<String, Network> = networks.clone();

    Ok(networks_clone[&name])
}