use anyhow::Result;

mod library;
mod trimmer;
mod counter;
mod utils;

use fxread::initialize_reader;
use library::Library;
use trimmer::Trimmer;
use counter::Counter;
use utils::write_results;


fn main() -> Result<()> {
    let output_path = "test_counts.tab";
    let lib_path = "example/library.fa";
    let lib_reader = initialize_reader(lib_path)?;
    let lib = Library::from_reader(lib_reader)?;

    let seq_path = "example/sequence.fq";
    let offset = 0;
    let size = lib.size();
    
    let seq_reader = initialize_reader(seq_path)?;
    let trim = Trimmer::from_reader(seq_reader, offset, size);

    let counter = Counter::new(trim, lib);

    match output_path.is_empty() {
        true => counter.pprint(),
        false => write_results(output_path, counter.results()).unwrap()
    }

    Ok(())
}
