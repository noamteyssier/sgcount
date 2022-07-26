use anyhow::Result;
use super::{Counter, Library};
use std::{fs::File, io::Write, fmt::Write as fmtWrite};


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

/// Writes the results dataframe either to the provided path
/// or to stdout
pub fn write_results(
        path: Option<String>, 
        results: &[Counter],
        library: &Library,
        names: &[String]) -> Result<()> 
{

    let iterable = library
        .values()
        .map(|alias| {
            results
                .iter()
                .fold(
                    String::from_utf8(alias.to_vec()).expect("invalid utf8"),
                    |mut accum, x| {
                    write!(accum, "\t{}", x.get_value(alias)).expect("unable to write to string");
                    accum
                })
        });

    let columns = generate_columns(names);

    match path {
        Some(p) => write_to_path(&p, iterable, columns),
        None => write_to_stdout(iterable, columns)
    }
}
