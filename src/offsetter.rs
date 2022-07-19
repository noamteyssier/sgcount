use fxread::{Record, initialize_reader};
use ndarray::{Axis, Array2, Array1};
use anyhow::Result;
use ndarray_stats::{EntropyExt, DeviationExt, QuantileExt};

/// Calculates the size of the first sequence in a [`fxread::FastxRead`] Iterator.
fn get_sequence_size(reader: &mut dyn Iterator<Item = Record>) -> usize {
    reader
        .next()
        .expect("empty reader")
        .seq()
        .len()
}

/// Assigns a stable index to each nucleotide
fn base_map(c: char) -> Option<usize> {
    match c {
        'A' => Some(0),
        'C' => Some(1),
        'G' => Some(2),
        'T' => Some(3),
        _ => None
    }
}

/// Creates a 2D matrix of shape (seq_size, 4) where each row represents the positional
/// index of the sequence and each column represents the number of observed nucleotides 
/// at that position
fn position_counts(reader: &mut dyn Iterator<Item = Record>) -> Array2<f64>{
    // skips the first record to calculate size
    let size = get_sequence_size(reader);
    reader
        .fold(
            Array2::<f64>::zeros((size, 4)),
            |mut posmat, record| {
                record
                    .seq()
                    .char_indices()
                    .map(|(idx, c)| (idx, base_map(c)))
                    .for_each(|(idx, jdx)| {
                        match jdx {
                            
                            // increment the nucleotide index and at the position
                            Some(j) => {
                                posmat[[idx, j]] += 1.
                            },

                            // increment each nucleotide index if an `N` is found (as it could be
                            // anything)
                            None => {
                                posmat[[idx, 0]] += 1.;
                                posmat[[idx, 1]] += 1.;
                                posmat[[idx, 2]] += 1.;
                                posmat[[idx, 3]] += 1.;
                            }
                        };
                    });
                posmat
            })
}

/// Normalizes the nucleotide counts across each row (i.e. sequence positional index)
fn normalize_counts(matrix: Array2<f64>) -> Array2<f64> {
    let (x, y) = matrix.dim();
    let sums = matrix.sum_axis(Axis(1));
    let norm = matrix / sums.broadcast((y, x)).expect("incompatible sizes").t();
    norm
}

/// Calculates the nucleotide entropy for each basepair position in an [`fxread::FastxRead`] iterator.
fn positional_entropy(reader: &mut dyn Iterator<Item = Record>) -> Array1<f64> {
    let pos_prob = normalize_counts(position_counts(reader));
    pos_prob
        .map_axis(
            Axis(1), 
            |axis| axis.entropy().expect("Unexpected Negatives in Calculation"))
}

/// Calculates the starting position which minimizes the entropy between two entropy arrays
fn minimize_mse(reference: Array1<f64>, comparison: Array1<f64>) -> usize {
    let size = comparison.len() - reference.len() + 1;
    assert!(size > 0);
    let mse = (0..size)
        .map(|idx| (idx, idx+reference.len()))
        .fold(
            Array1::<f64>::zeros(size), 
            |mut arr, (x, y)| {
                arr[x] += reference.mean_sq_err(&comparison.slice(ndarray::s![x..y])).expect("unexpected error");
                arr
            });
    mse.argmin().expect("MinMax Error")
}

/// Calculates the Offset in the Comparison by Minimizing
/// the MSE of Positional Entropy Observed in the Reference.
pub fn entropy_offset(
        library_path: &String,
        input_paths: &Vec<String>,
        subsample: usize) -> Result<usize> {

    let mut reference = initialize_reader(&library_path)?;
    let mut comparison = initialize_reader(&input_paths[0])?.take(subsample);

    let reference_entropy = positional_entropy(&mut reference);
    let comparison_entropy = positional_entropy(&mut comparison);

    let index = minimize_mse(reference_entropy, comparison_entropy);
    Ok(index)
}

#[cfg(test)]
mod test {
    use ndarray::Array1;
    use super::{minimize_mse, get_sequence_size, position_counts, normalize_counts, positional_entropy};
    use fxread::{FastaReader, FastxRead, Record};

    // create reader with an `AC` static prefix
    fn reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] = b">seq.0\nACT\n>seq.1\nACC\n>seq.2\nACT\n";
        Box::new(FastaReader::new(sequence))
    }


    // create reader with an `AACAA` static prefix on before the `AC` static prefix
    fn offset_reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] = b">seq.0\nAACAAACT\n>seq.1\nAACAAACC\n>seq.2\nAACAAACT\n";
        Box::new(FastaReader::new(sequence))
    }

    #[test]
    fn minimization() {
        let arr = Array1::linspace(0., 10., 11);
        let brr = Array1::linspace(10., 20., 100);
        assert_eq!(minimize_mse(arr, brr), 0);
    }

    #[test]
    #[should_panic]
    fn undersized_minimization() {
        let arr = Array1::linspace(0., 10., 11);
        let brr = Array1::linspace(10., 20., 5);
        minimize_mse(arr, brr);
    }

    #[test]
    fn sequence_size() {
        let mut reader = reader();
        let size = get_sequence_size(&mut reader);
        assert_eq!(size, 3);
        assert_eq!(reader.count(), 2);
    }

    #[test]
    fn positional_counts() {
        let posmat = position_counts(&mut reader());
        let expected = ndarray::array![
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 1.0]];
        let diff = posmat - expected;
        assert_eq!(diff.sum(), 0.);
    }

    #[test]
    fn normalize() {
        let test = ndarray::array![
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 1.0]];
        let norm = normalize_counts(test);
        let expected = ndarray::array![
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.5, 0.0, 0.5]];
        let diff = norm - expected;
        assert_eq!(diff.sum(), 0.);
    }

    #[test]
    fn offset() {
        let mut reference = reader();
        let mut comparison = offset_reader();

        let reference_entropy = positional_entropy(&mut reference);
        let comparison_entropy = positional_entropy(&mut comparison);
        let index = minimize_mse(reference_entropy, comparison_entropy);

        assert_eq!(index, 5);
    }
}
