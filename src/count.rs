use anyhow::Result;
use fxread::initialize_reader;
use rayon::prelude::*;
use indicatif::ProgressBar;
use std::thread;
use crate::{Permuter, Counter, Library, Offset, GeneMap};
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
    start_progress_bar_ref(pb, format!("Processing: {}", name));
    let counter = Counter::new(reader, library, permuter, offset, library.size());
    finish_progress_bar_ref(pb, format!("Finished: {}", name));

    Ok(counter)
}

/// Generates Mismatch Library if Necessary
fn generate_permutations(
        library: &Library,
        quiet: bool) -> Permuter {

    let pb = if quiet { None } else { Some(initialize_progress_bar()) };

    start_progress_bar(&pb, "Generating Mismatch Library".to_string());
    let permuter = Permuter::new(library.keys());
    finish_progress_bar(&pb, "Finished Mismatch Library".to_string());
    permuter
}


/// Counts the number of matching sgRNA-reads for all provided filepaths
pub fn count(
    library_path: &str,
    input_paths: Vec<String>,
    sample_names: &[String],
    output_path: Option<String>,
    offset: Vec<Offset>,
    exact : bool,
    genemap: &Option<GeneMap>,
    quiet: bool) -> Result<()> {


    // generate library
    let library = Library::from_reader(
        initialize_reader(library_path)?
        )?;

    // validate all library sgRNA aliases exist if genemap provided
    if let Some(g) = genemap {
        if !g.validate_library(&library) { panic!("Missing sgRNAs in gene map") }
    }

    // generate permuter if necessary
    let permuter = if !exact { 
        Some(generate_permutations(&library, quiet)) 
    } else { None };

    // generate multiprogress and individual progress bars
    let (mp, progress_bars) = if quiet { (None, None) } else { initialize_multi_progress(sample_names) };
    
    // start multiprogress if not quiet
    let mp = mp.map(|m| thread::spawn(move || m.join()));

    // main counting function
    let results: Result<Vec<Counter>> = input_paths
        .into_par_iter()
        .zip(sample_names)
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

    write_results(output_path, &results?, &library, sample_names, genemap)?;

    Ok(())
}
