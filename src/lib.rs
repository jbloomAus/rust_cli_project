
//! This module contains the logic for finding matches in a string
//! and writing them to a writer.


use log::error;

/// this function finds all the matches of a pattern in a string
pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            match writeln!(writer, "{}", line) {
                Ok(_) => (),
                Err(e) => return error!("Error writing to stdout: {}", e),
            }
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
