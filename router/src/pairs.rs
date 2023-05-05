use self::pancake_pair::PancakePair;

use serde::{Serialize, Deserialize};


pub mod pancake_pair;

pub trait Descriptor {
    fn to_json(&self) -> String;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Pair {
    pub network: String,
    pub protocol: String,
    pub swap_type: String,
    pub pair_key: String,
    pub pool_addr: String,
    pub token_arr: Vec<String>,
    pub router_pair_addr: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PairTypes {
    PancakePair(PancakePair)
}