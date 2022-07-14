use clap::Parser;
use anyhow::Result;

mod library;
mod trimmer;
mod counter;
mod results;
mod permutes;

use fxread::initialize_reader;
use library::Library;
use trimmer::Trimmer;
use counter::Counter;
use permutes::Permuter;
use results::write_results;


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

    /// Allow One Off Matching
    #[clap(short='d', long)]
    oneoff: bool
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    let library = Library::from_reader(
        initialize_reader(&args.library_path)?
        )?;
    let size = library.size();

    let permuter = match args.oneoff {
        true => Some(Permuter::new(library.keys())),
        false => None
    };

    let results: Vec<Counter> = args.input_paths
        .into_iter()
        .map(|x| initialize_reader(&x).unwrap())
        .map(|x| Trimmer::from_reader(x, args.offset, size))
        .map(|x| Counter::new(x, &library, &permuter))
        .collect();

    write_results(args.output_path, &results, &library)?;
    Ok(())
}
