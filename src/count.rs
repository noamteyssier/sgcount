use anyhow::Result;
use fxread::initialize_reader;
use spinners::{Spinner, Spinners};
use rayon::prelude::*;
use crate::{Permuter, Counter, Library, Trimmer};
use crate::results::write_results;

/// Counts the number of matching sgRNA-Reads for a provided
/// filepath
fn count_sample(
        path: &String,
        name: &String,
        offset: usize,
        library: &Library,
        permuter: &Option<Permuter>,
        quiet: bool) -> Result<Counter> {

    let reader = initialize_reader(path)?;
    let trimmer = Trimmer::from_reader(reader, offset, library.size());
    let spinner = match quiet {
        true => None,
        false => Some(Spinner::with_timer(Spinners::Dots3, format!("Processing: {}", name)))
    };
    let counter = Counter::new(trimmer, &library, &permuter);
    match spinner {
        Some(mut s) => s.stop_and_persist("🗸", format!("Finished: {}", name)),
        None => {}
    };
    Ok(counter)
}

/// Generates Mismatch Library if Necessary
fn generate_permutations(
        library: &Library,
        quiet: bool) -> Permuter {

    let spinner = match quiet {
        true => None,
        false => Some(Spinner::with_timer(Spinners::Dots10, format!("Generating Mismatch Library")))
    };
    let permuter = Permuter::new(library.keys());
    match spinner {
        Some(mut s) => s.stop_and_persist("🗸", format!("Finished Mismatch Library")),
        None => {}
    };
    permuter
}


/// Counts the number of matching sgRNA-reads for all provided filepaths
pub fn count(
    library_path: String,
    input_paths: Vec<String>,
    sample_names: Vec<String>,
    output_path: Option<String>,
    offset: usize,
    mismatch: bool,
    quiet: bool) -> Result<()> {


    // generate library
    let library = Library::from_reader(
        initialize_reader(&library_path)?
        )?;

    // generate permuter if necessary
    let permuter = match mismatch{
        true => Some(generate_permutations(&library, quiet)),
        false => None
    };

    // main counting function
    let results: Result<Vec<Counter>> = input_paths
        .into_par_iter()
        .zip(&sample_names)
        .map(|(path, name)| 
            count_sample(
                &path, 
                &name, 
                offset, 
                &library, 
                &permuter, 
                quiet))
        .collect();

    write_results(output_path, &results?, &library, &sample_names)?;

    Ok(())
}
