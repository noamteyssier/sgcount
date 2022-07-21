use fxread::{Record, initialize_reader};
use ndarray::{Axis, Array2, Array1, ArrayBase, ViewRepr, Dim};
use anyhow::Result;
use ndarray_stats::{EntropyExt, DeviationExt, QuantileExt};

/// An enumeration describing whether the sequences
/// are offset in a forward direction or if the offset
/// is better described from the reverse complement
#[derive(Debug, Copy, Clone)]
pub enum Offset {
    /// Reads will be processed with an offset in the forward direction
    Forward(usize),
    /// Reads will be processed with an offset on the reverse complement
    Reverse(usize)
}
impl Offset {
    /// Returns the internal index of the offset
    pub fn index(&self) -> &usize {
        match self {
            Self::Forward(index) => index,
            Self::Reverse(index) => index,
        }
    }
}

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

/// Convenience function for returning a subset of an array
fn slice_array(
        array: &Array1<f64>, 
        x: usize, 
        y: usize) -> ArrayBase<ViewRepr<&f64>, Dim<[usize; 1]>> 
{
    array.slice(ndarray::s![x..y])
}

/// Calculate Windowed MSE of two arrays
/// where the first array is smaller than
/// the second
fn windowed_mse(
        array1: &Array1<f64>,
        array2: &Array1<f64>) -> Array1<f64>
{
    let size = array2.len() - array1.len() + 1;
    (0..size)
        .map(|idx| (idx, idx+array1.len()))
        .fold(
            Array1::<f64>::zeros(size), 
            |mut arr, (x, y)| {
                arr[x] += array1.mean_sq_err(&slice_array(&array2, x, y)).expect("unexpected error");
                arr
            })
}

fn assign_offset(
        mse_forward: Array1<f64>, 
        mse_reverse: Array1<f64>) -> Offset
{
    let argmin_forward = match mse_forward.argmin() {
        Ok(m) => m,
        Err(why) => panic!("Unexpected minmax error in entropy: {}", why)
    };

    let argmin_reverse = match mse_reverse.argmin() {
        Ok(m) => m,
        Err(why) => panic!("Unexpected minmax error in entropy: {}", why)
    };

    let min_forward = match mse_forward.min() {
        Ok(m) => m,
        Err(why) => panic!("Unexpected minmax error in entropy: {}", why)
    };

    let min_reverse = match mse_reverse.min() {
        Ok(m) => m,
        Err(why) => panic!("Unexpected minmax error in entropy: {}", why)
    };

    match min_forward < min_reverse {
        
        // Reads are in forward directionality
        true => Offset::Forward(argmin_forward),

        // Reads are in reverse directionality
        false => Offset::Reverse(argmin_reverse)
    }
}

/// Calculates the starting position which minimizes the entropy between two entropy arrays
fn minimize_mse(reference: &Array1<f64>, comparison: &Array1<f64>) -> Offset {
    let size = comparison.len() - reference.len() + 1;
    assert!(size > 0);
    let rev_comparison = comparison.iter().rev().map(|x| x.clone()).collect();

    let mse_forward = windowed_mse(&reference, &comparison);
    let mse_reverse = windowed_mse(&reference, &rev_comparison);

    assign_offset(mse_forward, mse_reverse)
}

/// Calculates the Offset in the Comparison by Minimizing
/// the MSE of Positional Entropy Observed in the Reference.
pub fn entropy_offset(
        library_path: &String,
        input_paths: &Vec<String>,
        subsample: usize) -> Result<Offset> {

    let mut reference = initialize_reader(&library_path)?;
    let mut comparison = initialize_reader(&input_paths[0])?.take(subsample);

    let reference_entropy = positional_entropy(&mut reference);
    let comparison_entropy = positional_entropy(&mut comparison);

    let index = minimize_mse(&reference_entropy, &comparison_entropy);
    Ok(index)
}

/// Calculates the Offset in the Comparison by Minimizing
/// the MSE of Positional Entropy Observed in the Reference
/// For Each Provided Path
pub fn entropy_offset_group(
        library_path: &String,
        input_paths: &Vec<String>,
        subsample: usize) -> Result<Vec<Offset>>
{
    let mut reference = initialize_reader(&library_path)?;
    let reference_entropy = positional_entropy(&mut reference);
    let result: Vec<Offset> = input_paths
        .iter()
        .map(|x| 
            initialize_reader(x)
                .expect(&format!("Unable to open file: {}", x))
                .take(subsample))
        .map(|mut x| positional_entropy(&mut x))
        .map(|x| minimize_mse(&reference_entropy, &x))
        .collect();
    Ok(result)
}

#[cfg(test)]
mod test {
    use ndarray::Array1;
    use super::{minimize_mse, get_sequence_size, position_counts, normalize_counts, positional_entropy, Offset};
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

    // create reader with an `AACAA` static prefix on before the `AC` static prefix
    // but reverse complemented
    fn rc_offset_reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] = b">seq.0\nAGTTTGTT\n>seq.1\nGGTTTGTT\n>seq.2\nAGTTTGTT\n";
        Box::new(FastaReader::new(sequence))
    }

    #[test]
    fn minimization() {
        let arr = Array1::linspace(0., 10., 11);
        let brr = Array1::linspace(10., 20., 100);
        match minimize_mse(&arr, &brr) {
            Offset::Forward(x) => assert_eq!(x, 0),
            Offset::Reverse(_) => assert!(false)
        }
    }

    #[test]
    #[should_panic]
    fn undersized_minimization() {
        let arr = Array1::linspace(0., 10., 11);
        let brr = Array1::linspace(10., 20., 5);
        minimize_mse(&arr, &brr);
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
        let index = match minimize_mse(&reference_entropy, &comparison_entropy) {
            Offset::Forward(x) => x,
            Offset::Reverse(_) => panic!("Unexpected reverse")
        };

        assert_eq!(index, 5);
    }

    #[test]
    fn rc_offset() {
        let mut reference = reader();
        let mut comparison = rc_offset_reader();
        let reference_entropy = positional_entropy(&mut reference);
        let comparison_entropy = positional_entropy(&mut comparison);
        let index = match minimize_mse(&reference_entropy, &comparison_entropy) {
            Offset::Forward(_) => panic!("Unexpected forward"),
            Offset::Reverse(x) => x
        };
        assert_eq!(index, 5);
    }
}
