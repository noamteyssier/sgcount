
use std::{collections::HashMap, fs::File, fmt::Display, io::Write};
use anyhow::Result;

pub fn write_results<A: Display, B: Display>(path: &str, results: &HashMap<A, B>) -> Result<()> {
    results
        .iter()
        .map(|(k, v)| format!("{}\t{}\n", k, v))
        .fold(File::create(path)?, |mut accum, x| {
            accum.write(&x.into_bytes()).expect("error writing to file");
            accum
        });
    Ok(())
}
