use std::collections::HashMap;
use super::{Trimmer, Library};

pub struct Counter {
    results: HashMap<String, usize>
}
impl Counter {
    pub fn new(
        trimmer: Trimmer,
        library: &Library) -> Self {

        let results = Self::count(trimmer, library);
        Self { results }
    }

    fn count(trimmer: Trimmer, library: &Library) -> HashMap<String, usize> {
        trimmer
            .into_iter()
            .map(|x| x.seq().to_string())
            .filter(|x| library.contains(x))
            .fold(HashMap::new(), |mut accum, x| {
                *accum.entry(x).or_insert(0) += 1;
                accum
            })
    }

    pub fn get_value(&self, token: &str) -> &usize {
        match self.results.get(token) {
            Some(c) => c,
            None => &0
        }
    }

}
