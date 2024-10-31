use crate::Library;
use anyhow::{anyhow, Result};
use bstr::{io::BufReadExt, ByteSlice};
use hashbrown::HashMap;
use std::{fs::File, io::BufReader, path::Path};

/// Container to handle mapping of gene identifiers and
/// `sgRNA` identifiers
#[derive(Debug)]
pub struct GeneMap {
    map: HashMap<Vec<u8>, Vec<u8>>,
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
        let map = Self::build_from_file(path)?;
        Ok(Self { map })
    }

    /// Creates a new genemap from a buffer
    pub fn new_from_buffer<R: BufReadExt>(buffer: R) -> Result<Self> {
        let map = Self::build(buffer)?;
        Ok(Self { map })
    }

    /// Validates the provided path exists
    fn validate_path(path: &str) -> Result<()> {
        if Path::new(path).exists() {
            Ok(())
        } else {
            Err(anyhow!(
                "Provided gene mapping path doesn't exist: {}",
                path
            ))
        }
    }

    /// Builds the gene map from a file
    fn build_from_file(path: &str) -> Result<HashMap<Vec<u8>, Vec<u8>>> {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);
        Self::build(buffer)
    }

    /// Processes the tab delim file and panics if tabs are not found or duplicate `sgRNAs` are found
    fn build<R: BufReadExt>(mut buffer: R) -> Result<HashMap<Vec<u8>, Vec<u8>>> {
        let mut map = HashMap::new();
        buffer.for_byte_line(|line| {
            let pos = line
                .find_byte(b'\t')
                .unwrap_or_else(|| panic!("Missing '\t' in gene map"));
            let (gene, sgrna) = line.split_at(pos);
            assert!(
                map.insert(sgrna[1..].to_vec(), gene.to_vec()).is_none(),
                "Duplicate sgRNA key found in gene map: {}",
                std::str::from_utf8(&sgrna[1..]).expect("invalid utf8")
            );
            Ok(true)
        })?;
        Ok(map)
    }

    /// Gets the associated gene for a provided `sgRNA`
    #[must_use]
    pub fn get(&self, sgrna: &[u8]) -> Option<&Vec<u8>> {
        self.map.get(sgrna)
    }

    /// Validates that all aliases found within the library have an
    /// associated gene within this gene map
    ///
    /// If all aliases are found, returns None, otherwise returns the first alias that is not found
    #[must_use]
    pub fn missing_aliases(&self, library: &Library) -> Option<Vec<u8>> {
        library
            .values()
            .find(|alias| self.get(alias).is_none())
            .map(|alias| alias.to_vec())
        // library.values().all(|alias| self.get(alias).is_some())
    }
}

#[cfg(test)]
mod testing {
    use hashbrown::HashMap;

    use crate::Library;

    fn build_example_buffer() -> String {
        "gene1\tsgrna1\n\
         gene2\tsgrna2\n\
         gene3\tsgrna3\n"
            .to_string()
    }

    fn build_library() -> Library {
        let map = vec![
            (b"ACTG".to_vec(), b"sgrna1".to_vec()),
            (b"gtca".to_vec(), b"sgrna2".to_vec()),
            (b"TCAG".to_vec(), b"sgrna3".to_vec()),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>();
        Library::from_hashmap(map).unwrap()
    }

    fn build_invalid_library() -> Library {
        let map = vec![
            (b"ACTG".to_vec(), b"sgrna1".to_vec()),
            (b"gtca".to_vec(), b"sgrna4".to_vec()),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>();
        Library::from_hashmap(map).unwrap()
    }

    #[test]
    fn test_build() {
        let buffer = build_example_buffer();
        let genemap = super::GeneMap::new_from_buffer(buffer.as_bytes()).unwrap();
        assert_eq!(genemap.get(b"sgrna1").unwrap(), &b"gene1"[..]);
        assert_eq!(genemap.get(b"sgrna2").unwrap(), &b"gene2"[..]);
        assert_eq!(genemap.get(b"sgrna3").unwrap(), &b"gene3"[..]);
    }

    #[test]
    fn test_validate_library() {
        let buffer = build_example_buffer();
        let genemap = super::GeneMap::new_from_buffer(buffer.as_bytes()).unwrap();
        let library = build_library();
        assert!(genemap.validate_library(&library));
    }

    #[test]
    fn test_validate_library_invalid() {
        let buffer = build_example_buffer();
        let genemap = super::GeneMap::new_from_buffer(buffer.as_bytes()).unwrap();
        let library = build_invalid_library();
        assert!(!genemap.validate_library(&library));
    }

    #[test]
    fn test_from_file() {
        let filepath = "example/g2s.txt";
        let genemap = super::GeneMap::new(filepath).unwrap();
        assert_eq!(genemap.get(b"lib.0").unwrap(), &b"gene.0"[..]);
        assert_eq!(genemap.get(b"lib.99").unwrap(), &b"gene.9"[..]);
    }
}
