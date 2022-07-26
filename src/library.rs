use anyhow;
use anyhow::Result;
use hashbrown::HashMap;
use fxread::{Record, FastxRead};

type FxReader = Box<dyn FastxRead<Item = Record>>;

/// Container for input library sequences.
pub struct Library {
    table: HashMap<Vec<u8>, Vec<u8>>,
    size: usize
}
impl Library {

    /// Creates a library from a [`fxread::FastxRead`] capable object.
    /// Reads the records from iterator then confirms that all values
    /// are of equivalent size.
    pub fn from_reader(reader: FxReader) -> Result<Self> {
        let table = Self::table_from_reader(reader);
        let size = Self::calculate_base_size(&table)?;
        Ok( Self { table, size } )
    }

    /// Publically exposes the internal [`HashMap`] and returns
    /// the optional value (AKA its sequence id/header) to a provided token. 
    #[must_use] pub fn contains(&self, token: &[u8]) -> Option<&Vec<u8>> {
        match self.table.contains_key(token) {
            true => self.alias(token),
            false => None
        }
    }

    /// Returns the alias to a sequence (AKA its sequence id / header)
    #[must_use] pub fn alias(&self, token: &[u8]) -> Option<&Vec<u8>> {
        self.table.get(token)
    }

    /// An iterator over the sequences within the library
    pub fn keys(&self) -> impl Iterator<Item = &Vec<u8>> {
        self.table.keys()
    }

    /// An iteratory over the aliases within the library
    pub fn values(&self) -> impl Iterator<Item = &Vec<u8>> {
        self.table.values()
    }

    /// The unique sequence size of all elements within the library
    #[must_use] pub fn size(&self) -> usize {
        self.size
    }

    /// Validates that all sequences are of equivalent length
    fn validate_unique_size(keys: Vec<&Vec<u8>>) -> bool {
        keys
            .windows(2)
            .map(|x| (x[0], x[1]))
            .all(|(x, y)| x.len() == y.len())
    }

    /// Returns the basepair size of one of the sequences
    fn get_key_size(table: &HashMap<Vec<u8>, Vec<u8>>) -> usize {
        table.keys().next().unwrap().len()
    }

    /// Validates that all sequences are of equivalent length and returns
    /// that length
    fn calculate_base_size(table: &HashMap<Vec<u8>, Vec<u8>>) -> Result<usize> {
        match Self::validate_unique_size(table.keys().collect()) {
                true => Ok(Self::get_key_size(table)),
                false => Err(anyhow::anyhow!("Library sequence sizes are inconsistent"))
            }
    }

    /// Main init iterator which reads in all sequences fromthe reader and
    /// imports them into the internal [`HashMap`]
    fn table_from_reader(reader: FxReader) -> HashMap<Vec<u8>, Vec<u8>> {
        reader
            .into_iter()
            .fold(
                HashMap::new(),
                |mut map, x| {
                    match map.insert(x.seq().to_owned(), x.id().to_owned()) {
                        Some(_) => panic!("Unexpected duplicate sequence in library found: {}", std::str::from_utf8(x.seq()).unwrap()),
                        None => map
                    }
                })
    }
}

#[cfg(test)]
mod test {

    use fxread::{FastaReader, FastxRead, Record};
    use super::Library;

    fn reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] = b">seq.0\nACTG\n";
        Box::new(FastaReader::new(sequence))
    }

    fn duplicate_reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] = b">seq.0\nACTG\n>seq.1\nACTG\n";
        Box::new(FastaReader::new(sequence))
    }

    #[test]
    fn build() {
        let library = Library::from_reader(reader()).unwrap();
        assert_eq!(library.size(), 4);
        assert_eq!(library.keys().count(), 1);
    }

    #[test]
    fn validate_contains() {
        let library = Library::from_reader(reader()).unwrap();
        assert_eq!(library.contains(b"ACTG").unwrap(), b"seq.0");
        assert_eq!(library.contains(b"ACTT"), None);
    }

    #[test]
    #[should_panic]
    fn duplicates() {
        Library::from_reader(duplicate_reader()).unwrap();
    }
}
