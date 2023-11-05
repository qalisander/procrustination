// NOTE: https://www.codewars.com/kata/53e57dada0cb0400ba000688/train/rust

fn list_position(word: &str) -> u128 {
    todo!();
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::list_position;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    #[test]
    fn sample_tests() {
        let test_data = [
            (                  "A", 1),
            (               "ABAB", 2),
            (               "AAAB", 1),
            (               "BAAA", 4),
            (               "YMYM", 5),
            (           "QUESTION", 24572),
            (         "BOOKKEEPER", 10743),
            ("IMMUNOELECTROPHORETICALLY", 718393983731145698173),
        ];
        for (word, expected) in test_data {
            assert_eq!(list_position(word),
                       expected,
                       "\nYour result (left) did not match the expected output (right) for the input: \"{word}\"");
        }

    }
}