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

use anyhow::Result;
use clap::Parser;

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

/// Module for Mapping `sgRNAs` to their Parent Genes
pub mod genemap;

/// Module for utility functions regarding progress spinners
pub mod progress;

pub use count::count;
pub use counter::Counter;
pub use fxread::initialize_reader;
pub use genemap::GeneMap;
pub use library::Library;
use offsetter::entropy_offset_group;
pub use offsetter::{entropy_offset, Offset};
pub use permutes::Permuter;
use progress::{finish_progress_bar, initialize_progress_bar, start_progress_bar};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Filepath of the library
    #[clap(short, long, value_parser)]
    library_path: String,

    /// Filepath(s) of fastx (fastq, fasta, *.gz) sequences to map
    #[clap(short, long, value_parser, min_values = 1, required = true)]
    input_paths: Vec<String>,

    /// Sample Names
    #[clap(short = 'n', long, value_parser, min_values = 1, required = false)]
    sample_names: Option<Vec<String>>,

    /// Output filepath [default: stdout]
    #[clap(short, long, value_parser)]
    output_path: Option<String>,

    /// Gene to sgRNA mapping
    #[clap(short, long, value_parser)]
    genemap: Option<String>,

    /// Adapter Offset
    #[clap(short = 'a', long, value_parser)]
    offset: Option<usize>,

    /// Remove Position Recursion (i.e. offseting sequences by +/- 1 on mismatch condition)
    #[clap(short = 'p', long)]
    no_position_recursion: bool,

    /// Read Direction (reverse complement reads)
    #[clap(short = 'r', long)]
    reverse: bool,

    /// Disallow One Off Mismatch
    #[clap(short = 'x', long)]
    exact: bool,

    /// Number of Reads to Subsample in Determining Offset [default: 5000]
    #[clap(short = 's', long)]
    subsample: Option<usize>,

    /// Number of Threads to Use for Parallel Jobs
    #[clap(short = 't', long, default_value = "1")]
    threads: usize,

    /// Does not show progress
    #[clap(short = 'q', long)]
    quiet: bool,
}

/// Sets the number of threads globally
fn set_threads(threads: usize) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .unwrap();
}

/// Generates default sample names
fn generate_sample_names(input_paths: &[String]) -> Vec<String> {
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
    quiet: bool,
) -> Result<Vec<Offset>> {
    let subsample = subsample.unwrap_or(5000);
    let pb = if quiet {
        None
    } else {
        Some(initialize_progress_bar())
    };
    start_progress_bar(&pb, "Calculating Offset".to_string());
    let offset = entropy_offset_group(library_path, input_paths, subsample)?;
    finish_progress_bar(&pb, format!("Calculated Offsets: {:?}", offset));
    Ok(offset)
}

/// Validate Paths Exist
fn validate_paths(input_paths: &[String]) {
    for x in input_paths.iter() {
        if !Path::new(x).exists() {
            assert!(
                Path::new(x).exists(),
                "Provided filepath does not exist: {}",
                x
            );
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    set_threads(args.threads);

    // validates all input paths
    validate_paths(&args.input_paths);

    // generates sample names if required
    let sample_names = match args.sample_names {
        Some(s) => {
            if s.len() == args.input_paths.len() {
                s
            } else {
                panic!("Must provide as many sample names as there are input files")
            }
        }
        None => generate_sample_names(&args.input_paths),
    };

    // calculates offset if required
    let offset = match args.offset {
        Some(o) => {
            if args.reverse {
                vec![Offset::Reverse(o); args.input_paths.len()]
            } else {
                vec![Offset::Forward(o); args.input_paths.len()]
            }
        }
        None => calculate_offset(
            &args.library_path,
            &args.input_paths,
            args.subsample,
            args.quiet,
        )?,
    };

    // builds gene map is provided
    let genemap = match args.genemap {
        Some(g) => Some(GeneMap::new(&g)?),
        None => None,
    };

    // default position recursion is true; flag flips this bool
    let position_recursion = !args.no_position_recursion;

    // perform counting
    count(
        &args.library_path,
        args.input_paths,
        &sample_names,
        args.output_path,
        offset,
        args.exact,
        &genemap,
        position_recursion,
        args.quiet,
    )?;

    Ok(())
}
