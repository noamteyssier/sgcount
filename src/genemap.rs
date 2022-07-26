use std::{path::Path, fs::File, io::BufReader};
use hashbrown::HashMap;
use bstr::{io::BufReadExt, ByteSlice};
use anyhow::{anyhow, Result};

/// Container to handle mapping of gene identifiers and
/// sgRNA identifiers
pub struct GeneMap {
    map: HashMap<Vec<u8>, Vec<u8>>
}
impl GeneMap {
    /// Creates a new genemap from a filepath
    pub fn new(path: &str) -> Result<Self> {
        Self::validate_path(path)?;
        let map = Self::build(path)?;
        Ok( Self { map } )
    }

    fn validate_path(path: &str) -> Result<()> {
        if Path::new(path).exists() { 
            Ok(())
        } else {
            Err(anyhow!("Provided gene mapping path doesn't exist: {}", path))
        }
    }

    fn build(path: &str) -> Result<HashMap<Vec<u8>, Vec<u8>>> {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);
        let mut map = HashMap::new();
        buffer
            .for_byte_line(|line| {
                let pos = line.find_byte(b'\t')
                    .unwrap_or_else(|| {panic!("Missing '\t' in gene map")});
                let (gene, sgrna) = line.split_at(pos);
                assert!(
                    map.insert(sgrna.to_vec(), gene.to_vec()).is_some(),
                    "Duplicate sgRNA key found in gene map: {}", std::str::from_utf8(sgrna).expect("invalid utf8")
                );
                Ok(true)
            })?;
        Ok(map)
    }
}
