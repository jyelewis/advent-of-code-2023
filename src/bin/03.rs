use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/03.txt").expect("Failed to read input file");

    let answer1 = challenge_part1(&input);
    println!("Day 03, Part 1: {}", answer1);
    assert_eq!(answer1, 521601);

    let answer2 = challenge_part2(&input);
    println!("Day 03, Part 2: {}", answer2);
    assert_eq!(answer2, 80694070);
}

fn challenge_part1(input: &str) -> u32 {
    let input_2d = parse_input_to_grid(input);
    let numbers = EngineNumber::from_grid(&input_2d);

    // find all numbers adjacent to symbols & sum them up
    numbers
        .iter()
        .filter(|num| num.has_surrounding_symbol(&input_2d))
        .map(|num| num.value)
        .sum()
}

fn challenge_part2(input: &str) -> u32 {
    let input_2d = parse_input_to_grid(input);
    let numbers = EngineNumber::from_grid(&input_2d);

    // find all gears & sum their ratios
    input_2d
        .iter()
        .enumerate()
        // find all stars, generate list of positions
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, char)| **char == '*')
                .map(|(x, _)| (x, y))
                .collect::<Vec<(usize, usize)>>()
        })
        // gears are stars with exactly 2 surrounding numbers
        // find them, and return their gear ratio
        .filter_map(|(x, y)| {
            let adjacent_positions = [
                (x - 1, y - 1),
                (x, y - 1),
                (x + 1, y - 1),
                (x - 1, y),
                (x + 1, y),
                (x - 1, y + 1),
                (x, y + 1),
                (x + 1, y + 1),
            ];

            // brute force check every number to see if it's adjacent to this gear
            // filter down to a list of adjacent_numbers
            let adjacent_numbers: Vec<&EngineNumber> = numbers
                .iter()
                .filter(|num| {
                    adjacent_positions
                        .iter()
                        .any(|(pos_x, pos_y)| num.is_at_position(*pos_x, *pos_y))
                })
                .collect();

            if adjacent_numbers.len() == 2 {
                // found a gear - return it's ratio
                Some(adjacent_numbers[0].value * adjacent_numbers[1].value)
            } else {
                // not exactly 2 numbers? Must not be a gear
                None
            }
        })
        .sum()
}

fn parse_input_to_grid(input: &str) -> Vec<Vec<char>> {
    input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|row| row.chars().collect())
        .collect()
}

struct EngineNumber {
    x: usize,
    y: usize,
    length: usize,
    value: u32,
}

impl EngineNumber {
    fn from_grid(input_2d: &[Vec<char>]) -> Vec<EngineNumber> {
        input_2d
            .iter()
            .enumerate()
            .flat_map(|(y, row)| EngineNumber::from_row(y, row))
            .collect()
    }

    fn from_row(y: usize, row: &[char]) -> Vec<EngineNumber> {
        let mut engine_numbers: Vec<EngineNumber> = Vec::new();

        let mut numbers_chars: Vec<char> = Vec::new();
        for (x, char) in row.iter().enumerate() {
            if char.is_ascii_digit() {
                numbers_chars.push(*char);
                continue;
            }

            // non digit character, check if we have number(s) in the buffer
            if !numbers_chars.is_empty() {
                // we have a number to add
                let num = numbers_chars
                    .iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
                numbers_chars.clear();

                engine_numbers.push(EngineNumber {
                    x: x - num.to_string().len(),
                    y,
                    length: num.to_string().len(),
                    value: num,
                })
            }
        }

        engine_numbers
    }

    fn is_at_position(&self, x: usize, y: usize) -> bool {
        x >= self.x && x < self.x + self.length && y == self.y
    }

    fn has_surrounding_symbol(&self, input_2d: &[Vec<char>]) -> bool {
        // search around number (start & end are inclusive)
        let search_symbols = ['*', '#', '$', '/', '@', '&', '+', '=', '-', '%'];
        input_2d[(self.y - 1)..=(self.y + 1)].iter().any(|row| {
            row[(self.x - 1)..=(self.x + self.length)]
                .iter()
                .any(|symbol| search_symbols.contains(symbol))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_challenge_part1_example() {
        let example_input = "
...........
.467..114..
....*......
...35..633.
.......#...
.617*......
......+.58.
...592.....
.......755.
....$.*....
..664.598..
..........."
            .trim();

        assert_eq!(challenge_part1(example_input), 4361);
    }

    #[test]
    fn test_challenge_part2_example() {
        let example_input = "
...........
.467..114..
....*......
...35..633.
.......#...
.617*......
......+.58.
...592.....
.......755.
....$.*....
..664.598..
..........."
            .trim();

        assert_eq!(challenge_part2(example_input), 467835);
    }
}
