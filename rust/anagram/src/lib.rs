use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {

    let mut characters = word.to_lowercase().chars().collect::<Vec<char>>();

    let mut anagrams: HashSet<&str> = HashSet::new();

    for possible_anagram in possible_anagrams.into_iter() {
        let mut possible_anagram_characters = possible_anagram.to_lowercase().chars().collect::<Vec<char>>();

        if possible_anagram.to_lowercase() == word.to_lowercase() {
            // Skip the word itself as this is a trival anagram
            continue;
        }

        // Prefer unstable sorting for efficiency since chars are chars
        possible_anagram_characters.sort_unstable();
        characters.sort_unstable();

        // Anagrams to the same if they are identical after sorting
        if possible_anagram_characters == characters {
            anagrams.insert(possible_anagram);
        }
    }

    anagrams
}
