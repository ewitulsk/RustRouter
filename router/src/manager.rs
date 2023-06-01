
// use std::{collections::HashMap};

// use crate::{pairs::{Pair, PairTypes, PairNames, pancake_pair::{PancakePair, PancakeMetadata}}, registrys::{Registry, set_all_metadata, gen_all_pairs}};
// pub struct Manager<'a>{
//     pub registrys: HashMap<PairNames, RegistryTypes>,
//     pub managed_pairs: Option<Vec<&'a mut PairTypes>>,
//     // pub pairs_by_token: Option<HashMap<String, Vec<&'a PairTypes>>>
// }

// /*
// All token metadata is held in a protocols registry, to "refresh" a pair, the manager tells the registry to refresh it's metadata.
// The manager then iterates it's managed pairs and re-assigns the pairs metadata to that stored in the registry's metadata hashmap.


//  */

// impl<'a> Manager<'a> {
//     /*TODO: Add token list filters */
//     pub fn empty(init_registrys: HashMap<PairNames, RegistryTypes>) -> Manager<'a> {
        
//         // let pairs_by_token: HashMap<String, Vec<&PairTypes>> = gen_pairs_by_token_map(&pairs);
//         Manager {
//             managed_pairs: None,
//             registrys: init_registrys,
//             // pairs_by_token: None
//         }
//     }

//     pub fn init_pairs(&'a mut self){
//         let mut registrys: Vec<&mut RegistryTypes> = Vec::new();

//         for (_, registry) in &mut self.registrys {
//             registrys.push(registry);
//         }

//         let pairs: Vec<&'a mut PairTypes> = gen_all_pairs(registrys);
//         self.managed_pairs = Some(pairs);
//     }

//     // pub fn intit_pairs_by_token(&self) {
        
//     // }

//     /*Disallowing adding pairs for now, to much confusion on how exactly the pair will get it's metadata. 
//     Right now all pairs will be added via the registrys */
//     // pub fn add_pairs(&mut self, new_pairs: Vec<PairTypes>) {
//     //     self.managed_pairs.append(&mut (new_pairs.clone()));
//     // }

//     // pub fn refresh_pairs(&mut self) {
//     //     set_all_metadata(&mut self.registrys);

//     //     for pair in self.managed_pairs.as_mut().unwrap() {

//     //         match pair {
//     //             PairTypes::PancakePair(pair) => {
//     //                 let pancake_registry = self.registrys.get(&PairNames::PancakePair).unwrap();

//     //                 //This registry type is already know, I just don't know if it's possible to cast an enum to a known type.
//     //                 match pancake_registry {
//     //                     RegistryTypes::PancakeRegistry( pancake_registry) => {
//     //                         let pancake_metadata_map: &HashMap<String, PancakeMetadata> = &pancake_registry.metadata_map;
//     //                         let metadata_key = format!("<{}, {}>", &pair.base.token_arr[0], &pair.base.token_arr[1]);

//     //                         let pair_metadata = pancake_metadata_map.get(&metadata_key).unwrap();

//     //                         pair.metadata = pair_metadata.clone();
//     //                     }
//     //                 }
//     //             }
//     //         }
//     //     }
//     // }

//     // pub fn get_pairs_from_token(&self, token: String) -> &Vec<&PairTypes> {
//     //     return self.pairs_by_token.unwrap().get(&token).unwrap();
//     // }

// }
