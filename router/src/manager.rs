
use crate::pairs::{Pair, PairTypes};
pub struct Manager{
    pub managed_pairs: Vec<PairTypes>
}

impl Manager {
    pub fn empty() -> Manager {
        Manager {
            managed_pairs: Vec::new()
        }
    }

    pub fn add_pairs(&mut self, new_pairs: Vec<PairTypes>) {
        self.managed_pairs.append(&mut (new_pairs.clone()));
    }
}