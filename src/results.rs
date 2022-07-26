use anyhow::Result;
use crate::GeneMap;

use super::{Counter, Library};
use std::{fs::File, io::Write, fmt::Write as fmtWrite};


/// Writes the results dataframe to the provided path
fn write_to_path(
        path: &str, iterable: impl Iterator<Item = String>, 
        columns: &str) -> Result<()>
{
    let mut file = File::create(path)?;
    writeln!(file, "{}", columns)?;
    iterable
        .for_each(|x| {
            writeln!(file, "{}", x).expect("IO error in results");
        });
    Ok(())
}

/// Writes the results dataframe to stdout
fn write_to_stdout(
        iterable: impl Iterator<Item = String>, 
        columns: &str) 
{
    println!("{columns}");
    iterable
        .for_each(|x| println!("{x}"));
}

/// Creates a Tab Delim String from a List of Names
fn generate_columns(
        names: &[String]) -> String 
{
    names
        .iter()
        .fold(
            String::from("Guide"),
            |mut s, x| {
            write!(s, "\t{}", x).expect("unable to write to string");
            s
        })
}

/// Appends the alias's parent gene if a gene map is provided
fn append_gene(
    alias: &[u8],
    genemap: &Option<GeneMap>,
    accum: &mut String) 
{
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
    write!(accum, "\t{}", counter.get_value(alias)).expect("unable to write to string");
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
                .fold(
                    String::from_utf8(alias.clone()).expect("invalid utf8"),
                    |mut accum, x| {
                    append_gene(alias, genemap, &mut accum);
                    append_count(alias, x, &mut accum);
                    accum
                })
        });

    let columns = generate_columns(names);

    if let Some(p) = path {
        write_to_path(&p, iterable, &columns)
    } else {
        write_to_stdout(iterable, &columns);
        Ok(())
    }
}
