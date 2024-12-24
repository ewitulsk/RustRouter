use std::{collections::HashMap, rc::Rc, cell::RefCell};

use serde_json::{self, Value};

use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    pairs::{liquidswap_pair::{self, LiquidswapMetadata, LiquidswapPair}, pancake_pair::{PancakeMetadata, PancakePair}, Pair, PairMetadata, PairNames}, registrys::{liquidswap_registry::LiquidswapRegistry, pancake_registry::PancakeRegistry}, types::Network
};

use async_trait::async_trait;

pub mod pancake_registry;
pub mod liquidswap_registry;

#[async_trait]
pub trait Registry: Send + Sync {
    fn module_address(&self) -> &str;
    fn protocol(&self) -> PairNames;
    async fn get_pairs(&self, network: &Network) -> Vec<Box<dyn Pair>>;
    async fn get_metadata(&self, network: &Network, metadata_map: &mut HashMap<PairNames, HashMap<String, Box<dyn PairMetadata>> >);
    fn build_metadata_map_from_changes(&self, changes: Vec<Value>) -> HashMap<String, Box<dyn PairMetadata>>;
}

pub fn build_metadata_map_from_changes(registrys: &Vec<Box<dyn Registry>>, changes: Vec<Value>) -> HashMap<PairNames, HashMap<String, Box<dyn PairMetadata>> > {
    let start_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let mut metadata_map: HashMap<PairNames, HashMap<String, Box<dyn PairMetadata>> > = HashMap::new();

    for registry in registrys {
        let protocol_metadata_map = registry.build_metadata_map_from_changes(changes.clone());
        metadata_map.insert(registry.protocol(), protocol_metadata_map);
    }

    let end_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    println!("Took {:?} ms to build changes.", end_ms - start_ms);

    metadata_map
}

pub fn get_all_registerys_from_json(network: &Network) -> Vec<Box<dyn Registry>> {
    let json = serde_json::from_str::<Vec<Value>>(include_str!("../registerys.json")).unwrap();
    
    let pancake_registry_val = json.iter().find(|x| x["protocol"] == "pancake" && x["network"] == network.name).unwrap().clone();
    let pancake_registry = Box::new(serde_json::from_value::<PancakeRegistry>(pancake_registry_val).unwrap()) as Box<dyn Registry>;

    let liquidswap_registry_val = json.iter().find(|x| x["protocol"] == "liquidswap_constant_product" && x["network"] == network.name).unwrap().clone();
    let liquidswap_registry = Box::new(serde_json::from_value::<LiquidswapRegistry>(liquidswap_registry_val).unwrap()) as Box<dyn Registry>;

    let registrys = vec![pancake_registry, liquidswap_registry];
    return registrys;
}

// pub fn all_registrys<'a>(network: Network) -> HashMap<PairNames, RegistryTypes> {
//     let mut registryMap: HashMap<PairNames, RegistryTypes> = HashMap::new();

//     registryMap.insert(PairNames::PancakePair, 
//         RegistryTypes::PancakeRegistry(
//             PancakeRegistry {
//                 registry: Registry {
//                     network: network,
//                     pairs: None,
//                 },
//                 metadata_map: HashMap::new()
//             }
//         )
//     );
    
//     return registryMap;
// }

// pub fn gen_pairs_by_token_map(pairs: Vec<PairTypes>) -> HashMap<String, Vec<PairTypes>> {
//     let pairs_by_token_map:&mut HashMap<String, Vec<PairTypes>> = &mut HashMap::new();
//     for pair in pairs {
//         match pair {
//             PairTypes::PancakePair(pancake_pair) => {
//                 let tokens = pancake_pair.base.token_arr;
//                 for token in tokens {
//                     if pairs_by_token_map.contains_key(&token) {
//                         let token_vec = pairs_by_token_map.get_mut(&token).unwrap();
//                         token_vec.push(pair);
//                     }
//                     else {
//                         pairs_by_token_map.insert(token, vec![pair]);
//                     }
//                 }
//             }
//         }
//     }
//     return pairs_by_token_map.clone();
// }   

pub async fn gen_all_pairs(network: &Network, registrys: &mut Vec<Box<dyn Registry>>) -> (Vec<Rc<RefCell<Box<dyn Pair>>>>, HashMap<String, Vec<Rc<RefCell<Box<dyn Pair>>>> >) {
    let mut pairs: Vec<Rc<RefCell<Box<dyn Pair>>>> = Vec::new();
    let mut pairs_by_token: HashMap<String, Vec<Rc<RefCell<Box<dyn Pair>>>> > = HashMap::new();
    for registry in registrys {
        let reg_pairs = (*registry).get_pairs(network).await;

        for pair in reg_pairs {
            let token_arr = pair.get_token_arr().clone();
            let pair_rc_refcel: Rc<RefCell<Box<dyn Pair>>> = Rc::new(RefCell::new(pair));

            for token in token_arr {
                if !pairs_by_token.contains_key(&token) {
                    pairs_by_token.insert(token.clone(), Vec::new());
                }
                let pair_vec = pairs_by_token.get_mut(&token).unwrap();
                pair_vec.push(pair_rc_refcel.clone());
            }

            pairs.push(pair_rc_refcel.clone());
        }

    } 
    return (pairs, pairs_by_token);
}

pub async fn set_all_metadata(network: &Network, registrys: &mut Vec<Box<dyn Registry>>, metadata_map: &mut HashMap<PairNames, HashMap<String, Box<dyn PairMetadata>> >) {
    for registry in registrys {
       (*registry).get_metadata(network, metadata_map).await;
    }
}

pub fn update_pairs(pairs: &mut Vec<Rc<RefCell<Box<dyn Pair>>>>, metadata_map: &mut HashMap<PairNames, HashMap<String, Box<dyn PairMetadata>> >) {
    for pair_rc_ref in pairs {
        let mut pair = (**pair_rc_ref).borrow_mut();
        let protocol = pair.get_protocol();
        match protocol {
            "pancake" => {
                let mut pancake_pair = &mut *pair.as_any_mut().downcast_mut::<PancakePair>().unwrap();
                let pancake_metadata_map = &*metadata_map.get(&PairNames::PancakePair).unwrap();

                let identifier = format!("<{}, {}>", pancake_pair.token_arr[0], pancake_pair.token_arr[1]);
                
                if pancake_metadata_map.contains_key(&identifier) {
                    let pancake_metadata: &PancakeMetadata = &*(*(pancake_metadata_map.get(&identifier).unwrap())).as_any().downcast_ref::<PancakeMetadata>().unwrap();
                    
                    let temp_res = pancake_metadata.reserves.clone().unwrap();
                    let res_x = temp_res.get(0).unwrap();
                    let res_y = temp_res.get(1).unwrap();
                    

                    if(pancake_pair.metadata.reserves.is_some()){
                        let old_temp_res = pancake_pair.metadata.reserves.clone().unwrap();
                        let old_res_x = old_temp_res.get(0).unwrap();
                        let old_res_y = old_temp_res.get(1).unwrap();
                    }  
                    
                    pancake_pair.metadata = pancake_metadata.clone();
                }
            } 

            "liquidswap_constant_product" => {
                let mut pair = &mut *pair.as_any_mut().downcast_mut::<LiquidswapPair>().unwrap();

                let liquid_metadata_map = &*metadata_map.get(&PairNames::LiquidswapPair).unwrap();

                let curve = if pair.curve_type == liquidswap_pair::CurveType::Uncorrelated {"Uncorrelated"} else {"Stable"};

                let identifier = format!("{},{},{}", pair.token_arr[0], pair.token_arr[1], curve);
                
                if liquid_metadata_map.contains_key(&identifier) {
                    let metadata: &LiquidswapMetadata = &*(*(liquid_metadata_map.get(&identifier).unwrap())).as_any().downcast_ref::<LiquidswapMetadata>().unwrap();
                    
                    let temp_res = metadata.reserves.clone();
                    let res_x = temp_res.get(0).unwrap();
                    let res_y = temp_res.get(1).unwrap();
                    
                    pair.metadata = metadata.clone();
                }
            }

            &_ => {

            }  
        }
    }
}