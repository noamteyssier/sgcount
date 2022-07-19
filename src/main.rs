//! sgcount
//!
//! # Summary
//! This is a commandline tool to count the frequency of sgRNAs
//! in a group of provided sequencing files. It is meant to replace
//! methods of exact sequence matching without sacrificing speed and
//! replace costly alignment scripts using bwa or bowtie to align to
//! a library.

#![warn(missing_docs)]
use clap::Parser;
use anyhow::Result;

/// Module for Sequence Library
pub mod library;

/// Module for Sequence Trimming
pub mod trimmer;

/// Module for Matching Sequences Against a Library
pub mod counter;

/// Module for Handling Results
pub mod results;

/// Module for Unambiguous One-Off Sequence Generation 
pub mod permutes;

pub use fxread::initialize_reader;
pub use library::Library;
pub use trimmer::Trimmer;
pub use counter::Counter;
pub use permutes::Permuter;
pub use results::write_results;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    /// Filepath of the library
    #[clap(short, long, value_parser)]
    library_path: String,

    /// Filepath(s) of fastx (fastq, fasta, *.gz) sequences to map
    #[clap(short, long, value_parser, min_values=1, required=true)]
    input_paths: Vec<String>,

    /// Output filepath [default: stdout]
    #[clap(short, long, value_parser)]
    output_path: Option<String>,

    /// Adapter Offset
    #[clap(short='n', long, value_parser, default_value="0")]
    offset: usize,

    /// Allow One Off Mismatch
    #[clap(short='m', long)]
    mismatch: bool
}

fn count(
    library_path: String,
    input_paths: Vec<String>,
    output_path: Option<String>,
    offset: usize,
    mismatch: bool) -> Result<()> {

    let library = Library::from_reader(
        initialize_reader(&library_path)?
        )?;
    let size = library.size();

    let permuter = match mismatch{
        true => Some(Permuter::new(library.keys())),
        false => None
    };

    let results: Vec<Counter> = input_paths
        .into_iter()
        .map(|x| initialize_reader(&x).unwrap())
        .map(|x| Trimmer::from_reader(x, offset, size))
        .map(|x| Counter::new(x, &library, &permuter))
        .collect();

    write_results(output_path, &results, &library)?;

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    count(
        args.library_path,
        args.input_paths,
        args.output_path,
        args.offset,
        args.mismatch)?;
    Ok(())
}
