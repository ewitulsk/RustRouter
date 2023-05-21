use std::collections::HashMap;
use std::hash::Hash;
use std::str;
use std::mem;

use aptos_sdk::move_types::metadata;
use aptos_sdk::{
    rest_client::Client
};

use crate::pairs::OutputAmount;
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
use crate::utils::decimal_to_u64;
use crate::utils::u64_to_decimal;
use crate::{types::Network, pairs::{PairTypes, Pair, pancake_pair::PancakePair}, utils::{write_pair_descriptors, read_pair_descriptors}};

mod pairs;
// mod manager;
mod utils;
mod types;
mod registrys;


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


    // let mut registry_map: HashMap<PairNames, RegistryTypes> = all_registrys(network.clone());
    // let mut registrys: Vec<&mut RegistryTypes> = Vec::new();

    // for (_, registry) in registry_map.iter_mut() {
    //     registrys.push(registry);
    // }

    // let mut pairs: Vec<PairTypes> = Vec::new();
    // let metadata: HashMap<String, PancakeMetadata>;
    // let registry: &mut RegistryTypes = *registrys.get_mut(0).unwrap();
    // match registry {
    //     RegistryTypes::PancakeRegistry(pancake_registry) => {
    //         for pancake_pair in pancake_registry.get_pairs() {
    //             pairs.push(pancake_pair);
    //         }
    //         metadata = get_pancake_metadata(network);
    //     }
    // }

    // write_pair_descriptors(&pairs);

    let pancake_registry = Box::new(PancakeRegistry {});
    let mut pancake_metadata_map: HashMap<String, Box<dyn PairMetadata>> = HashMap::new();

    let mut registry_vec: Vec<Box<dyn Registry>> = Vec::new();
    registry_vec.push(pancake_registry);

    let mut metadata_map: HashMap<PairNames, HashMap<String, Box<dyn PairMetadata>> > = HashMap::new();
    metadata_map.insert(PairNames::PancakePair, pancake_metadata_map);

    let mut genned_pairs = gen_all_pairs(&network, &mut registry_vec);

    set_all_metadata(&network, &mut registry_vec, &mut metadata_map);

    update_pairs(&mut genned_pairs, &mut metadata_map);

    let token_in = String::from("0x159df6b7689437016108a019fd5bef736bac692b6d4a1f10c941f6fbb9a74ca6::oft::CakeOFT");
    let in_decimal = 8;
    let token_out: String = String::from("0x8d87a65ba30e09357fa2edea2c80dbac296e5dec2b18287113500b902942929d::celer_coin_manager::UsdcCoin");
    let out_decimal = 6;
    let input_amount = decimal_to_u64(1.0, in_decimal);


    let mut count = 0;
    for pair in &genned_pairs {
        match pair {
            PairTypes::PancakePair(pancake_pair) => {
                if pancake_pair.base.token_arr.contains(&token_in) && pancake_pair.base.token_arr.contains(&token_out) {
                    println!("ResX: {}, ResY: {}", (&pancake_pair.metadata.reserves.as_ref()).unwrap()[0], (&pancake_pair.metadata.reserves.as_ref()).unwrap()[1]);
        
                    break;
                }
                count += 1;
            }
        }
        
        
    }   

    let mut selected_pair = genned_pairs.get_mut(count).unwrap();
    match selected_pair {
        PairTypes::PancakePair(pancake_pair) => {
            let token_x = &pancake_pair.base.token_arr[0];
            let token_y = &pancake_pair.base.token_arr[1];
            println!("ResX: {}, ResY: {}", (&pancake_pair.metadata.reserves.as_ref()).unwrap()[0], (&pancake_pair.metadata.reserves.as_ref()).unwrap()[1]);


            let amount_out = pancake_pair.output_amount(input_amount, token_in.to_string(), token_out.to_string());

            println!("In: {}, Out: {}", u64_to_decimal(input_amount, in_decimal), u64_to_decimal(amount_out, out_decimal));
        }
    }
    
   

}
