use std::fs;

fn main() {
    let input_text = fs::read_to_string("inputs/01.txt").expect("Failed to read input file");

    let answer2 = sum_of_calibration_values(&input_text);

    println!("Day 1, Part 2: {}", answer2);
    assert_eq!(answer2, 53592);
}

fn sum_of_calibration_values(input: &str) -> u32 {
    input
        .split("\n") // split by new line
        .filter(|x| !x.is_empty()) // drop empty lines
        .map(|line| decode_calibration_value(line)) // split into chars iterator
        .sum()
}

fn first_num_in_str(input: &str) -> Option<u32> {
    if input.starts_with("one") || input.starts_with("1") {
        return Some(1);
    }
    if input.starts_with("two") || input.starts_with("2") {
        return Some(2);
    }
    if input.starts_with("three") || input.starts_with("3") {
        return Some(3);
    }
    if input.starts_with("four") || input.starts_with("4") {
        return Some(4);
    }
    if input.starts_with("five") || input.starts_with("5") {
        return Some(5);
    }
    if input.starts_with("six") || input.starts_with("6") {
        return Some(6);
    }
    if input.starts_with("seven") || input.starts_with("7") {
        return Some(7);
    }
    if input.starts_with("eight") || input.starts_with("8") {
        return Some(8);
    }
    if input.starts_with("nine") || input.starts_with("9") {
        return Some(9);
    }

    return None;
}

fn decode_calibration_value(line: &str) -> u32 {
    // find first number word moving forwards
    let mut first_num = None;
    let mut last_num = None;

    // move forward through the string one character at a time until we find the first number
    for cursor in 0..line.len() {
        let str_at_cursor = &line[cursor..];

        first_num = first_num_in_str(str_at_cursor);
        if first_num.is_some() {
            break;
        }
    }
    let first_num = first_num.expect("No first number found");

    // move backward through the string one character at a time until we find the last number
    for cursor in (0..line.len()).rev() {
        let str_at_cursor = &line[cursor..];

        last_num = first_num_in_str(str_at_cursor);
        if last_num.is_some() {
            break;
        }
    }
    let last_num = last_num.expect("No last number found");

    // treat first as the tens place, and last as the ones place
    return (first_num * 10) + last_num;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_first_num_in_str() {
        assert_eq!(first_num_in_str("one"), Some(1));
        assert_eq!(first_num_in_str("two"), Some(2));
        assert_eq!(first_num_in_str("three"), Some(3));
        assert_eq!(first_num_in_str("1"), Some(1));
        assert_eq!(first_num_in_str("2"), Some(2));
        assert_eq!(first_num_in_str("3"), Some(3));
        assert_eq!(first_num_in_str("asdf"), None);
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
