use std::{collections::HashMap};
use aptos_sdk::types::account_config::token;
use async_trait::async_trait;

use crate::{
    types::{Network}, 
    pairs::{
        Pair, 
        liquidswap_pair::{LiquidswapPair, CurveType, LiquidswapMetadata},
        PairNames, PairMetadata
    },
    utils::{query_aptos_events_raw, string_to_u64, query_aptos_resources_all_raw}};
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use regex::Regex;
use super::{Registry};


// {
//     "type": "0x163df34fccbf003ce219d3f1d9e70d140b60622cb9dd47599c25fb2f797ba6e::liquidity_pool::LiquidityPool<0x61ed8b048636516b4eaf4c74250fa4f9440d9c3e163d96aeb863fe658a4bdc67::CASH::CASH, 0x1::aptos_coin::AptosCoin, 0x163df34fccbf003ce219d3f1d9e70d140b60622cb9dd47599c25fb2f797ba6e::curves::Uncorrelated>",
//     "data": {
//       "coin_x_reserve": {
//         "value": "59023549902940"
//       },
//       "coin_y_reserve": {
//         "value": "522663094699"
//       },
//       "dao_fee": "33",
//       "fee": "30",
//       "last_block_timestamp": "1735063209",
//       "last_price_x_cumulative": "563957850866252598077788",
//       "last_price_y_cumulative": "53354636400856397559961483221",
//       "locked": false,
//       "lp_burn_cap": {
//         "dummy_field": false
//       },
//       "lp_coins_reserved": {
//         "value": "1000"
//       },
//       "lp_mint_cap": {
//         "dummy_field": false
//       },
//       "x_scale": "0",
//       "y_scale": "0"
//     }
//   },

#[derive(Clone, Serialize, Deserialize)]
pub struct LiquidswapRegistry {
    pool_address: String,
    module_address: String,
    protocol: String
}

#[async_trait]
impl Registry for LiquidswapRegistry {
    fn module_address(&self) -> &str {
        return &self.module_address;
    }

    fn protocol(&self) -> PairNames {
        return PairNames::LiquidswapPair;
    }

    async fn get_pairs(&self, network: &Network) -> Vec<Box<dyn Pair>>{
        println!("Getting Liquidswap Pairs...");
        let network_http = &network.http[..];
        let network_name = &network.name[..];
    
        let account = "0x61d2c22a6cb7831bee0f48363b0eec92369357aece0d1142062f7d5d85c7bef8";
        
        let all_resources_raw = query_aptos_resources_all_raw(network_http, account).await;
        let all_resources:Vec<Value> = serde_json::from_str(&all_resources_raw).unwrap();
        let mut liquidity_pool_resources: Vec<Value> = vec![];

        for res in all_resources {
            let type_str = res.get("type").unwrap().clone();
            if(type_str.as_str().unwrap().contains(&"::liquidity_pool::LiquidityPool".to_string())){
                liquidity_pool_resources.push(res.clone());
            }
        }

        println!("Liq Pool Length: {}", liquidity_pool_resources.len());
        let re = Regex::new(r"(.*)::liquidity_pool::LiquidityPool<([^,]+),\s*([^,]+),\s*([^>]+)>").unwrap();
    
        let mut liquidswap_pairs: Vec<Box<dyn Pair>> = vec![];
        for pair_data in liquidity_pool_resources {
            let type_str = pair_data.get("type").unwrap().clone();
            let captures = re.captures(type_str.as_str().unwrap()).expect("Captures Failed");
            let token_x = captures.get(2).unwrap().as_str().to_string();
            let token_y = captures.get(3).unwrap().as_str().to_string();
            let curve_str = captures.get(4).unwrap().as_str().to_string();
            let curve_type = if curve_str.contains("Uncorrelated") { CurveType::Uncorrelated } else { CurveType::Stable };
            let token_x_reserve_str = pair_data.get("data").unwrap().get("coin_x_reserve").unwrap().get("value").unwrap().as_str().unwrap().to_string();
            let token_y_reserve_str = pair_data.get("data").unwrap().get("coin_y_reserve").unwrap().get("value").unwrap().as_str().unwrap().to_string();
            let token_x_reserve = token_x_reserve_str.parse::<u64>().unwrap();
            let token_y_reserve = token_y_reserve_str.parse::<u64>().unwrap();
            let x_scale_str = pair_data.get("data").unwrap().get("x_scale").unwrap().as_str().unwrap().to_string();
            let y_scale_str = pair_data.get("data").unwrap().get("y_scale").unwrap().as_str().unwrap().to_string();
            let x_scale = x_scale_str.parse::<u64>().unwrap();
            let y_scale = y_scale_str.parse::<u64>().unwrap();
            let fee_str = pair_data.get("data").unwrap().get("fee").unwrap().as_str().unwrap().to_string();
            let fee = fee_str.parse::<u64>().unwrap();
            let dao_fee_str = pair_data.get("data").unwrap().get("dao_fee").unwrap().as_str().unwrap().to_string();
            let dao_fee = dao_fee_str.parse::<u64>().unwrap();

            let pair_key = format!("{}{}{}", account, token_x, token_y);

            let liquidswap_pair = LiquidswapPair {
                network: String::from(network_name),
                protocol: String::from("liquidswap_constant_product"),
                pair_name: PairNames::PancakePair,
                pair_key: String::from(pair_key),
                pool_addr: String::from(account),
                token_arr: Vec::from([token_x.clone(),token_y]),
                curve_type: curve_type,
                x_scale: x_scale,
                y_scale: y_scale,
                router_pair_addr: String::new(),
                fee: fee,
                dao_fee: dao_fee,
                metadata: LiquidswapMetadata { reserves: vec![token_x_reserve, token_y_reserve] }
            };
            
            liquidswap_pairs.push(Box::new(liquidswap_pair));
            
        }
        return liquidswap_pairs;
    }

    async fn get_metadata(&self, network: &Network, metadata_map: &mut HashMap<PairNames, HashMap<String, Box<dyn PairMetadata>> >){
        println!("Inserting Liquidswap Hashmap.");
        metadata_map.insert(PairNames::LiquidswapPair, HashMap::new());
    }

    fn build_metadata_map_from_changes(&self, changes: Vec<Value>) -> HashMap<String, Box<dyn PairMetadata>> {

        println!("Building Liquidswap Metadata From Changes...");

        let mut metadata_map: HashMap<String, Box<dyn PairMetadata>> = HashMap::new();

        // \\\"{\\\"address\\\":\\\"([^"]+)\\\"(.*)\{\\\"type\\\":\\\"([^"]+)\\\"(.*)\\\"reserve_x\\\":\\\"([^"]+)\\\"(.*)\\\"reserve_y\\\":\\\"([^"]+)\\\"(.*)/gm

        for change in changes {
            let re = Regex::new(r#""address":"([^"]+)".*?"type":"([^"]+)".*?"coin_x_reserve":.*?"value":"([^"]+)".*?"coin_y_reserve":.*?"value":"([^"]+)""#).unwrap();
            let type_re = Regex::new(r#".*?::liquidity_pool::LiquidityPool<([^"]+), ([^"]+),.*?::curves::([^"]+)>.*?"#).unwrap();
            if let Some(captures) = re.captures(change.to_string().as_str()){
                if let (Some(address), Some(type_str), Some(reserve_x), Some(reserve_y)) = (
                    captures.get(1).map(|m| m.as_str()),
                    captures.get(2).map(|m| m.as_str()),
                    captures.get(3).map(|m| m.as_str()),
                    captures.get(4).map(|m| m.as_str())
                ) {
                    // println!("Captured First");
                    if let Some(type_captures) = type_re.captures(type_str) {
                        if let (Some(token_x), Some(token_y), Some(curve)) = (
                            type_captures.get(1).map(|m| m.as_str()),
                            type_captures.get(2).map(|m| m.as_str()),
                            type_captures.get(3).map(|m| m.as_str())
                        ){
                            let identifier = format!("{},{},{}", token_x, token_y, curve);
                            
                            // println!("Identifier: {}", identifier);

                            let res_x = reserve_x.parse::<u64>().unwrap();
                            let res_y = reserve_y.parse::<u64>().unwrap();
    
                            let metadata = LiquidswapMetadata {
                                reserves: vec![res_x, res_y]
                            };
                
                            metadata_map.insert(identifier.clone(), Box::new(metadata));
                        }
                    }
                }
            }
        }

        return metadata_map;
        // return HashMap::new()
    }
}


