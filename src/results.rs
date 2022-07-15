use anyhow::Result;
use super::{Counter, Library};
use std::{fs::File, io::Write};


/// Writes the results dataframe to the provided path
fn write_to_path(path: &str, iterable: impl Iterator<Item = String>) -> Result<()>
{
    iterable
        .map(|x| x + "\n")
        .fold(File::create(path)?, |mut accum, x| {
            accum.write(&x.into_bytes()).expect("IO error in results");
            accum
        });
    Ok(())
}

/// Writes the results dataframe to stdout
fn write_to_stdout(iterable: impl Iterator<Item = String>) -> Result<()> 
{
    iterable
        .for_each(|x| println!("{x}"));
    Ok(())
}

/// Writes the results dataframe either to the provided path
/// or to stdout
pub fn write_results(
        path: Option<String>, 
        results: &Vec<Counter>,
        library: &Library) -> Result<()> 
{

    let iterable = library
        .values()
        .map(|alias| {
            results
                .iter()
                .fold(String::from(alias), |mut accum, x| {
                    accum += &format!("\t{}", x.get_value(alias));
                    accum
                })
        });

    match path {
        Some(p) => write_to_path(&p, iterable),
        None => write_to_stdout(iterable)
    }
}
