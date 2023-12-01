use std::fs;

fn main() {
    let input_text = fs::read_to_string("inputs/01.txt").expect("Failed to read input file");

    let answer = challenge(&input_text);

    println!("Day 01, Part 1: {}", answer);
    assert_eq!(answer, 55621);
}

fn challenge(input: &str) -> u32 {
    input
        .split("\n") // split by new line
        .filter(|x| !x.is_empty()) // drop empty lines
        .map(|line| decode_calibration_value(line))
        .sum()
}

fn decode_calibration_value(line: &str) -> u32 {
    // take the first and last number, and put them together
    // i.e "pqr3stu8vwx" -> 38

    // filter out non-numeric chars
    let mut number_chars = line.chars().filter(|char| char.is_numeric());

    let first = number_chars.next().unwrap().to_digit(10).unwrap();

    let last = if let Some(last_number_char) = number_chars.last() {
        last_number_char.to_digit(10).unwrap()
    } else {
        // no last number, so use the first one again
        first
    };

    // treat first as the tens place, and last as the ones place
    return (first * 10) + last;
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
        let example_input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(challenge(example_input), 142);
    }

    #[test]
    fn test_decode_calibration_value() {
        assert_eq!(decode_calibration_value("1abc2"), 12);
        assert_eq!(decode_calibration_value("pqr3stu8vwx"), 38);
        assert_eq!(decode_calibration_value("a1b2c3d4e5f"), 15);
        assert_eq!(decode_calibration_value("treb7uchet"), 77);
    }
}
