use anyhow;
use anyhow::Result;
use std::collections::HashMap;
use fxread::{Record, FastxRead};
use super::hamming_distance;

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

    fn hamming_match(&self, token: &str, distance: usize) -> Option<&String> {
        if distance > 0 {
            self.keys()
                .find(|x| hamming_distance(x, token) <= 1)
        } else {
            None
        }
    }

    pub fn contains(&self, token: &str, distance: usize) -> Option<&String> {
        match self.table.contains_key(token) {
            true => self.alias(token),
            false => match self.hamming_match(token, distance) {
                Some(s) => self.alias(s),
                _ => None
            }
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
