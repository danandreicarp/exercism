use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&str> = HashSet::new();

    let lc_word = word.to_lowercase();
    for anagram in possible_anagrams.iter() {
        if is_anagram(&lc_word, anagram) {
            anagrams.insert(anagram);
        }
    }

    anagrams
}

fn is_anagram(lc_word: &str, anagram: &str) -> bool {
    let lc_anagram = anagram.to_lowercase();
    if lc_word == lc_anagram {
        false
    } else {
        to_char_vec(lc_word) == to_char_vec(&lc_anagram)
    }
}

fn to_char_vec(word: &str) -> Vec<char> {
    let mut chars = Vec::with_capacity(word.len());
    for c in word.chars() {
        chars.push(c);
    }
    chars.sort_unstable();
    chars
}
