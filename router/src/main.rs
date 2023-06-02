use std::collections::HashMap;
use std::hash::Hash;
use std::str;
use std::mem;

use aptos_sdk::move_types::metadata;
use aptos_sdk::{
    rest_client::Client
};

use crate::pairs::PairMetadata;
use crate::pairs::PairNames;
use crate::pairs::pancake_pair;
use crate::pairs::pancake_pair::{PancakeMetadata};
use crate::registrys::Registry;
use crate::registrys::pancake_registry;
use crate::registrys::pancake_registry::PancakeRegistry;
use crate::registrys::gen_all_pairs;
use crate::registrys::set_all_metadata;
use crate::registrys::update_pairs;
use crate::router::find_best_routes_for_fixed_input_amount;
use crate::utils::decimal_to_u64;
use crate::utils::u64_to_decimal;
use crate::utils::write_pair_descriptors;
use crate::{types::Network, pairs::{Pair, pancake_pair::PancakePair}};

mod pairs;
mod manager;
mod utils;
mod types;
mod registrys;
mod router;


fn main() {
    println!("Hello, world!");
    let network: Network;
    match utils::get_network(String::from("aptos_mainnet")) {
        Ok(result) => {
            network = result;
            println!("Name: {}, ChainID: {}, HTTP: {}", network.name, network.chain_id, network.http);
        },
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }

    let pancake_registry = Box::new(PancakeRegistry {});
    let mut pancake_metadata_map: HashMap<String, Box<dyn PairMetadata>> = HashMap::new();

    let mut registry_vec: Vec<Box<dyn Registry>> = Vec::new();
    registry_vec.push(pancake_registry);

    let mut metadata_map: HashMap<PairNames, HashMap<String, Box<dyn PairMetadata>> > = HashMap::new();
    metadata_map.insert(PairNames::PancakePair, pancake_metadata_map);

    let mut gen_pairs_result = gen_all_pairs(&network, &mut registry_vec);
    let mut genned_pairs = gen_pairs_result.0;
    let mut pairs_by_token = gen_pairs_result.1;

    // write_pair_descriptors(&genned_pairs);
    // let mut genned_pairs: Vec<PairTypes> = read_pair_descriptors();


    set_all_metadata(&network, &mut registry_vec, &mut metadata_map);

    update_pairs(&mut genned_pairs, &mut metadata_map);

    let token_in = String::from("0x159df6b7689437016108a019fd5bef736bac692b6d4a1f10c941f6fbb9a74ca6::oft::CakeOFT");
    let in_decimal = 8;
    let token_out: String = String::from("0x8d87a65ba30e09357fa2edea2c80dbac296e5dec2b18287113500b902942929d::celer_coin_manager::UsdcCoin");
    let out_decimal = 6;
    let input_amount = decimal_to_u64(1.0, in_decimal);


    let route_vec = find_best_routes_for_fixed_input_amount(pairs_by_token, &token_in, &token_out, input_amount, 10);
    let best_route = &route_vec[0];

    println!("Path: {:?}", best_route.path);
    println!("Path Amounts: {:?}", best_route.path_amounts);   

}
