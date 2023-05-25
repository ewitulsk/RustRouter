use core::num;
use std::{any::Any, str::FromStr};

use crate::utils::{get_network, query_aptos_resources_raw, string_to_u128, string_to_u64};

use super::{Pair, PairMetadata, PairNames, Descriptor};

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct PancakeDescriptor {
    pub network: String,
    pub protocol: String,
    pub pair_name: PairNames,
    pub pair_key: String,
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
                pair_key: self.pair_key.clone(),
                pool_addr: self.pool_addr.clone(),
                token_arr: self.token_arr.clone(),
                router_pair_addr: self.router_pair_addr.clone()

            }
        )
    }
}