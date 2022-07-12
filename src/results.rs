use std::{fs::File, io::Write};
use anyhow::Result;
use super::{Counter, Library};

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

fn write_to_stdout(iterable: impl Iterator<Item = String>) -> Result<()> 
{
    iterable
        .for_each(|x| println!("{x}"));
    Ok(())
}

pub fn write_results(
        path: &str, 
        results: &Vec<Counter>,
        library: &Library) -> Result<()> 
{

    let iterable = library
        .keys()
        .map(|key| {
            results
                .iter()
                .fold(String::from(library.alias(key).unwrap()), |mut accum, x| {
                    accum += &format!("\t{}", x.get_value(key));
                    accum
                })
        });

    match path.is_empty() {
        false => write_to_path(path, iterable),
        true => write_to_stdout(iterable)
    }
}
