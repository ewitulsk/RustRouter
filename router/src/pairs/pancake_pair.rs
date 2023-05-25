use core::num;
use std::{any::Any, str::FromStr};

use crate::utils::{get_network, query_aptos_resources_raw, string_to_u128, string_to_u64};

use super::{Pair, PairMetadata, PairNames, Descriptor};

use serde::{Serialize, Deserialize};
use serde_json::Value;


#[derive(Serialize, Deserialize)]
pub struct PancakeDescriptor {
    pub network: String,
    pub protocol: String,
    pub pair_name: PairNames,
    pub pool_addr: String,
    pub token_arr: Vec<String>,
    pub router_pair_addr: String,
}

impl Descriptor for PancakeDescriptor {}

#[derive(Serialize, Deserialize, Clone)]
pub struct PancakeMetadata {
    pub reserves: Option<Vec<u64>>,
    // pub last_k: Option<u128>
}
impl PairMetadata for PancakeMetadata {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PancakePair {
    pub network: String,
    pub protocol: String,
    pub pair_name: PairNames,
    pub pair_key: String,
    pub pool_addr: String,
    pub token_arr: Vec<String>,
    pub router_pair_addr: String,
    pub metadata: PancakeMetadata
}

impl Pair for PancakePair {
    fn output_amount(&self, input_amount: u64, token_in: String, token_out: String) -> u64 {
        let in_index = self.token_arr.iter().position(|x| x == &token_in).unwrap();
        let out_index = self.token_arr.iter().position(|x| x == &token_out).unwrap();
        let reserve_out = self.metadata.reserves.as_ref().unwrap()[out_index];
        let reserve_in = self.metadata.reserves.as_ref().unwrap()[in_index];
        let amount_in_with_fee = (input_amount as u128) * 9975u128;
        let numerator = amount_in_with_fee * (reserve_out as u128);
        let denominator = ((reserve_in as u128) * 10000u128) + amount_in_with_fee;
        let amount_out = (numerator / denominator) as u64; 
        
        return amount_out;
    }

    fn get_descriptor(&self) -> Box<dyn Descriptor> {
        return Box::new(
            PancakeDescriptor {
                network: self.network.clone(),
                protocol: self.protocol.clone(),
                pair_name: self.pair_name.clone(),
                pool_addr: self.pool_addr.clone(),
                token_arr: self.token_arr.clone(),
                router_pair_addr: self.router_pair_addr.clone()

            }
        )
    }
}

pub fn pancake_from_value_descriptor(descriptor: Value) -> PancakePair {
    let token_val_arr: Vec<Value> = descriptor.get("token_arr").unwrap().as_array().unwrap().clone();
    let mut token_arr: Vec<String> = Vec::new();

    for val in token_val_arr {
        let token = val.as_str().unwrap().to_string();
        token_arr.push(token);
    }

    let network = descriptor.get("network").unwrap().clone();
    let protocol = descriptor.get("protocol").unwrap().clone();
    let pool_addr = descriptor.get("pool_addr").unwrap().clone();
    let router_addr = descriptor.get("router_pair_addr").unwrap().clone();
    let pair_key = format!("{}{}{}", pool_addr, token_arr[0], token_arr[1]);

    return PancakePair {
            network: network.as_str().unwrap().to_string(),
            protocol: protocol.as_str().unwrap().to_string(),
            pair_name: PairNames::PancakePair,
            pair_key: pair_key,
            pool_addr: pool_addr.as_str().unwrap().to_string(),
            token_arr: token_arr,
            router_pair_addr: router_addr.as_str().unwrap().to_string(),
            metadata: PancakeMetadata { reserves: None }
        }
}