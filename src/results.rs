use anyhow::Result;
use super::{Counter, Library};
use std::{fs::File, io::Write};


/// Writes the results dataframe to the provided path
fn write_to_path(
        path: &str, iterable: impl Iterator<Item = String>, 
        columns: String) -> Result<()>
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
        columns: String) -> Result<()> 
{
    println!("{columns}");
    iterable
        .for_each(|x| println!("{x}"));
    Ok(())
}

/// Creates a Tab Delim String from a List of Names
fn generate_columns(
        names: &Vec<String>) -> String 
{
    names
        .iter()
        .fold(
            String::from("Guide"),
            |mut s, x| {
            s += &format!("\t{}", x);
            s
        })
}

/// Writes the results dataframe either to the provided path
/// or to stdout
pub fn write_results(
        path: Option<String>, 
        results: &Vec<Counter>,
        library: &Library,
        names: &Vec<String>) -> Result<()> 
{

    let iterable = library
        .values()
        .map(|alias| {
            results
                .iter()
                .fold(
                    String::from_utf8(alias.to_vec()).expect("invalid utf8"),
                    |mut accum, x| {
                    accum += &format!("\t{}", x.get_value(alias));
                    accum
                })
        });

    let columns = generate_columns(names);

    match path {
        Some(p) => write_to_path(&p, iterable, columns),
        None => write_to_stdout(iterable, columns)
    }
}
