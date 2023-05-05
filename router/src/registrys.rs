use crate::{types::Network, pairs::{Pair, PairTypes}};

mod pancake_registry;


pub fn gen_all_pairs(network: Network) -> Vec<PairTypes>{
    let pancake_pairs = pancake_registry::get_pairs(network);
    return pancake_pairs;
}