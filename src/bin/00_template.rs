use std::fs;

fn main() {
    let input_text = fs::read_to_string("inputs/00.txt").expect("Failed to read input file");

    let items = input_text
        .split("\n") // split by new line
        .filter(|x| !x.is_empty()) // drop empty lines
        .map(|x| x.parse::<i32>().unwrap()); // parse each value

    items.for_each(|x| println!("{}", x)); // print each value
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_main() {
//         main();
//     }
// }
