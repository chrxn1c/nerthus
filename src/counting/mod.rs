use std::collections::HashMap;

pub(super) fn count_words(string: &str) -> usize {
    string.split_ascii_whitespace().count()
}

pub(super) fn count_characters(source_text: &str, word_count: usize) -> HashMap<char, i64> {
    let mut characters_to_count: HashMap<char, i64> = HashMap::with_capacity(word_count);
    let modified_text: String = source_text
        .chars()
        .filter(|char| char.is_ascii_alphabetic())
        .map(|char| char.to_owned())
        .collect();

    for character in modified_text.chars() {
        characters_to_count
            .entry(character)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    characters_to_count
}
