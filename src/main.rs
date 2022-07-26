//! sgcount
//!
//! # Summary
//! This is a commandline tool to count the frequency of `sgRNAs`
//! in a group of provided sequencing files. It is meant to replace
//! methods of exact sequence matching without sacrificing speed and
//! replace costly alignment scripts using bwa or bowtie to align to
//! a library.

#![warn(missing_docs)]
use std::path::Path;

use clap::Parser;
use anyhow::Result;

/// Module for Sequence Library
pub mod library;

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

/// Module for utility functions regarding progress spinners
pub mod progress;

pub use fxread::initialize_reader;
pub use library::Library;
pub use counter::Counter;
use offsetter::entropy_offset_group;
pub use permutes::Permuter;
pub use offsetter::{Offset, entropy_offset};
pub use count::count;
use progress::{finish_progress_bar, initialize_progress_bar, start_progress_bar};


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

    /// Read Direction (reverse complement reads)
    #[clap(short='r', long)]
    reverse: bool,

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
        input_paths: &[String]) -> Vec<String> {
    input_paths
        .iter()
        .enumerate()
        .map(|(idx, _)| format!("Sample.{:?}", idx))
        .collect()
}

/// Calculates Offset if Required
fn calculate_offset(
        library_path: &str,
        input_paths: &[String],
        subsample: Option<usize>,
        quiet: bool) -> Result<Vec<Offset>> { 

    let subsample = subsample.unwrap_or(5000);
    let pb = match quiet {
        true => None,
        false => Some(initialize_progress_bar())
    };
    start_progress_bar(&pb, "Calculating Offset".to_string());
    let offset = entropy_offset_group(library_path, input_paths, subsample)?;
    finish_progress_bar(&pb, format!("Calculated Offsets: {:?}", offset));
    Ok(offset)
}

/// Validate Paths Exist
fn validate_paths(input_paths: &[String]) {
    input_paths
        .iter()
        .for_each(|x| {
            if !Path::new(x).exists() { panic!("Provided filepath does not exist: {}", x) }
        })
}


fn main() -> Result<()> {
    let args = Args::parse();

    set_threads(args.threads);

    // validates all input paths
    validate_paths(&args.input_paths);

    // generates sample names if required
    let sample_names = match args.sample_names {
        Some(s) => if s.len() == args.input_paths.len() { s } else { panic!("Must provide as many sample names as there are input files") },
        None => generate_sample_names(&args.input_paths)
    };

    // calculates offset if required
    let offset = match args.offset {
        Some(o) => if args.reverse { vec![Offset::Reverse(o); args.input_paths.len()] } else { vec![Offset::Forward(o); args.input_paths.len()] },
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
