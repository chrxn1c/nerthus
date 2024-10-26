fn main() {
    let file_path = "./books/frankenstein";
    let file = std::fs::read_to_string(file_path).expect("file not found");
    println!("contents:\n{file}");
}
