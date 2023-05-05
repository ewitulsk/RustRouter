use std::str;

use aptos_sdk::{
    rest_client::Client
};

use crate::{types::Network, pairs::{PairTypes, Pair}, manager::Manager, utils::{write_pair_descriptors, read_pair_descriptors}};

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

    let pairs: Vec<PairTypes> = registrys::gen_all_pairs(network);


    manager.add_pairs(pairs);

    write_pair_descriptors(manager.managed_pairs);
    let read_pairs: Vec<PairTypes> = read_pair_descriptors();

    for pair_type in read_pairs {
        match pair_type {
            PairTypes::PancakePair(pair) => println!("Protocol: {}, SwapType: {}, PairKey: {}, PoolAddr: {}, routerPairAddr: {}", 
            pair.base.protocol, pair.base.swap_type, pair.base.pair_key, pair.base.pool_addr, pair.base.router_pair_addr)
        };
    }

    
}
