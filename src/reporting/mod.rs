use std::collections::HashMap;

pub(super) fn get_word_report(word_count: usize) -> String {
    std::format!("{word_count} words found in the document.")
}

pub(super) fn get_chars_report(chars: HashMap<char, i64>) -> String {
    chars
        .into_iter()
        .map(|entry| std::format!("The {} character was met {} times.\n", entry.0, entry.1))
        .collect()
}
