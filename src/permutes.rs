use std::collections::{HashSet, HashMap};

const LEXICON: [char; 5] = ['A', 'C', 'G', 'T', 'N'];

/// Calculates all unambiguous 'one-off' permutations 
/// for a set of sequences
pub struct Permuter {
    map: HashMap<String, String>,
    _null: HashSet<String>
}

impl Permuter {
    pub fn new<'a>(sequences: impl Iterator<Item = &'a String>) -> Self {
        let (map, null) = Self::build(sequences);
        Self { map, _null: null }
    }

    pub fn contains(&self, token: &str) -> Option<&String> {
        self.map.get(token)
    }

    fn build<'a>(
            sequences: impl Iterator<Item = &'a String>) -> (HashMap<String, String>, HashSet<String>) 
    {
        sequences
            .map(|seq| (seq, Self::permute_sequence(seq, &LEXICON)))
            .fold(
                (HashMap::new(), HashSet::new()), 
                |(mut table, mut null), (seq, permute)| {
                    permute
                        .iter()
                        .for_each(|x| Self::insert_sequence(seq, x, &mut null, &mut table));
                    (table, null)
                    })
    }

    fn permute_sequence(
            sequence: &str, 
            lexicon: &[char; 5]) -> Vec<String> 
    {
        sequence
            .char_indices()
            .map(|(idx, _)| Self::sequence_regions(sequence, idx))
            .map(|(p,x,s)| Self::build_permutations(p, s, x, lexicon))
            .flatten()
            .collect()
    }

    fn sequence_regions(
            sequence: &str, 
            idx: usize) -> (&str, &str, &str) 
    {
        let (prefix, poschar) = sequence.split_at(idx);
        let (_, suffix) = sequence.split_at(idx + 1);
        (prefix, poschar, suffix)
    }

    fn build_permutations(
            prefix: &str, 
            suffix: &str, 
            poschar: &str, 
            lexicon: &[char; 5]) -> Vec<String> 
    {
        lexicon
            .iter()
            .filter(move |y| *y != &Self::aschar(poschar))
            .map(|y| Self::build_permutation(prefix, suffix, y))
            .collect()      
    }

    fn aschar(poschar: &str) -> char {
        poschar.chars().next().unwrap()
    }

    fn build_permutation(
            prefix: &str, 
            suffix: &str, 
            insertion: &char) -> String 
    {
        let mut sequence = String::new();
        sequence.push_str(prefix);
        sequence.push(*insertion);
        sequence.push_str(suffix);
        sequence
    }

    fn insert_sequence(
            sequence: &str, 
            permutation: &String, 
            null: &mut HashSet<String>, 
            table: &mut HashMap<String, String>) 
    {
        if !null.contains(sequence) {
            null.insert(sequence.to_string());
        }

        if !null.contains(permutation) {
            match table.contains_key(permutation) {
                true => Self::insert_to_null(permutation, null, table),
                false => Self::insert_to_table(permutation, sequence, table)
            }
        }
    }

    fn insert_to_null(
            permutation: &String, 
            null: &mut HashSet<String>, 
            table: &mut HashMap<String, String>) 
    {
        table.remove(permutation);
        null.insert(permutation.to_owned());
    }

    fn insert_to_table(
            permutation: &String, 
            sequence: &str, 
            table: &mut HashMap<String, String>) 
    {
        table.insert(permutation.to_owned(), sequence.to_string());
    }
}


#[cfg(test)] 
mod test {

    /// # Example Test
    /// This will create all unambiguous one-off sequences
    /// for a list of sequences.
    ///
    /// Imagine the case of the following two sequences.
    ///
    ///  AC   CG
    ///-----------
    /// ~CC   CA
    ///  GC  ~CC
    ///  TC   CT
    ///  NC   CN
    ///  AA  ~AG
    /// ~AG   GG
    ///  AT   TG
    ///  AN   NG
    ///
    ///  All `~` demarcated are ambiguous between the two and should
    ///  be present in the null, alongside the origin sequence `AC` 
    ///  and `CG`

    use super::Permuter;

    #[test]
    fn build() {
        let sequences = vec!["AC".to_string(), "CG".to_string()];
        Permuter::new(sequences.iter());
    }

    #[test]
    fn validate_singleton() {
        let sequences = vec!["ACTG".to_string()];
        let permuter = Permuter::new(sequences.iter());
        let truth = vec![
            "AATG","ACGG","ACAG","TCTG",
            "ACNG","NCTG","ACTA","GCTG",
            "AGTG","ACTC","ATTG","ANTG",
            "ACCG","ACTT","CCTG","ACTN"
        ];
        assert!(truth.iter().all(|x| permuter.map.contains_key(*x)));
        assert!(truth.iter().all(|x| !permuter._null.contains(*x)));
        assert!(permuter._null.contains("ACTG"));
        assert_eq!(permuter._null.len(), 1);
    }

    #[test]
    fn validate_positive() {
        let sequences = vec!["AC".to_string(), "CG".to_string()];
        let permuter = Permuter::new(sequences.iter());

        let known_positives = vec!["GC", "TC", "NC", "AA", "AT", "AN", "CA", "CT", "CN", "GG", "TG", "NG"];

        // validate known positives in map
        known_positives
            .iter()
            .for_each(|x| assert!(permuter.map.contains_key(*x)));
        assert_eq!(permuter.map.len(), 12);

        // validate known negatives not in map
        known_positives 
            .iter()
            .for_each(|x| assert!(!permuter._null.contains(*x)));
    }

    #[test]
    fn validate_negative() {
        let sequences = vec!["AC".to_string(), "CG".to_string()];
        let permuter = Permuter::new(sequences.iter());

        let known_negatives = vec!["AG", "CG", "CC", "AG"];

        // validate known negatives in._null
        known_negatives
            .iter()
            .for_each(|x| assert!(permuter._null.contains(*x)));
        assert_eq!(permuter._null.len(), 4);

        // validate known positives not in._null
        known_negatives
            .iter()
            .for_each(|x| assert!(!permuter.map.contains_key(*x)));
    }

}
