mod pancake_pair;

pub trait Descriptor {
    fn get_descriptor(&self) -> String;
}

pub struct Pair {
    protocol: String,
    swap_type: String,
    pair_key: String,
    pool_addr: String,
    token_arr: Vec<String>,
    router_pair_add: String,
}