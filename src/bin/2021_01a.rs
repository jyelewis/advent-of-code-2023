use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/2021_01.txt").expect("Failed to read input file");

    let answer = challenge(&input);

    println!("2021, Day 01, Part 1: {}", answer);
    assert_eq!(answer, 1692);
}

fn challenge(input: &str) -> u32 {
    // parse out input into an iter of u32s
    let mut depths = input
        .split('\n') // split by new line
        .filter(|x| !x.is_empty()) // drop empty lines
        .map(|x| x.parse::<i32>().unwrap()); // parse each value

    let mut num_depths_increases = 0;

    // take the first depth as our starting point
    let mut last_depth = depths.next().unwrap();

    // first depth has already been consumed, so we start at the second
    for depth in depths {
        if depth > last_depth {
            num_depths_increases += 1;
        }

        last_depth = depth;
    }

    num_depths_increases
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
        let example_input = "\
199
200
208
210
200
207
240
269
260
263
        "
        .trim();
        assert_eq!(challenge(example_input), 7);
    }
}
