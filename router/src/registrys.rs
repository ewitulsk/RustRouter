use std::collections::HashMap;

use crate::{types::{Network}, pairs::{Pair, PairTypes, Metadata}};

use self::pancake_registry::get_metadata;

mod pancake_registry;


pub fn gen_all_pairs(network: Network) -> Vec<PairTypes>{
    let pancake_pairs = pancake_registry::get_pairs(network);
    return pancake_pairs;
}

pub fn _test_get_metadata(network: Network) -> HashMap<String, Metadata> {
    get_metadata(network)
}