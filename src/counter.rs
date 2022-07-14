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

#[cfg(test)]
mod test {

    use fxread::{FastaReader, FastxRead, Record};
    use super::{Library, Trimmer, Counter};

    fn trim_reader(distance: bool) -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] = match distance {
            false => b">seq.0\nACTG\n",
            true => b">seq.0\nAGTG\n",
        };
        Box::new(FastaReader::new(sequence))
    }

    fn lib_reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] = b">seq.0\nACTG\n";
        Box::new(FastaReader::new(sequence))
    }

    fn trimmer(distance: bool) -> Trimmer {
        Trimmer::from_reader(trim_reader(distance), 0, 4)
    }

    fn library() -> Library {
        Library::from_reader(lib_reader()).unwrap()
    }

    #[test]
    fn count_no_distance() {
        let no_distance_trimmer = trimmer(false);
        let distance_trimmer = trimmer(true);
        let library = library();
        let no_distance_count = Counter::new(no_distance_trimmer, &library, 0);
        let distance_count = Counter::new(distance_trimmer, &library, 0);
        assert_eq!(*no_distance_count.get_value("seq.0"), 1);
        assert_eq!(*distance_count.get_value("seq.0"), 0);
    }

    #[test]
    fn count_with_distance() {
        let no_distance_trimmer = trimmer(false);
        let distance_trimmer = trimmer(true);
        let library = library();
        let no_distance_count = Counter::new(no_distance_trimmer, &library, 1);
        let distance_count = Counter::new(distance_trimmer, &library, 1);
        assert_eq!(*no_distance_count.get_value("seq.0"), 1);
        assert_eq!(*distance_count.get_value("seq.0"), 1);
    }

}
