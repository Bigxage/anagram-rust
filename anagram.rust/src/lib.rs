use std::collections::HashMap;

/// Normalize a word by:
/// - converting it to lowercase
/// - counting each character
fn normalized_counts(word: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for ch in word.to_lowercase().chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }
    counts
}

/// Main function to find anagrams.
///
/// Takes:
/// - `word`: the target word
/// - `candidates`: a slice of strings with lifetime `'a`
///
/// Returns:
/// - a list of candidates (same lifetime `'a`) that are valid anagrams.
pub fn anagrams_for<'a>(word: &str, candidates: &[&'a str]) -> Vec<&'a str> {
    let target_lower = word.to_lowercase();
    let target_counts = normalized_counts(word);

    candidates
        .iter()
        .copied()
        .filter(|candidate| {
            // Rule: A word is not its own anagram.
            if candidate.to_lowercase() == target_lower {
                return false;
            }

            // Compare character counts
            normalized_counts(candidate) == target_counts
        })
        .collect()
}