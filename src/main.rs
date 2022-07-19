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
use spinners::{Spinners, Spinner};

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

/// Module for Determining Entropy Offset of Reads
pub mod offsetter;

pub use fxread::initialize_reader;
pub use library::Library;
pub use trimmer::Trimmer;
pub use counter::Counter;
pub use permutes::Permuter;
pub use results::write_results;
pub use offsetter::entropy_offset;


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
    #[clap(short='n', long, value_parser)]
    offset: Option<usize>,

    /// Allow One Off Mismatch
    #[clap(short='m', long)]
    mismatch: bool,

    /// Number of Reads to Subsample in Determining Offset [default: 5000]
    #[clap(short='s', long)]
    subsample: Option<usize>,

    /// Does not show progress
    #[clap(short='q', long)]
    quiet: bool
}

fn count(
    library_path: String,
    input_paths: Vec<String>,
    output_path: Option<String>,
    offset: usize,
    mismatch: bool,
    quiet: bool) -> Result<()> {

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
        .map(|x| {
            let spinner = match quiet {
                true => None,
                false => Some(Spinner::with_timer(Spinners::Dots, "Processing Sample".to_string()))
            };
            let counter = Counter::new(x, &library, &permuter);
            match spinner {
                Some(mut s) => s.stop_and_persist("🗸", "Counted Sample".to_string()),
                None => {}
            };
            counter
        })
        .collect();

    write_results(output_path, &results, &library)?;

    Ok(())
}

fn calculate_offset(
        library_path: &String,
        input_paths: &Vec<String>,
        subsample: Option<usize>,
        quiet: bool) -> Result<usize> { 

    let subsample = match subsample{
        Some(n) => n,
        None => 5000
    };
    let spinner = match quiet {
        true => None,
        false => Some(Spinner::with_timer(Spinners::Dots, "Calculating Offset".to_string()))
    };
    let offset = entropy_offset(library_path, input_paths, subsample)?;
    match spinner {
        Some(mut s) => s.stop_and_persist("🗸", format!("Calculated Offset: {}bp", offset)),
        None => {}
    };
    Ok(offset)
}


fn main() -> Result<()> {
    let args = Args::parse();

    let offset = match args.offset {
        Some(o) => o,
        None => calculate_offset(&args.library_path, &args.input_paths, args.subsample, args.quiet)?
    };
    count(
        args.library_path,
        args.input_paths,
        args.output_path,
        offset,
        args.mismatch,
        args.quiet)?;
    Ok(())
}
