use std::{collections::HashMap, hash::Hash, vec};

use crate::{types::{Network}, pairs::{Pair, PairTypes, PairNames, pancake_pair::PancakeMetadata}, registrys::pancake_registry::{PancakeRegistry}};

pub mod pancake_registry;

#[derive(Clone)]
pub enum RegistryTypes {
    PancakeRegistry(PancakeRegistry)
}

#[derive(Clone)]
pub struct Registry {
    pub network: Network,
    pub pairs: Option<Vec<PairTypes>>,
}

pub trait Populate {
    fn get_pairs(&mut self) -> Vec<PairTypes>;
    fn get_metadata(&mut self);
}

pub fn all_registrys<'a>(network: Network) -> HashMap<PairNames, RegistryTypes> {
    let mut registryMap: HashMap<PairNames, RegistryTypes> = HashMap::new();

    registryMap.insert(PairNames::PancakePair, 
        RegistryTypes::PancakeRegistry(
            PancakeRegistry {
                registry: Registry {
                    network: network,
                    pairs: None,
                },
                metadata_map: HashMap::new()
            }
        )
    );
    
    return registryMap;
}

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

pub fn gen_all_pairs<'a>(registrys: Vec<&'a mut RegistryTypes>) -> Vec<&'a mut PairTypes>{
    let mut pairs: Vec<&mut PairTypes> = Vec::new();
    for registry in registrys {
        match registry {
            RegistryTypes::PancakeRegistry(pancake_registry) => {
                pancake_registry.get_pairs();
                let pancake_pairs = pancake_registry.registry.pairs.as_mut().unwrap();
                pairs.extend(pancake_pairs);
            }
        }
    } 
    return pairs;
}

pub fn set_all_metadata(registrys: &mut HashMap<PairNames, RegistryTypes>) {
    for (_, registry) in registrys.iter_mut() {
        match registry {
            RegistryTypes::PancakeRegistry(registry) => {
                registry.get_metadata();
            }
        }
    }
}