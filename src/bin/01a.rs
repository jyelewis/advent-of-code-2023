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
        .map(|line| decode_calibration_value(line)) // "decode" the magic number from each line
        .sum()
}

fn decode_calibration_value(line: &str) -> u32 {
    // take the first and last number, and put them together
    // i.e "pqr3st1u8vwx" -> 38

    let number_chars: Vec<u32> = line
        .chars() // split into char iterator
        .filter(|char| char.is_numeric()) // filter out non-numeric chars
        .map(|digit_char| digit_char.to_digit(10).unwrap()) // parse digit chars '1' -> 1
        .collect();

    // i.e left with [3, 1, 8]. First & last are 3 & 8
    let first = number_chars.first().unwrap();
    let last = number_chars.last().unwrap();

    // treat first as the tens place, and last as the ones place
    // i.e. 3 * 10 + 8 = 38
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
        let example_input = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
        "
        .trim();

        assert_eq!(challenge(example_input), 142);
    }

    #[test]
    fn test_decode_calibration_value() {
        assert_eq!(decode_calibration_value("1abc2"), 12);
        assert_eq!(decode_calibration_value("pqr3stu8vwx"), 38);
        assert_eq!(decode_calibration_value("pqr3st1u8vwx"), 38);
        assert_eq!(decode_calibration_value("a1b2c3d4e5f"), 15);
        assert_eq!(decode_calibration_value("treb7uchet"), 77);
    }
}
