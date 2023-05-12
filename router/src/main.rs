use std::str;
use std::mem;

use aptos_sdk::{
    rest_client::Client
};

use crate::registrys::_test_get_metadata;
use crate::{types::Network, pairs::{PairTypes, Pair, pancake_pair::PancakePair, Refresh}, manager::Manager, utils::{write_pair_descriptors, read_pair_descriptors}};

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

    let mut manager = Manager::empty();

    // protocol: String,
    // swap_type: String,
    // pair_key: String,
    // pool_addr: String,
    // token_arr: Vec<String>,
    // router_pair_add: String,

    // let pairs: Vec<PairTypes> = registrys::gen_all_pairs(network);


    // manager.add_pairs(pairs);

    
    
    // write_pair_descriptors(&manager.managed_pairs);
    

    // let mut read_pairs: Vec<PairTypes> = read_pair_descriptors().clone();

    // manager.refresh_pairs();

    // let enum_pair = read_pairs[0].clone();

    // println!("Amount: {}", read_pairs.len());
    


    //TODO!!!
    //Make registry class
    //add traits for get_pairs and get_metadata
    //make all metadata feilds optional
    //make managers refresh pairs call get_metada
    //maybe move pair generation to manager?
    



    let map = _test_get_metadata(network);

    let meta_test = map.get(&String::from("<0x1::aptos_coin::AptosCoin, 0xa1ea1833696326fbef7d135a4fa318e4cf3f3365329b145ee37969628c8ee7bb::LeagueOfShogunsToken::LeagueOfShoguns>")).unwrap();

    println!("BalX: {}, BalY: {}, K: {}", &meta_test.reserves[0], &meta_test.reserves[1], &meta_test.last_k);

}
