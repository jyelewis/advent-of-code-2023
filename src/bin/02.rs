use std::cmp::max;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/02.txt").expect("Failed to read input file");

    let answer1 = challenge_part1(&input);
    println!("Day 02, Part 1: {}", answer1);
    assert_eq!(answer1, 2716);

    let answer2 = challenge_part2(&input);
    println!("Day 02, Part 2: {}", answer2);
    assert_eq!(answer2, 72227);
}

fn challenge_part1(input: &str) -> u32 {
    let available_cubes = CubeSet::from_str("12 red, 13 green, 14 blue");

    input
        .split("\n") // split by new line
        .filter(|line| !line.is_empty()) // drop empty lines
        .map(|line| Game::from_str(line)) // parse each value into game
        .filter(|game| game.possible_with_only_cube_set(&available_cubes)) // filter to only possible with our cube set
        .map(|possible_game| possible_game.id) // map to game id
        .sum() // sum game ids
}

fn challenge_part2(input: &str) -> u32 {
    input
        .split("\n") // split by new line
        .filter(|line| !line.is_empty()) // drop empty lines
        .map(|line| Game::from_str(line).minimum_cube_set().power()) // parse each value into game, get minumum cube set & calculate power
        .sum() // sum powers
}

// --------------------------- Game structure ---------------------------------------------
struct Game {
    id: u32,
    game_rounds: Vec<CubeSet>,
}

impl Game {
    pub fn from_str(game_str: &str) -> Game {
        // "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green" -> Game

        let parts: Vec<&str> = game_str.split(":").collect();
        let game_rounds_str = parts[1].trim();

        let game_rounds: Vec<CubeSet> = game_rounds_str
            .split(";")
            .map(|round_str| CubeSet::from_str(round_str))
            .collect();

        Game {
            // "Game 12" -> 12
            id: parts[0]
                .split(" ")
                .skip(1)
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            game_rounds,
        }
    }

    pub fn possible_with_only_cube_set(&self, available_cubes: &CubeSet) -> bool {
        self.game_rounds
            .iter()
            .all(|round| round.possible_with_only_cube_set(available_cubes))
    }

    pub fn minimum_cube_set(&self) -> CubeSet {
        let mut cube_set = CubeSet::default();

        for round in &self.game_rounds {
            cube_set.red = max(cube_set.red, round.red);
            cube_set.green = max(cube_set.green, round.green);
            cube_set.blue = max(cube_set.blue, round.blue);
        }

        cube_set
    }
}

// --------------------------- CubeSet structure ---------------------------------------------
struct CubeSet {
    blue: u32,
    red: u32,
    green: u32,
}

impl Default for CubeSet {
    fn default() -> Self {
        CubeSet {
            blue: 0,
            red: 0,
            green: 0,
        }
    }
}

impl CubeSet {
    pub fn from_str(round_str: &str) -> CubeSet {
        // "3 blue, 4 red" -> CubeSet

        let parts: Vec<&str> = round_str.split(",").collect();
        let mut cube_set = CubeSet::default();

        for part in parts {
            let part = part.trim();

            // "12 blue" -> 12, "blue"
            let count = part.split(" ").collect::<Vec<&str>>()[0]
                .parse::<u32>()
                .unwrap();
            let color = part.split(" ").collect::<Vec<&str>>()[1];

            match color {
                "blue" => cube_set.blue = count,
                "red" => cube_set.red = count,
                "green" => cube_set.green = count,
                _ => panic!("Unknown color: {}", color),
            }
        }

        cube_set
    }

    pub fn possible_with_only_cube_set(&self, available_cubes: &CubeSet) -> bool {
        available_cubes.blue >= self.blue
            && available_cubes.red >= self.red
            && available_cubes.green >= self.green
    }

    pub fn power(&self) -> u32 {
        self.blue * self.red * self.green
    }
}

// --------------------------- Tests ---------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_example_input_part1() {
        let example_input = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "
        .trim();

        assert_eq!(challenge_part1(example_input), 8);
    }

    #[test]
    fn test_power() {
        let cube_set = CubeSet {
            red: 4,
            green: 2,
            blue: 6,
        };

        assert_eq!(cube_set.power(), 48);

        let cube_set = CubeSet {
            red: 20,
            green: 13,
            blue: 6,
        };

        assert_eq!(cube_set.power(), 1560);
    }

    #[test]
    fn test_minimum_cube_set() {
        let game = Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let mcs = game.minimum_cube_set();
        assert_eq!(mcs.red, 4);
        assert_eq!(mcs.green, 2);
        assert_eq!(mcs.blue, 6);

        let game =
            Game::from_str("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
        let mcs = game.minimum_cube_set();
        assert_eq!(mcs.red, 1);
        assert_eq!(mcs.green, 3);
        assert_eq!(mcs.blue, 4);

        let game = Game::from_str(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        );
        let mcs = game.minimum_cube_set();
        assert_eq!(mcs.red, 20);
        assert_eq!(mcs.green, 13);
        assert_eq!(mcs.blue, 6);
    }

    #[test]
    fn test_example_input_part2() {
        let example_input = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "
        .trim();

        assert_eq!(challenge_part2(example_input), 2286);
    }
}
