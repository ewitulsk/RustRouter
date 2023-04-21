use std::str;

use aptos_sdk::{
    rest_client::Client
};

mod pairs;
mod manager;
mod utils;
mod types;

static NODE_URL: &str = "";

fn queryEvents(
    account: &str,
    event: &str
){
    let mut query = String::new();

    query.push_str(NODE_URL);
    query.push_str("/accounts/");
    query.push_str(account);
    query.push_str("/events/");

    // let client = Client::new(query);

}

fn main() {
    println!("Hello, world!");
    match utils::get_network(String::from("aptos_mainnet")) {
        Ok(result) => {
            println!("Name: {}, ChainID: {}, HTTP: {}", result.name, result.chain_id, result.http);
        },
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
}
