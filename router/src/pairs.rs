use std::any::Any;

use serde::{Serialize, Deserialize};
use erased_serde::serialize_trait_object;

pub mod pancake_pair;

pub trait PairMetadata: Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

pub trait Pair: Send + Sync {
    fn output_amount(&self, input_amount: u64, token_in: &String, token_out: &String) -> u64;
    fn get_descriptor(&self) -> Box<dyn Descriptor>;
    fn get_protocol(&self) -> &str;
    fn get_pair_key(&self) -> &str;
    fn get_token_arr(&self) -> &Vec<String>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Descriptor: erased_serde::Serialize {}

serialize_trait_object!(Descriptor);

#[derive(Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub enum PairNames {
    PancakePair
}


