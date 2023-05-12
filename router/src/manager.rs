
use crate::pairs::{Pair, PairTypes, Refresh};
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

    pub fn refresh_pairs(&mut self) {
        let mut count = 0;
        for pair in self.managed_pairs.iter_mut() {
            println!("Refreshing: {}", count);
            count += 1;
            pair.refresh_pair();
        }
    }
}