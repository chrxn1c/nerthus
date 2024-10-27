use std::collections::HashMap;

pub(super) fn count_words(string: &str) -> usize {
    string.split(" ").count()
}

pub(super) fn count_characters(string: &str, word_count: usize) -> HashMap<char, i64> {
    let mut characters_to_count: HashMap<char, i64> = HashMap::with_capacity(word_count);
    for character in string.chars() {
        characters_to_count
            .entry(character)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    characters_to_count
}
