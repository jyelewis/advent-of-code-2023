use sscanf::sscanf;
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

    // number of possible games with the given cubes in the bag
    input
        .split("\n") // split by new line
        .filter(|line| !line.is_empty()) // drop empty lines
        .map(|line| Game::from_str(line)) // parse each value into game
        .filter(|game| game.is_possible_with_only_cube_set(&available_cubes)) // filter to only possible with our cube set
        .map(|possible_game| possible_game.id) // map to game id
        .sum() // sum game ids
}

fn challenge_part2(input: &str) -> u32 {
    // sum of powers of minimum cube set for each game
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
        let (id, game_rounds_str) = sscanf!(game_str, "Game {u32}: {str}").unwrap();

        Game {
            // "Game 12" -> 12
            id,
            // parse each provided round into a CubeSet
            game_rounds: game_rounds_str
                .split(";")
                .map(|round_str| CubeSet::from_str(round_str))
                .collect(),
        }
    }

    pub fn is_possible_with_only_cube_set(&self, available_cubes: &CubeSet) -> bool {
        // whether all the rounds within this game could have been played with only the available cubes
        self.game_rounds
            .iter()
            .all(|round| round.is_possible_with_only_cube_set(available_cubes))
    }

    pub fn minimum_cube_set(&self) -> CubeSet {
        let mut cube_set = CubeSet::default();

        // find the maximum number of each colour used in any round
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
    pub fn from_str(cubes_str: &str) -> CubeSet {
        // cubes_str: "12 blue, 5 red, 6 green"

        // start with a zeroed CubeSet, not all colours may be provided in the cubes_str
        let mut cube_set = CubeSet::default();

        for cube_color_str in cubes_str.split(",") {
            // " 12 blue" -> 12, "blue"
            let (count, color) = sscanf!(cube_color_str.trim(), "{u32} {str}").unwrap();

            match color {
                "blue" => cube_set.blue = count,
                "red" => cube_set.red = count,
                "green" => cube_set.green = count,
                _ => panic!("Unknown color: {}", color),
            }
        }

        cube_set
    }

    pub fn is_possible_with_only_cube_set(&self, available_cubes: &CubeSet) -> bool {
        // does the provided available_cubes, have enough cubes to replicate this cube_set?
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
    fn test_cubeset_from_str() {
        let cube_set = CubeSet::from_str("3 blue, 4 red");
        assert_eq!(cube_set.red, 4);
        assert_eq!(cube_set.green, 0);
        assert_eq!(cube_set.blue, 3);

        let cube_set = CubeSet::from_str("1 red, 2 green, 6 blue");
        assert_eq!(cube_set.red, 1);
        assert_eq!(cube_set.green, 2);
        assert_eq!(cube_set.blue, 6);
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
