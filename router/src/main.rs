#![allow(dead_code)]

use std::collections::HashMap;

use tracing_subscriber;

use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    handler::{HandlerWithoutStateExt, Handler},
    extract::State,
    Json, Router,
};

use crate::pairs::PairMetadata;
use crate::pairs::PairNames;
use crate::registrys::Registry;
use crate::registrys::pancake_registry::PancakeRegistry;
use crate::registrys::gen_all_pairs;
use crate::registrys::set_all_metadata;
use crate::registrys::update_pairs;
use crate::router::find_best_routes_for_fixed_input_amount;
use crate::routes::find_best_routes::find_best_routes;
use crate::utils::decimal_to_u64;
use crate::types::{ServerConfig, ServerConfigWrapper, Network};

mod pairs;
mod manager;
mod utils;
mod types;
mod registrys;
mod router;
mod routes{
    pub mod find_best_routes;
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Hello, world!");

    tracing_subscriber::fmt::init();

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
    let pancake_metadata_map: HashMap<String, Box<dyn PairMetadata>> = HashMap::new();

    let mut registry_vec: Vec<Box<dyn Registry>> = Vec::new();
    registry_vec.push(pancake_registry);

    let mut metadata_map: HashMap<PairNames, HashMap<String, Box<dyn PairMetadata>> > = HashMap::new();
    metadata_map.insert(PairNames::PancakePair, pancake_metadata_map);

    let gen_pairs_result = gen_all_pairs(&network, &mut registry_vec).await;
    let mut genned_pairs = gen_pairs_result.0;
    let pairs_by_token = gen_pairs_result.1;

    // write_pair_descriptors(&genned_pairs);
    // let mut genned_pairs: Vec<PairTypes> = read_pair_descriptors();


    let server_config = ServerConfig {
        network: &network,
        registry_vec: &mut registry_vec,
        metadata_map: &mut metadata_map,
        genned_pairs: &mut genned_pairs,
        pairs_by_token: pairs_by_token
    };

    let wrapper = ServerConfigWrapper {server_config};

    let app = Router::with_state(wrapper)
    .route("/", 
        get(|| async {"Hello, world!"})
    )
    // .route("/test",
    //     get(|| async {"Test"})
    // );
    .route("/route", 
        get(|| {find_best_routes})
    );



    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    

    // let token_in = String::from("0x1::aptos_coin::AptosCoin");
    // let in_decimal = 8;
    // let token_out: String = String::from("0x159df6b7689437016108a019fd5bef736bac692b6d4a1f10c941f6fbb9a74ca6::oft::CakeOFT");
    // let _out_decimal = 6;
    // let input_amount = decimal_to_u64(1.0, in_decimal);


    // let route_vec = find_best_routes_for_fixed_input_amount(pairs_by_token, &token_in, &token_out, input_amount, 10);
    // let best_route = &route_vec[0];

    // println!("Path: {:?}", best_route.path);
    // println!("Path Amounts: {:?}", best_route.path_amounts);   

}
