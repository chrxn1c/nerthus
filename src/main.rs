mod counting;
mod reporting;

fn main() {
    let file_path = "./books/frankenstein";
    let file = std::fs::read_to_string(file_path).expect("file not found");
    let file = file.as_str();

    let word_count = counting::count_words(file);
    let char_count = counting::count_characters(file, word_count);

    let word_report = reporting::get_word_report(word_count);
    let char_report = reporting::get_chars_report(char_count);

    println!("{word_report}\n\n{char_report}");
}
