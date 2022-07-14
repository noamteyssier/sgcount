use anyhow;
use anyhow::Result;
use std::collections::HashMap;
use fxread::{Record, FastxRead};

type FxReader = Box<dyn FastxRead<Item = Record>>;

pub struct Library {
    table: HashMap<String, String>,
    size: usize
}
impl Library {

    pub fn from_reader(reader: FxReader) -> Result<Self> {
        let table = Self::table_from_reader(reader);
        let size = Self::calculate_base_size(&table)?;
        Ok( Self { table, size } )
    }

    fn validate_unique_size(keys: Vec<&String>) -> bool {
        keys
            .windows(2)
            .map(|x| (x[0], x[1]))
            .all(|(x, y)| x.len() == y.len())
    }

    fn get_key_size(table: &HashMap<String, String>) -> usize {
        table.keys().next().unwrap().len()
    }

    fn calculate_base_size(table: &HashMap<String, String>) -> Result<usize> {
        match Self::validate_unique_size(table.keys().collect()) {
                true => Ok(Self::get_key_size(table)),
                false => Err(anyhow::anyhow!("Library sequence sizes are inconsistent"))
            }
    }

    fn table_from_reader(reader: FxReader) -> HashMap<String, String> {
        reader
            .into_iter()
            .map(|x| (
                    x.seq().to_string(), 
                    x.id().to_string()))
            .collect()
    }

    pub fn contains(&self, token: &str) -> Option<&String> {
        match self.table.contains_key(token) {
            true => self.alias(token),
            false => None
        }
    }

    pub fn alias(&self, token: &str) -> Option<&String> {
        self.table.get(token)
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.table.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &String> {
        self.table.values()
    }

    pub fn size(&self) -> usize {
        self.size
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

    #[test]
    fn build() {
        let library = Library::from_reader(reader()).unwrap();
        assert_eq!(library.size(), 4);
        assert_eq!(library.keys().count(), 1);
    }

    #[test]
    fn validate_contains() {
        let library = Library::from_reader(reader()).unwrap();
        assert_eq!(library.contains("ACTG"), Some(&String::from("seq.0")));
        assert_eq!(library.contains("ACTT"), None);
    }
}
