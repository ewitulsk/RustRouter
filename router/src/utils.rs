use std::io::BufWriter;
use std::{fs, hash::Hash};
use std::collections::HashMap;

use serde::{Serialize, Deserialize, Deserializer};
use anyhow::{Result, anyhow, Ok};
use reqwest::blocking::{Client, Response};
use aptos_sdk::types::network_address;
use serde_json::json;

use crate::pairs::{PairTypes, Pair};
use crate::types::{Network, NetworkReference};

pub fn read_pair_descriptors() -> Vec<PairTypes> {
    let data: String = fs::read_to_string("descriptors.json").expect("Failed to read file");
    let descriptors: Vec<PairTypes> = serde_json::from_str(&data).unwrap();

    return descriptors;
}

pub fn write_pair_descriptors(pairs: Vec<PairTypes>) {
    let data = json!(pairs);

    let json_str = data.to_string();

    fs::write("descriptors.json", json_str);

}

pub fn string_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<u64>().map_err(serde::de::Error::custom)
}

pub fn query_aptos_events_raw(
    network_address: &str,
    account: &str,
    event: &str
) -> String {

    // https://fullnode.mainnet.aptoslabs.com/v1 <-- Network Address
    // /accounts/
    // 0xc7efb4076dbe143cbcd98cfaaa929ecfc8f299203dfff63b95ccb6bfe19850fa <-- Account
    // /events/
    // 0xc7efb4076dbe143cbcd98cfaaa929ecfc8f299203dfff63b95ccb6bfe19850fa::swap::SwapInfo/pair_created <-- Event (location and field)

    let mut query = String::new();
    query.push_str(network_address);
    query.push_str("/accounts/");
    query.push_str(account);
    query.push_str("/events/");
    query.push_str(event);

    let client = Client::new();

    let resp: Response = client.get(query).send().unwrap();
    let mut body: String = String::new();
    if resp.status().is_success() {
        body = resp.text().unwrap();
        // println!("{}", body);
    }
    else {
        println!("Faild with status code: {}", resp.status());
    }
    return body;
   
}

pub fn get_network(name: String) -> Result<Network> {
    let data: String = fs::read_to_string("networks.json").expect("Failed to read file");

    let networks_as_reference: HashMap<String, NetworkReference> = serde_json::from_str(&data).unwrap();

    if !networks_as_reference.contains_key(&name){
        return Err(anyhow!("Network does not exist"));
    }

    let mut networks: HashMap<String, Network> = HashMap::new();
    for (name_ref, data) in networks_as_reference.iter() {
        let network = Network {
            name: String::from(data.name),
            http: String::from(data.http),
            chain_id: data.chain_id
        };
        let name = String::from(name_ref);

        networks.insert(name, network);
    }


    Ok(networks.get(&name).unwrap().clone())
}