use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn read_words_from_file(file_path: &str) -> io::Result<Vec<String>> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Read lines from the file
    let mut words = Vec::new();
    for line in reader.lines() {
        let line = line?;
        // Split each line into words and add them to the vector
        for word in line.split_whitespace() {
            words.push(word.to_string());
        }
    }
    Ok(words)
}