use std::collections::HashMap;
use super::{Trimmer, Library, Permuter};

pub struct Counter {
    results: HashMap<String, usize>
}
impl Counter {
    pub fn new(
        trimmer: Trimmer,
        library: &Library,
        permuter: &Option<Permuter>) -> Self {

        let results = Self::count(trimmer, library, permuter);
        Self { results }
    }

    fn check_library<'a>(token: &str, library: &'a Library) -> Option<&'a String> {
        library.contains(token)
    }

    fn check_permuter<'a>(token: &str, permuter: &'a Option<Permuter>) -> Option<&'a String> {
        match permuter {
            Some(p) => p.contains(token),
            None => None
        }

    }

    fn assign<'a>(token: &str, library: &'a Library, permuter: &Option<Permuter>) -> Option<&'a String> {
        match Self::check_library(token, library) {
            Some(s) => Some(s),
            None => match Self::check_permuter(token, permuter) {
                Some(s) => library.alias(s),
                None => None
            }
        }

    }

    fn count<'a>(trimmer: Trimmer, library: &Library, permuter: &Option<Permuter>) -> HashMap<String, usize> {
        trimmer
            .into_iter()
            .map(|x| x.seq().to_string())
            .filter_map(|x| Self::assign(&x, library, permuter))
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
    use super::{Library, Trimmer, Counter, Permuter};

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

    fn permuter() -> Permuter {
        Permuter::new(library().keys())
    }

    #[test]
    fn count_no_distance_no_permute() {
        let trimmer = trimmer(false);
        let library = library();
        let count = Counter::new(trimmer, &library, &None);
        assert_eq!(*count.get_value("seq.0"), 1);
    }

    #[test]
    fn count_no_distance_with_permute() {
        let trimmer = trimmer(true);
        let library = library();
        let count = Counter::new(trimmer, &library, &None);
        assert_eq!(*count.get_value("seq.0"), 0);
    }

    #[test]
    fn count_with_distance_no_permute() {
        let trimmer = trimmer(true);
        let library = library();
        let count = Counter::new(trimmer, &library, &None);
        assert_eq!(*count.get_value("seq.0"), 0);
    }

    #[test]
    fn count_with_distance_with_permute() {
        let trimmer = trimmer(true);
        let library = library();
        let permuter = permuter();
        let count = Counter::new(trimmer, &library, &Some(permuter));
        assert_eq!(*count.get_value("seq.0"), 1);
    }
}
