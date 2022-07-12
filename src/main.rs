use anyhow::Result;
use clap::{App, Arg, ArgMatches};

mod library;
mod trimmer;
mod counter;
mod results;

use fxread::initialize_reader;
use library::Library;
use trimmer::Trimmer;
use counter::Counter;
use results::write_results;


fn get_args() -> ArgMatches {
    App::new("sgcount")
        .version("0.1.0")
        .author("Noam Teyssier")
        .about("Maps sgRNA counts for multiple fastq files")
        .arg(Arg::with_name("library")
            .short('l')
            .long("library")
            .value_name("LIBRARY")
            .help("Sets a library fasta file to match against")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("inputs")
            .help("Input fastq file[s] to process")
            .short('i')
            .long("input")
            .value_name("INPUTS")
            .min_values(1)
            .required(true))
        .arg(Arg::with_name("output")
            .help("output file path to write tab-delim to")
            .short('o')
            .long("output")
            .value_name("OUTPUT")
            .takes_value(true)
            .required(false))
        .arg(Arg::with_name("offset")
            .help("sequence offset from prefix to begin matching")
            .short('n')
            .long("offset")
            .value_name("OFFSET")
            .takes_value(true)
            .required(false)
            .default_value("0"))
        .get_matches()
}

fn main() -> Result<()> {
    let matches = get_args();
    let lib_path = matches.value_of("library").unwrap();
    let input_paths: Vec<_> = matches.values_of("inputs").unwrap().collect();
    let output_path = matches.value_of("output").unwrap_or("");
    let offset = matches.value_of("offset").unwrap().parse::<usize>().unwrap();

    let library = Library::from_reader(
        initialize_reader(lib_path)?
        )?;
    let size = library.size();

    let results: Vec<Counter> = input_paths
        .into_iter()
        .map(|x| initialize_reader(x).unwrap())
        .map(|x| Trimmer::from_reader(x, offset, size))
        .map(|x| Counter::new(x, &library))
        .collect();

    write_results(output_path, &results, &library)?;
    Ok(())
}
