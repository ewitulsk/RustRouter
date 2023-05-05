use crate::{types::Network, pairs::{Pair, PairTypes, pancake_pair::PancakePair}, utils::{query_aptos_events_raw, string_to_u64}};
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
    
    let raw_data = query_aptos_events_raw(network_http, account, event);
    let data: Vec<PancakeData> = serde_json::from_str(&raw_data).unwrap();

    let mut all_pancake_pairs:Vec<PairTypes> = Vec::new();

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
            unique_field: 10
        };

        all_pancake_pairs.push(PairTypes::PancakePair(pancake_pair));
    };

    return all_pancake_pairs;
}