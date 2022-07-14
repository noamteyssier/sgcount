use std::collections::HashMap;
use super::{Trimmer, Library};

pub struct Counter {
    results: HashMap<String, usize>
}
impl Counter {
    pub fn new(
        trimmer: Trimmer,
        library: &Library,
        distance: usize) -> Self {

        let results = Self::count(trimmer, library, distance);
        Self { results }
    }

    fn count(trimmer: Trimmer, library: &Library, distance: usize) -> HashMap<String, usize> {
        trimmer
            .into_iter()
            .map(|x| x.seq().to_string())
            .filter_map(|x| library.contains(&x, distance))
            .fold(HashMap::new(), |mut accum, x| {
                *accum.entry(x.to_string()).or_insert(0) += 1;
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
