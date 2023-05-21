use std::any::Any;

use self::pancake_pair::PancakePair;

use aptos_sdk::move_types::metadata;
use serde::{Serialize, Deserialize};


pub mod pancake_pair;

pub trait PairMetadata {
    fn as_any(&self) -> &dyn Any;
}

pub trait Pair {
    fn output_amount(&self, input_amount: u64, token_in: String, token_out: String) -> u64;
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PairTypes {
    PancakePair(PancakePair)
}

#[derive(Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub enum PairNames {
    PancakePair
}


