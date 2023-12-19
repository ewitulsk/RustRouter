use serde::{Deserialize, Serialize};
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

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct NetworkReference<'a> {
    pub name: &'a str,
    pub http: &'a str,
    pub chain_id:  u64
}

#[derive(Clone)]
pub struct Network {
    pub name: String,
    pub http: String,
    pub chain_id: u64
}

#[derive(Clone)]
pub struct ServerConfig {
    pub network: &Network,
    pub registry_vec: &mut Vec<Box<dyn Registry>>,
    pub metadata_map: &mut HashMap<PairNames, HashMap<String, Box<dyn PairMetadata>>>,
    pub genned_pairs: &mut Vec<Rc<RefCell<Box<dyn Pair>>>>,
    pub pairs_by_token: HashMap<String, Vec<Rc<RefCell<Box<dyn Pair>>>>>
}

pub struct ServerConfigWrapper {
    server_config: ServerConfig
}
unsafe impl Send for ServerConfigWrapper {}