use crate::progress::{
    finish_progress_bar, finish_progress_bar_ref, initialize_multi_progress,
    initialize_progress_bar, start_progress_bar, start_progress_bar_ref,
};
use crate::results::write_results;
use crate::{Counter, GeneMap, Library, Offset, Permuter};
use anyhow::{bail, Result};
use fxread::initialize_reader;
use indicatif::ProgressBar;
use rayon::prelude::*;

/// Counts the number of matching sgRNA-Reads for a provided
/// filepath
fn count_sample(
    path: &str,
    name: &str,
    offset: Offset,
    library: &Library,
    permuter: &Option<Permuter>,
    position_recursion: bool,
    pb: Option<&ProgressBar>,
) -> Result<Counter> {
    let reader = initialize_reader(path)?;
    start_progress_bar_ref(pb, format!("Processing: {}", name));
    let counter = Counter::new(
        reader,
        library,
        permuter,
        offset,
        library.size(),
        position_recursion,
    );
    finish_progress_bar_ref(
        pb,
        format!(
            "Finished: {}; Fraction mapped: {:.3} [{} / {}]",
            name,
            counter.fraction_mapped(),
            counter.matched_reads(),
            counter.total_reads()
        ),
    );
    Ok(counter)
}

/// Generates Mismatch Library if Necessary
fn generate_permutations(library: &Library, quiet: bool) -> Permuter {
    let pb = if quiet {
        None
    } else {
        Some(initialize_progress_bar())
    };

    start_progress_bar(&pb, "Generating Mismatch Library".to_string());
    let permuter = Permuter::new(library.keys());
    finish_progress_bar(&pb, "Finished Mismatch Library".to_string());
    permuter
}

/// Validates that the library size is not too large with respect to the input sequences
fn validate_library_size(library: &Library, input_paths: &[String]) -> Result<bool> {
    for path in input_paths {
        let mut reader = initialize_reader(path)?;
        let size = reader.next().unwrap().seq().len();
        if library.size() > size {
            return Ok(false);
        }
    }
    Ok(true)
}

/// Counts the number of matching sgRNA-reads for all provided filepaths
pub fn count(
    library_path: &str,
    input_paths: Vec<String>,
    sample_names: &[String],
    output_path: Option<String>,
    offset: Vec<Offset>,
    exact: bool,
    genemap: &Option<GeneMap>,
    position_recursion: bool,
    include_zero: bool,
    quiet: bool,
) -> Result<()> {
    // generate library
    let library = Library::from_reader(initialize_reader(library_path)?)?;

    // validate all library sgRNA aliases exist if genemap provided
    if let Some(g) = genemap {
        assert!(g.validate_library(&library), "Missing sgRNAs in gene map");
    }

    // validate library size
    if !validate_library_size(&library, &input_paths)? {
        bail!("Sequences in reference library are larger than the sequences in input.\n\nConsider reducing the length of your reference sequences (i.e. extracting the variable region of the sgRNA or reducing the length of the adapters.)")
    }

    // generate permuter if necessary
    let permuter = if exact {
        None
    } else {
        Some(generate_permutations(&library, quiet))
    };

    // generate multiprogress and individual progress bars
    let (_mp, progress_bars) = if quiet {
        (None, None)
    } else {
        initialize_multi_progress(sample_names)
    };

    // main counting function
    let results: Result<Vec<Counter>> = input_paths
        .into_par_iter()
        .zip(sample_names)
        .zip(offset)
        .enumerate()
        .map(|(idx, ((path, name), offset))| {
            count_sample(
                &path,
                name,
                offset,
                &library,
                &permuter,
                position_recursion,
                match &progress_bars {
                    Some(pbs) => Some(&pbs[idx]),
                    None => None,
                },
            )
        })
        .collect();

    write_results(
        output_path,
        &results?,
        &library,
        sample_names,
        genemap,
        include_zero,
    )?;

    Ok(())
}
