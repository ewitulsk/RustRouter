use self::pancake_pair::PancakePair;

use serde::{Serialize, Deserialize};


pub mod pancake_pair;

pub trait Descriptor {
    fn get_pair(&self) -> Pair;
}

pub trait OutputAmount {
    fn output_amount(&self) -> u64;
}

pub trait Refresh {
    fn refresh_pair(&mut self);
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
pub struct Metadata {
    pub reserves: Vec<u64>,
    pub last_k: u128
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PairTypes {
    PancakePair(PancakePair)
}

impl Descriptor for PairTypes {
    fn get_pair(&self) -> Pair {
        match self {
            PairTypes::PancakePair(pair) => pair.base.clone()
        }
    }
}

impl Refresh for PairTypes {
    fn refresh_pair(&mut self) {
        match self {
            PairTypes::PancakePair(pair) => pair.refresh_pair()
        }
    }
}

