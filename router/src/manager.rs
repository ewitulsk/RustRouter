
use crate::pairs::Pair;
struct Manager{
    managed_pairs: Vec<Pair>
}

impl Manager {
    fn add_pairs(&mut self, new_pairs: &mut Vec<Pair>) {
        self.managed_pairs.append(new_pairs);
    }
}