use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use axum::{debug_handler, extract::State};

use crate::pairs::PairMetadata;
use crate::pairs::PairNames;
use crate::pairs::Pair;
use crate::registrys::Registry;
use crate::registrys::pancake_registry::PancakeRegistry;
use crate::registrys::gen_all_pairs;
use crate::registrys::set_all_metadata;
use crate::registrys::update_pairs;
use crate::router::find_best_routes_for_fixed_input_amount;
use crate::routes::find_best_routes;
use crate::utils::decimal_to_u64;
use crate::types::ServerConfig;


#[debug_handler]
pub async fn find_best_routes(
    State(server_config): State<ServerConfig>
) {
    set_all_metadata(server_config.network, server_config.registry_vec, server_config.metadata_map).await;
    update_pairs(server_config.genned_pairs, server_config.metadata_map);

    let token_in = String::from("0x1::aptos_coin::AptosCoin");
    let in_decimal = 8;
    let token_out: String = String::from("0x159df6b7689437016108a019fd5bef736bac692b6d4a1f10c941f6fbb9a74ca6::oft::CakeOFT");
    let _out_decimal = 6;
    let input_amount = decimal_to_u64(1.0, in_decimal);


    let route_vec = find_best_routes_for_fixed_input_amount(server_config.pairs_by_token, &token_in, &token_out, input_amount, 10);
    let best_route = &route_vec[0];
}