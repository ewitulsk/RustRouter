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
use std::sync::mpsc::{self, RecvTimeoutError, RecvError};
use std::time::Duration;
use std::thread;
use std::rc::Rc;
use std::cell::RefCell;

use crate::pairs::PairMetadata;
use crate::pairs::PairNames;
use crate::pairs::Pair;
use crate::registrys::Registry;
use crate::registrys::pancake_registry::PancakeRegistry;
use crate::registrys::{
    gen_all_pairs, 
    get_all_registerys_from_json, 
    build_metadata_map_from_changes,
    set_all_metadata, 
    update_pairs
};
use crate::router::find_best_routes_for_fixed_input_amount;
use crate::utils::{decimal_to_u64, get_aptos_version};
use crate::{
    types::{Network, ChannelUpdateMetadata, ChannelRegistrysToWatch}
};
use crate::aptos_transaction_watcher::aptos_watch_transactions;

mod pairs;
mod manager;
mod utils;
mod types;
mod registrys;
mod router;
mod aptos_transaction_watcher;

async fn initalize_router(network: &Network) -> (
    Vec<Box<dyn registrys::Registry>>, //registery_vec
    HashMap<PairNames, HashMap<std::string::String, Box<dyn PairMetadata>>>, //metadata_map
    Vec<Rc<RefCell<Box<(dyn Pair + 'static)>>>>, //genned_pairs
    HashMap<String, Vec<Rc<RefCell<Box<(dyn Pair + 'static)>>>>>, //pairs_by_token
) {
    let mut registry_vec = get_all_registerys_from_json(network);
    let pancake_metadata_map: HashMap<String, Box<dyn PairMetadata>> = HashMap::new();

    let mut metadata_map: HashMap<PairNames, HashMap<String, Box<dyn PairMetadata>> > = HashMap::new();
    metadata_map.insert(PairNames::PancakePair, pancake_metadata_map);

    let gen_pairs_result = gen_all_pairs(network, &mut registry_vec).await;
    let mut genned_pairs = gen_pairs_result.0;
    let pairs_by_token = gen_pairs_result.1;

    return (registry_vec, metadata_map, genned_pairs, pairs_by_token);
}


//The route forwared is to create a thread whos purpose is to maintain the state of 
//the pairs_by_token mapping and to fufill token routing requests. 
//Also split state management and async blockchain indexing into two different threads.
//One thread from the server, one thread that maintains the pairs_by_token mapping, and one thread that indexes the blockchain.

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Routey Is Live!"
}

#[derive(serde::Deserialize)]
struct RouteRequest {
    token_in: String,
    in_decimal: u64,
    token_out: String,
    out_decimal: u64,
    input_amount: u64,
}

struct ChannelRouteRequest {
   route_request: RouteRequest,
   fromthread_tx: mpsc::Sender<RouteResponseBody>,
}

#[derive(serde::Serialize)]
struct RouteResponseBody {
    path: Vec<String>,
    path_amounts: Vec<u64>,
}

#[derive(Clone)]
struct ServerState{
    tothread_tx: mpsc::Sender<ChannelRouteRequest>,
}

async fn token_route_handler(
    State(state): State<ServerState>,
    Json(payload): Json<RouteRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let (fromthread_tx, fromthread_rx) = mpsc::channel::<RouteResponseBody>();

    state.tothread_tx.send(
        ChannelRouteRequest{route_request: payload, fromthread_tx: fromthread_tx}
    ).unwrap();

    let response_body = fromthread_rx.recv().unwrap();

    return Ok(Json(response_body));
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let (tothread_tx, tothread_rx) = mpsc::channel::<ChannelRouteRequest>();
    let (tothread_updater_tx, tothread_updater_rx) = mpsc::channel::<ChannelUpdateMetadata>();

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

    

    let router_network = network.clone();
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            let (mut registry_vec, mut metadata_map, mut genned_pairs, pairs_by_token) = initalize_router(&router_network).await;
            let registrys_to_watch = registry_vec.iter().map(|x| x.module_address().to_string()).collect::<Vec<String>>();

            let starting_version = get_aptos_version(&router_network.http).await.unwrap();
            //We should be running this in the loop, BUT, it is inefficiently querying data for each pair.
            //So we're hitting a node rate limit.

            set_all_metadata(&network, &mut registry_vec, &mut metadata_map).await;
            update_pairs(&mut genned_pairs, &mut metadata_map);

            thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async move {
                    aptos_watch_transactions(&router_network, starting_version, &tothread_updater_tx).await;
                });
            });

            loop {
                match tothread_updater_rx.recv_timeout(Duration::from_millis(100)) {
                    Ok(message) => {
                        match message.channel_tx {
                            Some(channel_tx) => {
                                channel_tx.send(ChannelRegistrysToWatch{
                                    registrys_to_watch: registrys_to_watch.clone()
                                }).unwrap();
                            }
                            None => {}
                        }
                        match message.new_metadata {
                            Some(new_metadata) => {
                                let mut metadata_map = build_metadata_map_from_changes(&registry_vec, new_metadata);
                                update_pairs(&mut genned_pairs, &mut metadata_map);
                            }
                            None => {}
                        }
                    },
                    Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => {
                        println!("Disconnected")
                    }
                }
                
                match tothread_rx.recv_timeout(Duration::from_millis(500)) {
                    Ok(message) => {
                        let payload = message.route_request;
                        let token_in = payload.token_in;
                        let token_out = payload.token_out;
                        let input_amount = payload.input_amount;
                        let route_vec = find_best_routes_for_fixed_input_amount(&pairs_by_token, &token_in, &token_out, input_amount, 10);
                        let best_route = &route_vec[0];

                        let response_body = RouteResponseBody {
                            path: best_route.path.clone(),
                            path_amounts: best_route.path_amounts.clone(),
                        };

                        message.fromthread_tx.send(response_body).unwrap();
                        
                        println!("Path: {:?}", best_route.path);
                        println!("Path Amounts: {:?}", best_route.path_amounts);   
                    },
                    Err(RecvTimeoutError::Timeout) => {
                        println!("Timeout");
                    }
                    Err(RecvTimeoutError::Disconnected) => {
                        println!("Disconnected");
                    }
                }
            }
        });
    });

    let state = ServerState{
        tothread_tx: tothread_tx,
    };

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route(
            "/find_best_routes_for_fixed_input_amount",
            post(token_route_handler)
        )
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
