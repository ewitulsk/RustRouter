#![allow(dead_code)]

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};

use tracing_subscriber;
use tokio;

use std::collections::HashMap;

use crate::pairs::PairMetadata;
use crate::pairs::PairNames;
use crate::registrys::Registry;
use crate::registrys::pancake_registry::PancakeRegistry;
use crate::registrys::gen_all_pairs;
use crate::registrys::set_all_metadata;
use crate::registrys::update_pairs;
use crate::router::find_best_routes_for_fixed_input_amount;
use crate::utils::decimal_to_u64;
use crate::{types::Network};

mod pairs;
mod manager;
mod utils;
mod types;
mod registrys;
mod router;


struct ServerState{
    network: Network,
    registry_vec: Vec<Box<dyn Registry>>,
    metadata_map: HashMap<PairNames, HashMap<String, Box<dyn PairMetadata>> >,
}

async fn initalize_router() {
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


    set_all_metadata(&network, &mut registry_vec, &mut metadata_map).await;

    update_pairs(&mut genned_pairs, &mut metadata_map);

    let token_in = String::from("0x1::aptos_coin::AptosCoin");
    let in_decimal = 8;
    let token_out: String = String::from("0x159df6b7689437016108a019fd5bef736bac692b6d4a1f10c941f6fbb9a74ca6::oft::CakeOFT");
    let _out_decimal = 6;
    let input_amount = decimal_to_u64(1.0, in_decimal);


    let route_vec = find_best_routes_for_fixed_input_amount(pairs_by_token, &token_in, &token_out, input_amount, 10);
    let best_route = &route_vec[0];

    println!("Path: {:?}", best_route.path);
    println!("Path Amounts: {:?}", best_route.path_amounts);   

}


//The route forwared is to create a thread whos purpose is to maintain the state of 
//the pairs_by_token mapping and to fufill token routing requests. 
//Also split state management and async blockchain indexing into two different threads.
//One thread from the server, one thread that maintains the pairs_by_token mapping, and one thread that indexes the blockchain.

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Routey Is Live!"
}

async fn token_route_handler(
    Path(token_in): Path<String>,
    Path(in_decimal): Path<u64>,
    Path(token_out): Path<String>,
    Path(out_decimal): Path<u64>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{
    Ok(StatusCode::OK)
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/find_best_routes_for_fixed_input_amount/:token_in/:in_decimal/:token_out/:out_decimal", get(token_route_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}