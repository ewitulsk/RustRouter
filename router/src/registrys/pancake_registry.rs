use std::{collections::HashMap};

use crate::{types::{Network}, pairs::{Pair, pancake_pair::{PancakePair, PancakeMetadata}, PairNames, PairMetadata}, utils::{query_aptos_events_raw, string_to_u64, query_aptos_resources_all_raw}};
use serde::{Serialize, Deserialize};

use super::{Registry};

//         "version": "126524019",
//         "guid": {
//             "creation_number": "4",
//             "account_address": "0xc7efb4076dbe143cbcd98cfaaa929ecfc8f299203dfff63b95ccb6bfe19850fa"
//         },
//         "sequence_number": "453",
//         "type": "0xc7efb4076dbe143cbcd98cfaaa929ecfc8f299203dfff63b95ccb6bfe19850fa::swap::PairCreatedEvent",
//         "data": {
//             "token_x": "0x3b6b4346280841a98236054920a5cf09afd5b2bbdfddc0b7de2673dca41072b::MICRO::Micro",
//             "token_y": "0xf22bede237a07e121b56d91a491eb7bcdfd1f5907926a9e58338f964a01b17fa::asset::USDT",
//             "user": "0x3b6b4346280841a98236054920a5cf09afd5b2bbdfddc0b7de2673dca41072b"
//         }


#[derive(Serialize, Deserialize)]
struct GUID {
    #[serde(deserialize_with = "string_to_u64")]
    creation_number: u64,
    account_address: String
}

#[derive(Serialize, Deserialize)]
struct Data {
    token_x: String,
    token_y: String,
    user: String
}

#[derive(Serialize, Deserialize)]
pub struct PancakeData {
    #[serde(deserialize_with = "string_to_u64")]
    version: u64,
    guid: GUID,
    #[serde(deserialize_with = "string_to_u64")]
    sequence_number: u64,
    #[serde(rename = "type")]
    event_type: String,
    data: Data
}

pub struct PancakeRegistry {}
impl Registry for PancakeRegistry {
    fn get_pairs(&self, network: &Network) -> Vec<Box<dyn Pair>>{
        let network_http = &network.http[..];
        let network_name = &network.name[..];
    
        let account = "0xc7efb4076dbe143cbcd98cfaaa929ecfc8f299203dfff63b95ccb6bfe19850fa";
        let event = "0xc7efb4076dbe143cbcd98cfaaa929ecfc8f299203dfff63b95ccb6bfe19850fa::swap::SwapInfo/pair_created";
        
        let mut all_pancake_pairs:Vec<Box<dyn Pair>> = Vec::new();
    
        let mut query: bool = true;
        let mut start: u64 = 0;
        while query {
            let raw_data = query_aptos_events_raw(network_http, account, event, start, 100);
            let data: Vec<PancakeData> = serde_json::from_str(&raw_data).unwrap();
    
            if data.len() < 100 {
                query = false;
            }
    
    
            for pair_data in data {
                // println!("X: {} Y: {}", pair_data.data.token_x, pair_data.data.token_y);
                let pair_key = format!("{}{}{}", account, pair_data.data.token_x, pair_data.data.token_y);
    
                let pancake_pair = PancakePair {
                    network: String::from(network_name),
                    protocol: String::from("pancake"),
                    pair_name: PairNames::PancakePair,
                    pair_key: String::from(pair_key),
                    pool_addr: String::from(account),
                    token_arr: Vec::from([pair_data.data.token_x, pair_data.data.token_y]),
                    router_pair_addr: String::new(),
                    metadata: PancakeMetadata {
                        reserves: None,
                    }
                };
    
                all_pancake_pairs.push(Box::new(pancake_pair));
            }
    
            start += 100;
        };

        return all_pancake_pairs;
    }

    fn get_metadata(&self, network: &Network, metadata_map: &mut HashMap<PairNames, HashMap<String, Box<dyn PairMetadata>> >){
        let network_http = &network.http[..];
    
        let account = "0xc7efb4076dbe143cbcd98cfaaa929ecfc8f299203dfff63b95ccb6bfe19850fa";
    
        let all_resources_raw = query_aptos_resources_all_raw(network_http, account);
    
        let all_resources:Vec<serde_json::Value> = serde_json::from_str(&all_resources_raw).unwrap();

        let mut pancake_map: HashMap<String, Box<dyn PairMetadata>> = HashMap::new();
        
        let mut count = 0;
        for resource in all_resources {
            let _type = resource.get("type").unwrap().as_str().unwrap();
            if _type.contains(&format!("{}::swap::TokenPairReserve", account)) {
                let token_names = String::from(&_type[90..]);
    
                let data = resource.get("data").unwrap();
                let res_x = data.get("reserve_x").unwrap().as_str().unwrap().parse::<u64>().unwrap();
                let res_y = data.get("reserve_y").unwrap().as_str().unwrap().parse::<u64>().unwrap();
    
                let metadata = PancakeMetadata {
                    reserves: Some(vec![res_x, res_y])
                };
    
                pancake_map.insert(token_names.clone(), Box::new(metadata));
    
                count += 1;
            }
        }
        println!("Total: {}", count);

        metadata_map.insert(PairNames::PancakePair, pancake_map);
    }

    
}


