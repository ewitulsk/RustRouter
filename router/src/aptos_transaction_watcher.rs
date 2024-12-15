use crate::utils::{query_aptos_transactions_by_version, get_aptos_version};
use crate::types::Network;

pub async fn aptos_watch_transactions(network: &Network) {
    let mut last_version = 0;
    let mut cur_version = get_aptos_version(&network.http).await.unwrap();
    while(true){
        let transactions = query_aptos_transactions_by_version(&network.http, cur_version, 10000).await;
        
        last_version = cur_version;
        cur_version = transactions[transactions.len() - 1].get("version").unwrap().as_str().unwrap().parse::<u64>().unwrap();
        
        println!("Found {} new transactions", transactions.len());
        
        cur_version += 1;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}