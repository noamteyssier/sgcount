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

/// Module for Performing Individual Sample Counting
pub mod count;

pub use fxread::initialize_reader;
pub use library::Library;
pub use trimmer::Trimmer;
pub use counter::Counter;
pub use permutes::Permuter;
pub use offsetter::entropy_offset;
pub use count::count;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    /// Filepath of the library
    #[clap(short, long, value_parser)]
    library_path: String,

    /// Filepath(s) of fastx (fastq, fasta, *.gz) sequences to map
    #[clap(short, long, value_parser, min_values=1, required=true)]
    input_paths: Vec<String>,

    /// Sample Names
    #[clap(short='n', long, value_parser, min_values=1, required=false)]
    sample_names: Option<Vec<String>>,

    /// Output filepath [default: stdout]
    #[clap(short, long, value_parser)]
    output_path: Option<String>,

    /// Adapter Offset
    #[clap(short='a', long, value_parser)]
    offset: Option<usize>,

    /// Allow One Off Mismatch
    #[clap(short='m', long)]
    mismatch: bool,

    /// Number of Reads to Subsample in Determining Offset [default: 5000]
    #[clap(short='s', long)]
    subsample: Option<usize>,

    /// Number of Threads to Use for Parallel Jobs
    #[clap(short='t', long, default_value="1")]
    threads: usize,

    /// Does not show progress
    #[clap(short='q', long)]
    quiet: bool
}

/// Sets the number of threads globally
fn set_threads(threads: usize) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .unwrap();
}

/// Generates default sample names
fn generate_sample_names(
        input_paths: &Vec<String>) -> Vec<String> {
    input_paths
        .iter()
        .enumerate()
        .map(|(idx, _)| format!("Sample.{:?}", idx))
        .collect()
}

/// Calculates Offset if Required
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

    set_threads(args.threads);

    // generates sample names if required
    let sample_names = match args.sample_names {
        Some(s) => if s.len() != args.input_paths.len() { 
                panic!("Must provide as many sample names as there are input files") 
            } else { s },
        None => generate_sample_names(&args.input_paths)
    };

    // calculates offset if required
    let offset = match args.offset {
        Some(o) => o,
        None => calculate_offset(&args.library_path, &args.input_paths, args.subsample, args.quiet)?
    };

    // perform counting
    count(
        args.library_path,
        args.input_paths,
        sample_names,
        args.output_path,
        offset,
        args.mismatch,
        args.quiet)?;

    Ok(())
}
