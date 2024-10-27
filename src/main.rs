mod reporting;
fn main() {
    let file_path = "./books/frankenstein";
    let file = std::fs::read_to_string(file_path).expect("file not found");
    let file = file.as_str();

    let word_count = reporting::count_words(file);
    let char_count = reporting::count_characters(file, word_count);

    println!("word count: {word_count}");
    println!("char count: {char_count:?}");
}
