use std::collections::HashMap;
use std::hash::Hash;
use std::str;
use std::mem;

use aptos_sdk::{
    rest_client::Client
};

use crate::pairs::OutputAmount;
use crate::pairs::PairNames;
use crate::pairs::pancake_pair;
use crate::pairs::pancake_pair::{PancakeMetadata};
use crate::registrys::pancake_registry::get_pancake_metadata;
use crate::registrys::Populate;
use crate::registrys::all_registrys;
use crate::registrys::gen_all_pairs;
use crate::registrys::set_all_metadata;
use crate::utils::decimal_to_u64;
use crate::utils::u64_to_decimal;
use crate::{registrys::{RegistryTypes}, types::Network, pairs::{PairTypes, Pair, pancake_pair::PancakePair}, manager::Manager, utils::{write_pair_descriptors, read_pair_descriptors}};

mod pairs;
mod manager;
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


    let mut registry_map: HashMap<PairNames, RegistryTypes> = all_registrys(network.clone());
    let mut registrys: Vec<&mut RegistryTypes> = Vec::new();

    for (_, registry) in registry_map.iter_mut() {
        registrys.push(registry);
    }

    let mut pairs: Vec<PairTypes> = Vec::new();
    let metadata: HashMap<String, PancakeMetadata>;
    let registry: &mut RegistryTypes = *registrys.get_mut(0).unwrap();
    match registry {
        RegistryTypes::PancakeRegistry(pancake_registry) => {
            for pancake_pair in pancake_registry.get_pairs() {
                pairs.push(pancake_pair);
            }
            metadata = get_pancake_metadata(network);
        }
    }

    write_pair_descriptors(&pairs);

    let mut refreshed_pancake_pairs:Vec<PancakePair> = Vec::new();

    for pair in pairs {
        match pair {
            PairTypes::PancakePair(pancake_pair) => {
                let pair_id = format!("<{}, {}>", &pancake_pair.base.token_arr[0], &pancake_pair.base.token_arr[1]);

                let pair_metadata = metadata.get(&pair_id).unwrap();

                let refreshed_pancake_pair = PancakePair {
                    base: pancake_pair.base.clone(),
                    metadata: pair_metadata.clone()
                };

                refreshed_pancake_pairs.push(refreshed_pancake_pair)
            }
        }
    }




    let token_in = String::from("0x1::aptos_coin::AptosCoin");
    let in_decimal = 8;
    let token_out: String = String::from("0x7fd500c11216f0fe3095d0c4b8aa4d64a4e2e04f83758462f2b127255643615::thl_coin::THL");
    let out_decimal = 8;
    let input_amount = decimal_to_u64(1.0, in_decimal);




    let mut count = 0;
    for pancake_pair in &refreshed_pancake_pairs {
        
        if pancake_pair.base.token_arr.contains(&token_in) && pancake_pair.base.token_arr.contains(&token_out) {
            println!("ResX: {}, ResY: {}", (&pancake_pair.metadata.reserves.as_ref()).unwrap()[0], (&pancake_pair.metadata.reserves.as_ref()).unwrap()[1]);

            break;
        }
        count += 1;
    }   

    let mut selected_pair = refreshed_pancake_pairs.get_mut(count).unwrap();
    let token_x = &selected_pair.base.token_arr[0];
    let token_y = &selected_pair.base.token_arr[1];
    println!("ResX: {}, ResY: {}", (&selected_pair.metadata.reserves.as_ref()).unwrap()[0], (&selected_pair.metadata.reserves.as_ref()).unwrap()[1]);


    let amount_out = selected_pair.output_amount(input_amount, token_in.to_string(), token_out.to_string());

    println!("In: {}, Out: {}", u64_to_decimal(input_amount, in_decimal), u64_to_decimal(amount_out, out_decimal));
   

}
