use anyhow::Result;
use hashbrown::HashSet;

/// Sets the number of threads globally
pub fn set_threads(threads: usize) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .unwrap();
}

/// Converts a vector of bytes to a string
pub fn vec_to_nuc(vec: &[u8]) -> Result<String> {
    Ok(std::str::from_utf8(vec)?.to_string())
}

/// Generates default sample names
pub fn generate_sample_names(input_paths: &[String]) -> Vec<String> {
    // calculate basenames of input files
    let base_names = input_paths
        .iter()
        .map(|x| x.split('/').last().unwrap())
        .map(|x| x.trim_end_matches(".gz"))
        .map(|x| x.trim_end_matches(".fasta"))
        .map(|x| x.trim_end_matches(".fastq"))
        .map(|x| x.trim_end_matches(".fa"))
        .map(|x| x.trim_end_matches(".fq"))
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let simple_names = input_paths
        .iter()
        .enumerate()
        .map(|(idx, _)| format!("Sample.{:?}", idx))
        .collect::<Vec<String>>();

    // check if there are duplicate values
    let mut seen = HashSet::new();
    for name in base_names.iter() {
        seen.insert(name.clone());
    }

    if seen.len() == base_names.len() {
        base_names
    } else {
        eprintln!("WARNING: Duplicate Basenames Detected, Using incrementing sample names");
        simple_names
    }
}

#[cfg(test)]
mod testing {

    #[test]
    fn test_sample_names() {
        let paths = [
            "example/some_name_1.fastq.gz",
            "example/some_name_2.fastq",
            "example/some_name_3.fasta.gz",
            "example/some_name_4.fasta",
            "example/some_name_5.fq.gz",
            "example/some_name_6.fq",
            "example/some_name_7.fa.gz",
            "example/some_name_8.fa",
        ]
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
        let names = super::generate_sample_names(&paths);
        let expected = vec![
            "some_name_1",
            "some_name_2",
            "some_name_3",
            "some_name_4",
            "some_name_5",
            "some_name_6",
            "some_name_7",
            "some_name_8",
        ];
        assert_eq!(names, expected);
    }

    #[test]
    fn test_sample_names_duplicates() {
        let paths = [
            "example/some_name_1.fastq.gz",
            "example/some_name_1.fastq",
            "example/some_name_3.fasta.gz",
            "example/some_name_4.fasta",
            "example/some_name_5.fq.gz",
            "example/some_name_6.fq",
            "example/some_name_7.fa.gz",
            "example/some_name_8.fa",
        ]
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
        let names = super::generate_sample_names(&paths);
        let expected = vec![
            "Sample.0", "Sample.1", "Sample.2", "Sample.3", "Sample.4", "Sample.5", "Sample.6",
            "Sample.7",
        ];
        assert_eq!(names, expected);
    }
}
