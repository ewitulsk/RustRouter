use core::num;
use std::any::Any;

use crate::utils::{get_network, query_aptos_resources_raw, string_to_u128, string_to_u64};

use super::{Pair, PairMetadata, PairNames};

use serde::{Serialize, Deserialize};

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
}