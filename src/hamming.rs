pub fn hamming_distance(seq_a: &str, seq_b: &str) -> usize {
    assert_eq!(seq_a.len(), seq_b.len());
    seq_a.chars().zip(seq_b.chars())
        .filter(|(x, y)| x != y)
        .count()
}

#[test]
pub fn test_hamming_distance() {
    assert_eq!(hamming_distance("a", "b"), 1);
    assert_eq!(hamming_distance("a", "a"), 0);
    assert_eq!(hamming_distance("abc", "bbb"), 2);
    assert_eq!(hamming_distance("abb", "bbb"), 1);
    assert_eq!(hamming_distance("abcd", "bbbb"), 3);
}
