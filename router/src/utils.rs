use std::io::BufWriter;
use std::{fs, hash::Hash};
use std::collections::HashMap;

use serde::{Serialize, Deserialize, Deserializer};
use anyhow::{Result, anyhow, Ok};
use reqwest::blocking::{Client, Response};
use aptos_sdk::types::network_address;
use serde_json::{json, Value};

use crate::pairs::pancake_pair::{PancakePair, PancakeMetadata, pancake_from_value_descriptor, self};
use crate::pairs::{Pair, self, Descriptor};
use crate::types::{Network, NetworkReference};

pub fn decimal_to_u64(float_val: f64, decimals: i32) -> u64 {
    let multiplier = 10f64.powi(decimals);
    return (float_val * multiplier) as u64
} 

pub fn u64_to_decimal(val: u64, decimals: i32) -> f64 {
    let float_val = val as f64;
    let divisor = 10f64.powi(decimals);
    return (float_val/divisor);
}

// pub fn read_pair_descriptors() -> Vec<PairTypes> {
//     let data: String = fs::read_to_string("descriptors.json").expect("Failed to read file");
//     let value_descriptors: Vec<Value> = serde_json::from_str(&data).unwrap();
//     let mut typed_pairs:Vec<PairTypes> = Vec::new();


//     for value_descriptor in value_descriptors {
//         let protocol = &*value_descriptor.get("protocol").unwrap().as_str().unwrap();
//         match protocol {
//             "pancake" => {
//                 typed_pairs.push(
//                     pancake_from_value_descriptor(value_descriptor)
//                 )
//             }
//             _ => {
//                 println!("Unknown Pair");
//             }
//         }
//     }


//     return typed_pairs;
// }

pub fn write_pair_descriptors(pairs: &Vec<Box<dyn Pair>>) {
    let mut pair_descriptors:Vec<Box<dyn Descriptor>> = Vec::new();
    for pair in pairs {
        let protocol: &str = &*pair.get_protocol();
        match protocol {
            "pancake" => {
                let pancake_pair = pair.as_any().downcast_ref::<PancakePair>().unwrap();
                let descriptor = (*pancake_pair).get_descriptor();
                pair_descriptors.push(descriptor);
            }

            &_ => {
                
            }
        }
        
        
    }

    let data = json!(pair_descriptors);

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

pub fn string_to_u128<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<u128>().map_err(serde::de::Error::custom)
}

pub fn query_aptos_events_raw(
    network_address: &str,
    account: &str,
    event: &str,
    start: u64,
    limit: u64
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
    query.push_str("?start=");
    query.push_str(start.to_string().as_str());
    query.push_str("&limit=");
    query.push_str(limit.to_string().as_str());

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

pub fn query_aptos_resources_raw(
    network_address: &str,
    account: &str,
    resource: &str
) -> String {

    let mut query = String::new();
    query.push_str(network_address);
    query.push_str("/accounts/");
    query.push_str(account);
    query.push_str("/resource/");
    query.push_str(resource);

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

pub fn query_aptos_resources_all_raw(
    network_address: &str,
    account: &str,
) -> String {

    let mut query = String::new();
    query.push_str(network_address);
    query.push_str("/accounts/");
    query.push_str(account);
    query.push_str("/resources");

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