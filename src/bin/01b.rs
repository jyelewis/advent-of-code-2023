use std::fs;

fn main() {
    let input_text = fs::read_to_string("inputs/01.txt").expect("Failed to read input file");

    let answer = challenge(&input_text);

    println!("Day 01, Part 2: {}", answer);
    assert_eq!(answer, 53592);
}

fn challenge(input: &str) -> u32 {
    input
        .split('\n') // split by new line
        .filter(|x| !x.is_empty()) // drop empty lines
        .map(decode_calibration_value) // "decode" the magic number from each line
        .sum()
}

fn decode_calibration_value(line: &str) -> u32 {
    // find first number word moving forwards
    // find  last number word moving backwards
    let mut first_num = None;
    let mut last_num = None;

    // move forward through the string one character at a time until we find the first number
    // abconetwonez
    // ...^__       move cursor forward 3 until we match "one"
    for cursor in 0..line.len() {
        // abconetwonez
        //  bconetwonez
        //   conetwonez
        //    onetwonez <-- match found at start of string "one"
        let str_at_cursor = &line[cursor..];

        first_num = num_from_start_of_str(str_at_cursor);
        if first_num.is_some() {
            break;
        }
    }
    let first_num = first_num.expect("No first number found");

    // move backward through the string one character at a time until we find the last number
    // abconetwonez
    //         ^__.      move cursor backwards 4 until we match "one"
    for cursor in (0..line.len()).rev() {
        //    z
        //   ez
        //  nez
        // onez <-- match found at start of string "one"
        let str_at_cursor = &line[cursor..];

        last_num = num_from_start_of_str(str_at_cursor);
        if last_num.is_some() {
            break;
        }
    }
    let last_num = last_num.expect("No last number found");

    // treat first as the tens place, and last as the ones place
    // i.e. 3 * 10 + 8 = 38
    (first_num * 10) + last_num
}

// finds the first number (either string or digit) in a string
fn num_from_start_of_str(input: &str) -> Option<u32> {
    // "one_asdf" -> 1
    // "asdf_one_asdf -> 1
    // "2qwer" -> 2

    // check if the first character is already a digit (0-9) and return if so
    if let Some(digit) = input.chars().next().unwrap().to_digit(10) {
        return Some(digit);
    }

    // see if we can find a number word at the start of the string
    let number_words = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for (word, number) in number_words.iter() {
        if input.starts_with(word) {
            return Some(*number);
        }
    }

    // couldn't find either :(
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_example_input() {
        let example_input = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
        "
        .trim();

        assert_eq!(challenge(example_input), 281);
    }

    #[test]
    fn test_num_from_start_of_str() {
        assert_eq!(num_from_start_of_str("one"), Some(1));
        assert_eq!(num_from_start_of_str("two"), Some(2));
        assert_eq!(num_from_start_of_str("twone"), Some(2));
        assert_eq!(num_from_start_of_str("three"), Some(3));
        assert_eq!(num_from_start_of_str("threeeeeeee"), Some(3));
        assert_eq!(num_from_start_of_str("1"), Some(1));
        assert_eq!(num_from_start_of_str("2"), Some(2));
        assert_eq!(num_from_start_of_str("3"), Some(3));
        assert_eq!(num_from_start_of_str("123"), Some(1));
        assert_eq!(num_from_start_of_str("asdf4"), None);
        assert_eq!(num_from_start_of_str("asdf"), None);
    }

    #[test]
    fn test_decode_int_calibration_value() {
        assert_eq!(decode_calibration_value("1abc2"), 12);
        assert_eq!(decode_calibration_value("pqr3stu8vwx"), 38);
        assert_eq!(decode_calibration_value("a1b2c3d4e5f"), 15);
        assert_eq!(decode_calibration_value("treb7uchet"), 77);
    }

    #[test]
    fn test_decode_str_calibration_value() {
        // provided tests
        assert_eq!(decode_calibration_value("two1nine"), 29);
        assert_eq!(decode_calibration_value("eightwothree"), 83);
        assert_eq!(decode_calibration_value("abcone2threexyz"), 13);
        assert_eq!(decode_calibration_value("xtwone3four"), 24);
        assert_eq!(decode_calibration_value("4nineeightseven2"), 42);
        assert_eq!(decode_calibration_value("zoneight234"), 14);
        assert_eq!(decode_calibration_value("7pqrstsixteen"), 76);

        // custom tests
        assert_eq!(decode_calibration_value("abconetwone"), 11);
    }
}
