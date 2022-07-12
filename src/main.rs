use anyhow::Result;

mod library;
mod trimmer;
mod counter;
mod results;

use fxread::initialize_reader;
use library::Library;
use trimmer::Trimmer;
use counter::Counter;
use results::write_results;


fn main() -> Result<()> {
    let output_path = "test_counts.tab";

    let lib_path = "example/library.fa";
    let lib_reader = initialize_reader(lib_path)?;
    let lib = Library::from_reader(lib_reader)?;

    let seq_path = vec!["example/sequence.fq", "example/sequence.fq"];
    let offset = 0;
    let size = lib.size();
    
    let results: Vec<Counter> = seq_path
        .into_iter()
        .map(|x| initialize_reader(x).unwrap())
        .map(|x| Trimmer::from_reader(x, offset, size))
        .map(|x| Counter::new(x, &lib))
        .collect();

    write_results(output_path, &results, &lib)?;

    Ok(())
}
