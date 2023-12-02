use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/2021_01.txt").expect("Failed to read input file");

    let answer = challenge(&input);

    println!("2021, Day 01, Part 2: {}", answer);
    assert_eq!(answer, 1724);
}

fn challenge(input: &str) -> u32 {
    // parse out input into an iter of u32s
    let depths: Vec<u32> = input
        .split('\n') // split by new line
        .filter(|x| !x.is_empty()) // drop empty lines
        .map(|x| x.parse::<u32>().unwrap()) // parse each value
        .collect();

    let mut num_depths_increases = 0;

    // take the first window "a" as our starting point
    let mut last_depth = value_for_sliding_window(&depths, 0);

    // first depth has already been consumed, so we start at the second
    for i in 1..depths.len() - 2 {
        // need to stop short of the end because our window extends forward
        let this_depth = value_for_sliding_window(&depths, i);
        if this_depth > last_depth {
            num_depths_increases += 1;
        }

        last_depth = this_depth;
    }

    num_depths_increases
}

fn value_for_sliding_window(depths: &[u32], start_index: usize) -> u32 {
    vec![
        depths[start_index],
        depths[start_index + 1],
        depths[start_index + 2],
    ]
    .iter()
    .sum()
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
        assert_eq!(challenge(example_input), 5);
    }
}
