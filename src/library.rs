use std::collections::HashMap;
use anyhow::Result;
use anyhow;
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

    fn calculate_base_size(table: &HashMap<String, String>) -> Result<usize> {
        // determine base length
        let base_len = table
            .keys()
            .next()
            .expect("Empty Library")
            .len();

        // compare all keys to base length
        match table
            .keys()
            .map(|x| x.len())
            .all(|x| x == base_len) 
            {
                true => Ok(base_len),
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

    pub fn contains(&self, token: &str) -> bool {
        self.table.contains_key(token)
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
