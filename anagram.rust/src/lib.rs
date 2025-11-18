use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, candidates: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let mut word_sorted = word_lower.chars().collect::<Vec<_>>();
    word_sorted.sort_unstable();

    candidates
        .iter()
        .copied()
        .filter(|candidate| {
            let candidate_lower = candidate.to_lowercase();

            // skip identical words
            if candidate_lower == word_lower {
                return false;
            }

            // check if sorted letters match
            let mut candidate_sorted = candidate_lower.chars().collect::<Vec<_>>();
            candidate_sorted.sort_unstable();

            candidate_sorted == word_sorted
        })
        .collect() // <-- collects into HashSet<&str>
}
}
