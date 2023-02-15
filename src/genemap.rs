use std::{path::Path, fs::File, io::BufReader};
use hashbrown::HashMap;
use bstr::{io::BufReadExt, ByteSlice};
use anyhow::{anyhow, Result};
use crate::Library;

/// Container to handle mapping of gene identifiers and
/// `sgRNA` identifiers
pub struct GeneMap {
    map: HashMap<Vec<u8>, Vec<u8>>
}
impl GeneMap {
    
    /// Creates a new genemap from a hashmap
    /// Used for testing
    pub fn from_hashmap(map: HashMap<Vec<u8>, Vec<u8>>) -> Self {
        Self { map }
    }

    /// Creates a new genemap from a filepath
    pub fn new(path: &str) -> Result<Self> {
        Self::validate_path(path)?;
        let map = Self::build(path)?;
        Ok( Self { map } )
    }

    /// Validates the provided path exists
    fn validate_path(path: &str) -> Result<()> {
        if Path::new(path).exists() { 
            Ok(())
        } else {
            Err(anyhow!("Provided gene mapping path doesn't exist: {}", path))
        }
    }

    /// Processes the tab delim file and panics if tabs are not found or duplicate `sgRNAs` are found
    fn build(path: &str) -> Result<HashMap<Vec<u8>, Vec<u8>>> {
        let file = File::open(path)?;
        let mut buffer = BufReader::new(file);
        let mut map = HashMap::new();
        buffer
            .for_byte_line(|line| {
                let pos = line.find_byte(b'\t')
                    .unwrap_or_else(|| {panic!("Missing '\t' in gene map")});
                let (gene, sgrna) = line.split_at(pos);
                assert!(
                    map.insert(sgrna[1..].to_vec(), gene.to_vec()).is_none(),
                    "Duplicate sgRNA key found in gene map: {}", std::str::from_utf8(&sgrna[1..]).expect("invalid utf8")
                );
                Ok(true)
            })?;
        Ok(map)
    }

    /// Gets the associated gene for a provided `sgRNA`
    #[must_use] pub fn get(&self, sgrna: &[u8]) -> Option<&Vec<u8>> {
        self.map.get(sgrna)
    }

    /// Validates that all aliases found within the library have an
    /// associated gene within this gene map
    #[must_use] pub fn validate_library(&self, library: &Library) -> bool {
        library
            .values()
            .all(|alias| self.get(alias).is_some())
    }
}
