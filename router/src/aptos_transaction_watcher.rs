use std::sync::mpsc::{self, Sender};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use serde_json::Value;
use crate::utils::{query_aptos_transactions_by_version, get_aptos_version};
use crate::types::{Network, ChannelUpdateMetadata};
use crate::registrys::Registry;
use crate::types::ChannelRegistrysToWatch;

pub async fn aptos_watch_transactions(network: &Network, starting_version: u64, tothread_updater_tx: &Sender<ChannelUpdateMetadata>) {
    let (fromthread_tx, fromthread_rx) = mpsc::channel::<ChannelRegistrysToWatch>();

    tothread_updater_tx.send(
        ChannelUpdateMetadata{new_metadata: None, channel_tx: Some(fromthread_tx)}
    ).unwrap();

    let registrys_to_watch = fromthread_rx.recv().unwrap().registrys_to_watch;
    println!("Registrys to watch: {:?}", registrys_to_watch);
    let mut last_version = 0;
    let mut cur_version = starting_version;
    // let mut cur_version = 2085727565; <--- Test Start. Has pancake swap in next version.
    while(true){
        let mut changes_map = HashMap::<String, Vec<Value>>::new();
        let transactions = query_aptos_transactions_by_version(&network.http, cur_version, 10000).await;
        transactions.iter().for_each(|tx| {
            let changes: Vec<Value> = tx.get("changes").unwrap().as_array().unwrap().to_vec();
            changes.iter().for_each(|change| {
                if(change.get("address").is_some()){
                    let address = change.get("address").unwrap().to_string().strip_prefix("\"").unwrap().strip_suffix("\"").unwrap().to_string();
                    
                    if(!changes_map.contains_key(&address)){
                        changes_map.insert(address, vec![change.clone()]);
                    } else {
                        changes_map.get_mut(&address).unwrap().push(change.clone());
                    }
                }
            });
        });

        registrys_to_watch.iter().for_each(|registry| {
            if(changes_map.contains_key(registry)){ 
                let registry_changes = changes_map.get(registry).unwrap();
                tothread_updater_tx.send(
                    ChannelUpdateMetadata{new_metadata: Some(registry_changes.clone()), channel_tx: None}
                ).unwrap();
            }
        });


        last_version = cur_version;
        cur_version = transactions[transactions.len() - 1].get("version").unwrap().as_str().unwrap().parse::<u64>().unwrap();
        
        println!("Found {} new transactions", transactions.len());
        
        cur_version += 1;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}