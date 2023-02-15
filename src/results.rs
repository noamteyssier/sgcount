use anyhow::Result;
use crate::{Counter, Library, GeneMap};
use std::{fs::File, io::{Write, stdout}, fmt::Write as fmtWrite};


/// Writes the results to stdout / path
fn write(
    path: Option<String>,
    iterable: impl Iterator<Item = String>,
    columns: &str
    ) -> Result<()>
{
    let mut writer = match_output(path)?;
    writeln!(writer, "{}", columns)?;
    iterable
        .for_each(|x| {
            writeln!(writer, "{}", x).expect("IO error in results");
        });
    Ok(())
}

/// Assigns the writer to stdout or to a path
fn match_output(path: Option<String>) -> Result<Box<dyn Write>>
{
    match path {
        Some(p) => Ok(Box::new(File::create(p)?)),
        None => Ok(Box::new(stdout()))
    }
}

/// Creates a Tab Delim String from a List of Names
fn generate_columns(
        names: &[String],
        genemap: &Option<GeneMap>) -> String 
{
    names
        .iter()
        .enumerate()
        .fold(
            String::from("Guide"),
            |mut s, (idx, x)| {
            if idx == 0 && genemap.is_some() {
                write!(s, "\tGene").expect("unable to write to string");
            }
            write!(s, "\t{}", x).expect("unable to write to string");
            s
        })
}

/// Appends the alias's parent gene if a gene map is provided
fn append_gene(
    alias: &[u8],
    genemap: &Option<GeneMap>,
    idx: usize,
    accum: &mut String) 
{
    if idx > 0 { return }
    if let Some(g) = genemap {
        if let Some(gene) = g.get(alias) {
            write!(
                accum, 
                "\t{}", 
                std::str::from_utf8(gene).expect("invalid utf8")
            ).expect("unable to write to string");
        } else {
            panic!("Missing sgrna -> gene mapping");
        }
    }
}

/// appends a samples count for a provided alias to the growing string
fn append_count(
    alias: &[u8],
    counter: &Counter,
    accum: &mut String) 
{
    write!(
        accum, 
        "\t{}", 
        counter.get_value(alias)
    ).expect("unable to write to string");
}

/// Writes the results dataframe either to the provided path
/// or to stdout
pub fn write_results(
        path: Option<String>, 
        results: &[Counter],
        library: &Library,
        names: &[String],
        genemap: &Option<GeneMap>) -> Result<()> 
{

    let iterable = library
        .values()
        .map(|alias| {
            results
                .iter()
                .enumerate()
                .fold(
                    String::from_utf8(alias.clone()).expect("invalid utf8"),
                    |mut accum, (idx, x)| {
                    append_gene(alias, genemap, idx, &mut accum);
                    append_count(alias, x, &mut accum);
                    accum
                })
        });

    let columns = generate_columns(names, genemap);
    write(path, iterable, &columns)
}

#[cfg(test)]
mod testing {
    use hashbrown::HashMap;
    use super::*;

    fn build_counter() -> Counter {
        let map = vec![
            (b"gene1".to_vec(), 100),
            (b"gene2".to_vec(), 200)
        ].into_iter().collect::<HashMap<_, _>>();
        Counter::from_hashmap(map)
    }

    fn build_library() -> Library {
        let map = vec![
            (b"ACTG".to_vec(), b"gene1".to_vec()),
            (b"GTCA".to_vec(), b"gene2".to_vec())
        ].into_iter().collect::<HashMap<_, _>>();
        Library::from_hashmap(map).unwrap()
    }

    fn build_gene_map() -> GeneMap {
        let map = vec![
            (b"gene1".to_vec(), b"GENE1".to_vec()),
            (b"gene2".to_vec(), b"GENE2".to_vec())
        ].into_iter().collect::<HashMap<_, _>>();
        GeneMap::from_hashmap(map)
    }

    #[test]
    fn test_generate_columns() {
        let names = vec!["A".to_string(), "B".to_string()];
        let genemap = None;
        let columns = generate_columns(&names, &genemap);
        assert_eq!(columns, "Guide\tA\tB");
    }

    #[test]
    fn test_write_results() {
        let path = Some("test.txt".to_string());
        let results = vec![
            build_counter(),
            build_counter()
        ];
        let library = build_library();
        let genemap = build_gene_map();
        let names = ["sample1".to_string(), "sample2".to_string()];

        write_results(path, &results, &library, &names, &Some(genemap)).unwrap();
    }

    #[test]
    fn test_append_count() {
        let counter = build_counter();
        let mut accum = String::new();
        append_count(b"gene1", &counter, &mut accum);
        assert_eq!(accum, "\t100");
    }

    #[test]
    fn test_append_gene() {
        let genemap = build_gene_map();
        let mut accum = String::new();
        append_gene(b"gene1", &Some(genemap), 0, &mut accum);
        assert_eq!(accum, "\tGENE1");
    }

}
