use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/01.txt").expect("Failed to read input file");

    let answer = challenge(&input);

    println!("Day 00, Part 1: {}", answer);
    // assert_eq!(answer, 0);
}

fn challenge(input: &str) -> u32 {
    let items = input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap()); // parse each value

    items.for_each(|x| println!("{}", x));

    return 0;
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_main() {
//         main();
//     }
//
//     #[test]
//     fn test_example_input() {
//         let example_input = "???";
//         assert_eq!(challenge(example_input), 0);
//     }
// }
