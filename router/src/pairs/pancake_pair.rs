use crate::utils::{get_network, query_aptos_resources_raw, string_to_u128, string_to_u64};

use super::{Descriptor, Pair, OutputAmount, Refresh, Metadata};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PancakePair {
    pub base: Pair,
    pub metadata: Option<Metadata>
}

// impl Descriptor for PancakePair {
//     fn to_json(&self) -> String {
//         String::new()
//     }
// }

// impl OutputAmount for PancakePair {
//     fn output_amount(&self) -> u64 {
//         const PRECISION: u64 = 10000;
//         const MAX_U128: u128 = 340282366920938463463374607431768211455;


//     }
// }

impl Refresh for PancakePair {
    fn refresh_pair(&mut self) {
        let network = get_network(self.base.network.clone()).unwrap();

        let network_http = &network.http[..];

        let account = "0xc7efb4076dbe143cbcd98cfaaa929ecfc8f299203dfff63b95ccb6bfe19850fa";
        let resource = format!("{}::swap::TokenPairMetadata<{}, {}>", account, self.base.token_arr[0], self.base.token_arr[1]);
        
        let raw_data = query_aptos_resources_raw(network_http, account, &resource);

        let val: serde_json::Value = serde_json::from_str(&raw_data).unwrap();

        let data = val.get("data").unwrap();
        let last_k = data.get("k_last").unwrap().as_str().unwrap().parse::<u128>().unwrap(); //Who needs error handling?
        let bal_x = data.get("balance_x").unwrap().get("value").unwrap().as_str().unwrap().parse::<u64>().unwrap();
        let bal_y = data.get("balance_y").unwrap().get("value").unwrap().as_str().unwrap().parse::<u64>().unwrap();

        let metadata = Metadata {
            last_k: last_k,
            reserves: vec![bal_x, bal_y]
        };

        self.metadata = Some(metadata);
    }
}