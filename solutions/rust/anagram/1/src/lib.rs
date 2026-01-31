use std::collections::HashSet;

// Find all anagrams of a given word from a list of possible anagrams
// An anagram is a word formed by rearranging the letters of another word
// The function is case-insensitive and does not consider the original word as its own anagram
// Returns a set of anagrams found in the list
// life-time annotations ensure that the returned references are valid as long as the input slice is valid
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&'a str]) -> HashSet<&'a str> {
    // todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    // Normalize the input word by converting it to lowercase and sorting its characters
    let word_lower = word.to_lowercase();
    // Use a HashSet to store unique anagrams
    // This avoids duplicates in the result
    let mut result = HashSet::new();
    // Sort the characters of the input word for comparison
    // This allows us to easily check for anagrams by comparing sorted character arrays
    let mut word_chars: Vec<char> = word_lower.chars().collect();

    // Sort the characters to prepare for anagram comparison
    // Using unstable sort for performance, as we don't need a stable sort here
    word_chars.sort_unstable();

    // Iterate through each candidate word to check if it's an anagram of the input word 
    // We skip the candidate if it's identical to the input word (case-insensitive)
    for &candidate in possible_anagrams {

        // Convert candidate to lowercase for case-insensitive comparison
        let candidate_lower = candidate.to_lowercase();

        // Check if the candidate is an anagram by comparing sorted character arrays
        if candidate_lower != word_lower {
            let mut candidate_chars: Vec<char> = candidate_lower.chars().collect();
            candidate_chars.sort_unstable();
            if candidate_chars == word_chars {
                result.insert(candidate);
            }
        }
    }
    result
}
