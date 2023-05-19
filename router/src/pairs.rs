use self::pancake_pair::PancakePair;

use aptos_sdk::move_types::metadata;
use serde::{Serialize, Deserialize};


pub mod pancake_pair;

pub trait Descriptor {
    fn get_pair(&self) -> Pair;
}

pub trait OutputAmount {
    fn output_amount(&self, input_amount: u64, token_in: String, token_out: String) -> u64;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Pair {
    pub network: String,
    pub protocol: String,
    pub pair_name: PairNames,
    pub pair_key: String,
    pub pool_addr: String,
    pub token_arr: Vec<String>,
    pub router_pair_addr: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PairTypes {
    PancakePair(PancakePair)
}

#[derive(Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub enum PairNames {
    PancakePair
}

impl Descriptor for PairTypes {
    fn get_pair(&self) -> Pair {
        match self {
            PairTypes::PancakePair(pair) => pair.base.clone()
        }
    }
}

