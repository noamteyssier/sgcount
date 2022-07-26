use anyhow::Result;
use fxread::initialize_reader;
use rayon::prelude::*;
use indicatif::ProgressBar;
use std::thread;
use crate::{Permuter, Counter, Library, Offset};
use crate::results::write_results;
use crate::progress::{finish_progress_bar, finish_progress_bar_ref, initialize_multi_progress, initialize_progress_bar, start_progress_bar, start_progress_bar_ref};

/// Counts the number of matching sgRNA-Reads for a provided
/// filepath
fn count_sample(
        path: &str,
        name: &str,
        offset: Offset,
        library: &Library,
        permuter: &Option<Permuter>,
        pb: Option<&ProgressBar>) -> Result<Counter> {

    let reader = initialize_reader(path)?;
    start_progress_bar_ref(&pb, format!("Processing: {}", name));
    let counter = Counter::new(reader, library, permuter, offset, library.size());
    finish_progress_bar_ref(&pb, format!("Finished: {}", name));

    Ok(counter)
}

/// Generates Mismatch Library if Necessary
fn generate_permutations(
        library: &Library,
        quiet: bool) -> Permuter {

    let pb = match quiet {
        true => None,
        false => Some(initialize_progress_bar())
    };

    start_progress_bar(&pb, "Generating Mismatch Library".to_string());
    let permuter = Permuter::new(library.keys());
    finish_progress_bar(&pb, "Finished Mismatch Library".to_string());
    permuter
}


/// Counts the number of matching sgRNA-reads for all provided filepaths
pub fn count(
    library_path: String,
    input_paths: Vec<String>,
    sample_names: Vec<String>,
    output_path: Option<String>,
    offset: Vec<Offset>,
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

    // generate multiprogress and individual progress bars
    let (mp, progress_bars) = match quiet {
        true => (None, None),
        false => initialize_multi_progress(&sample_names)
    };
    
    // start multiprogress if not quiet
    let mp = mp.map(|m| thread::spawn(move || m.join()));

    // main counting function
    let results: Result<Vec<Counter>> = input_paths
        .into_par_iter()
        .zip(&sample_names)
        .zip(offset)
        .enumerate()
        .map(|(idx, ((path, name), offset))| 
            count_sample(
                &path, 
                name, 
                offset, 
                &library, 
                &permuter, 
                match &progress_bars {
                    Some(pbs) => Some(&pbs[idx]),
                    None => None
                }))
        .collect();

    // join multiprogress if not quiet
    if let Some(m) = mp { m.join().unwrap()? };

    write_results(output_path, &results?, &library, &sample_names)?;

    Ok(())
}
