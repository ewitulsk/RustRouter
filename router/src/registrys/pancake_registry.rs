use std::{collections::HashMap, hash::Hash};

use crate::{types::{Network}, pairs::{Pair, PairTypes, Metadata, pancake_pair::{PancakePair}}, utils::{query_aptos_events_raw, string_to_u64, query_aptos_resources_all_raw}};
use serde::{Serialize, Deserialize};

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
struct PancakeData {
    #[serde(deserialize_with = "string_to_u64")]
    version: u64,
    guid: GUID,
    #[serde(deserialize_with = "string_to_u64")]
    sequence_number: u64,
    #[serde(rename = "type")]
    event_type: String,
    data: Data
}


pub fn get_pairs(network: Network) -> Vec<PairTypes>{
    let network_http = &network.http[..];
    let network_name = &network.name[..];

    let account = "0xc7efb4076dbe143cbcd98cfaaa929ecfc8f299203dfff63b95ccb6bfe19850fa";
    let event = "0xc7efb4076dbe143cbcd98cfaaa929ecfc8f299203dfff63b95ccb6bfe19850fa::swap::SwapInfo/pair_created";
    
    let mut all_pancake_pairs:Vec<PairTypes> = Vec::new();

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

            let pair = Pair {
                network: String::from(network_name),
                protocol: String::from("pancake"),
                swap_type: String::from("UniV2"),
                pair_key: String::from(pair_key),
                pool_addr: String::from(account),
                token_arr: Vec::from([pair_data.data.token_x, pair_data.data.token_y]),
                router_pair_addr: String::new()
            };

            let pancake_pair = PancakePair {
                base: pair,
                metadata: None
            };

            all_pancake_pairs.push(PairTypes::PancakePair(pancake_pair));
        }

        start += 100;
    };

    return all_pancake_pairs;
}

pub fn get_metadata(network: Network) -> HashMap<String, Metadata> {
    let network_http = &network.http[..];
    let network_name = &network.name[..];

    let account = "0xc7efb4076dbe143cbcd98cfaaa929ecfc8f299203dfff63b95ccb6bfe19850fa";

    let all_resources_raw = query_aptos_resources_all_raw(network_http, account);

    let all_resources:Vec<serde_json::Value> = serde_json::from_str(&all_resources_raw).unwrap();

    let mut metadata_map: HashMap<String, Metadata> = HashMap::new();

    let mut count = 0;
    for resource in all_resources {
        let _type = resource.get("type").unwrap().as_str().unwrap();
        if _type.contains(&format!("{}::swap::TokenPairMetadata", account)) {
            let token_names = String::from(&_type[91..]);

            let data = resource.get("data").unwrap();
            let last_k = data.get("k_last").unwrap().as_str().unwrap().parse::<u128>().unwrap(); //Who needs error handling?
            let bal_x = data.get("balance_x").unwrap().get("value").unwrap().as_str().unwrap().parse::<u64>().unwrap();
            let bal_y = data.get("balance_y").unwrap().get("value").unwrap().as_str().unwrap().parse::<u64>().unwrap();

            let metadata = Metadata {
                last_k: last_k,
                reserves: vec![bal_x, bal_y]
            };

            metadata_map.insert(token_names.clone(), metadata);

            count += 1;
            println!("{}\n", token_names);
        }

        
        // let data = resource.get("data").unwrap();
    }
    println!("Total: {}", count);

    
    return metadata_map;
}