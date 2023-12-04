use std::collections::VecDeque;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/2022_23.txt").expect("Failed to read input file");

    let answer1 = challenge_part1(&input);
    println!("2022, Day 23, Part 1: {}", answer1);
    assert_eq!(answer1, 4114);

    // Base: 7.93s / 490ms
    // Optimisation custom hashset + iterator: 610ms / 73ms
    // next: Measure where time is taken

    // let start = std::time::Instant::now();
    let answer2 = challenge_part2(&input);
    // println!("Time elapsed in challenge_part2() is: {:?}", start.elapsed());

    println!("2022, Day 23, Part 2: {}", answer2);
    assert_eq!(answer2, 970);
}

fn challenge_part1(input: &str) -> usize {
    let mut game = Game::from_str(input);

    game.tick_until(10);

    game.num_empty_ground_tiles()
}

fn challenge_part2(input: &str) -> usize {
    let mut game = Game::from_str(input);

    game.tick_until_complete();

    // num_ticks = last tick where there was movement
    // we want the FIRST tick there was NO movement, add one
    game.num_ticks + 1
}

#[derive(Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

const POSITION_MAP_WIDTH: usize = 225;
const POSITION_MAP_HEIGHT: usize = 225;
const POSITION_MAP_OFFSET: usize = 75;
struct ElfPositionMap {
    pos_spacial_idx: [u8; POSITION_MAP_WIDTH * POSITION_MAP_HEIGHT],
    // honestly probably faster to scan the area. It's like 50% dense
}

impl ElfPositionMap {
    fn new() -> ElfPositionMap {
        ElfPositionMap {
            pos_spacial_idx: [0; POSITION_MAP_WIDTH * POSITION_MAP_HEIGHT],
        }
    }

    fn index(&self, position: &ElfPosition) -> usize {
        let shifted_x = (position.x + POSITION_MAP_OFFSET as i32) as usize;
        let shifted_y = (position.y + POSITION_MAP_OFFSET as i32) as usize;

        shifted_x + shifted_y * POSITION_MAP_WIDTH
    }

    fn set(&mut self, position: &ElfPosition, value: u8) {
        self.pos_spacial_idx[self.index(position)] = value;
    }

    fn add(&mut self, position: &ElfPosition, value: u8) {
        self.pos_spacial_idx[self.index(position)] += value;
    }

    fn get(&self, position: &ElfPosition) -> u8 {
        self.pos_spacial_idx[self.index(position)]
    }

    fn iter(&self) -> ElfPositionMapIterator {
        ElfPositionMapIterator {
            elf_position_map: self,
            current_index: 0,
        }
    }
}

struct ElfPositionMapIterator<'a> {
    elf_position_map: &'a ElfPositionMap,
    current_index: usize,
}

impl<'a> Iterator for ElfPositionMapIterator<'a> {
    type Item = ElfPosition;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_index < self.elf_position_map.pos_spacial_idx.len() {
            let current_index = self.current_index;
            self.current_index += 1;

            if self.elf_position_map.pos_spacial_idx[current_index] == 1 {
                let x = (current_index % POSITION_MAP_WIDTH) as i32 - POSITION_MAP_OFFSET as i32;
                let y = (current_index / POSITION_MAP_WIDTH) as i32 - POSITION_MAP_OFFSET as i32;
                return Some(ElfPosition { x, y });
            }
        }

        None
    }
}

struct Game {
    elf_positions: ElfPositionMap,
    // TODO: coming for this next
    move_ordering: VecDeque<Direction>,
    is_complete: bool,
    num_ticks: usize,
}

impl Game {
    fn from_str(input: &str) -> Game {
        let mut elf_positions = ElfPositionMap::new();

        // optimisation: re-use elf position when loading from string
        let mut elf_position = ElfPosition { x: 0, y: 0 };

        let lines = input.split('\n').filter(|x| !x.is_empty());
        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    elf_position.x = x as i32;
                    elf_position.y = y as i32;
                    elf_positions.set(&elf_position, 1);
                }
            }
        }

        Game {
            elf_positions,
            // default move ordering
            move_ordering: VecDeque::from(vec![
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ]),
            is_complete: false,
            num_ticks: 0,
        }
    }
    fn get_next_positions(&self) -> Option<ElfPositionMap> {
        // room for optimisation here, takes ~4ms per tick in debug mode

        // for each position, compute a proposed next position for this game state
        let proposed_moves: Vec<(ElfPosition, ElfPosition)> = self
            .elf_positions
            .iter()
            .map(|elf_position| (elf_position.proposed_next_position(self), elf_position))
            // TODO: this is killing performance, we we do something smarter to iterate these?
            .collect();

        // iterate proposed moves, count proposed moves per destination
        // and check if we want to move at all
        let mut has_moves = false;

        // TODO: re-use this position map between ticks
        let mut num_elfs_wanting_to_move_into_position = ElfPositionMap::new();

        for (proposed_position, current_position) in proposed_moves.iter() {
            // if we want to move, the game isn't over - take note of this
            if *proposed_position != *current_position {
                has_moves = true;
            }

            // keep track of how many elfs want to move into this position
            num_elfs_wanting_to_move_into_position.add(proposed_position, 1);
        }

        if !has_moves {
            // we're done!
            return None;
        }

        // TODO: re-use this position map between ticks
        // iterate proposed moves, move into proposed solution if no other elfs want to move there
        let mut next_positions = ElfPositionMap::new();
        for (proposed_position, current_position) in proposed_moves {
            // keep track of how many elfs want to move into this position
            if num_elfs_wanting_to_move_into_position.get(&proposed_position) == 1 {
                next_positions.set(&proposed_position, 1);
            } else {
                // someone else wants to move here.. stay where we are
                next_positions.set(&current_position, 1);
            }
        }

        Some(next_positions)
    }

    fn rotate_move_ordering(&mut self) {
        // take the first rule, and rotate it around to the back of the list
        let first_rule = self.move_ordering.pop_front().unwrap();

        self.move_ordering.push_back(first_rule);
    }

    fn tick(&mut self) {
        if let Some(next_elf_positions) = self.get_next_positions() {
            self.elf_positions = next_elf_positions;
            self.rotate_move_ordering();
            self.num_ticks += 1;
        } else {
            // if there are no next moves the game is complete
            self.is_complete = true;
        }
    }

    fn tick_until(&mut self, num_ticks: usize) {
        while self.num_ticks < num_ticks {
            self.tick();
        }
    }

    fn tick_until_complete(&mut self) {
        while !self.is_complete {
            self.tick();
        }
    }

    fn num_empty_ground_tiles(&self) -> usize {
        let min_x = self.elf_positions.iter().map(|p| p.x).min().unwrap();
        let max_x = self.elf_positions.iter().map(|p| p.x).max().unwrap();
        let min_y = self.elf_positions.iter().map(|p| p.y).min().unwrap();
        let max_y = self.elf_positions.iter().map(|p| p.y).max().unwrap();

        let mut num_empty_ground_tiles = 0;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.elf_positions.get(&ElfPosition { x, y }) == 0 {
                    num_empty_ground_tiles += 1;
                }
            }
        }
        num_empty_ground_tiles
    }

    #[cfg(test)]
    fn to_debug_string(&self, width: i32, height: i32) -> String {
        let mut output_str = String::new();

        for y in 0..=(height - 1) {
            for x in 0..=(width - 1) {
                output_str += if self.elf_positions.get(&ElfPosition { x, y }) == 1 {
                    "#"
                } else {
                    "."
                }
            }
            output_str += "\n";
        }

        output_str.trim().to_string()
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct ElfPosition {
    x: i32,
    y: i32,
}

impl ElfPosition {
    fn proposed_next_position(&self, game: &Game) -> ElfPosition {
        let current_elf_positions = &game.elf_positions;
        let n = current_elf_positions.get(&self.north()) == 1;
        let ne = current_elf_positions.get(&self.north().east()) == 1;
        let e = current_elf_positions.get(&self.east()) == 1;
        let se = current_elf_positions.get(&self.south().east()) == 1;
        let s = current_elf_positions.get(&self.south()) == 1;
        let sw = current_elf_positions.get(&self.south().west()) == 1;
        let w = current_elf_positions.get(&self.west()) == 1;
        let nw = current_elf_positions.get(&self.north().west()) == 1;

        if !n && !ne && !e && !se && !s && !sw && !w && !nw {
            // If no other Elves are in one of those eight positions,
            // the Elf does not do anything during this round.
            // propose we stay here
            return self.clone();
        }

        for move_direction in &game.move_ordering {
            match move_direction {
                Direction::North => {
                    // If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
                    if !n && !ne && !nw {
                        return self.north();
                    }
                }
                Direction::East => {
                    // If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
                    if !e && !ne && !se {
                        return self.east();
                    }
                }
                Direction::South => {
                    // If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
                    if !s && !se && !sw {
                        return self.south();
                    }
                }
                Direction::West => {
                    // If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.
                    if !w && !nw && !sw {
                        return self.west();
                    }
                }
            }
        }

        // no where to move, stay here
        self.clone()
    }

    fn north(&self) -> ElfPosition {
        ElfPosition {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn east(&self) -> ElfPosition {
        ElfPosition {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn south(&self) -> ElfPosition {
        ElfPosition {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn west(&self) -> ElfPosition {
        ElfPosition {
            x: self.x - 1,
            y: self.y,
        }
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
    fn test_elf_positions() {
        let pos1 = ElfPosition { x: 0, y: 0 };

        let moved = pos1.north().east().east().south();
        assert_eq!(moved.x, 2);
        assert_eq!(moved.y, 0);
    }

    #[test]
    fn test_example_steps() {
        let initial_state = "
.....
..##.
..#..
.....
..##.
.....
            "
        .trim();

        let mut game = Game::from_str(initial_state);
        assert_eq!(game.to_debug_string(5, 6), initial_state);

        // after tick 1
        game.tick();
        assert_eq!(game.num_ticks, 1);
        assert_eq!(
            game.to_debug_string(5, 6),
            "
..##.
.....
..#..
...#.
..#..
.....
            "
            .trim()
        );

        // after tick 2
        game.tick();
        assert_eq!(game.num_ticks, 2);
        assert_eq!(
            game.to_debug_string(5, 6),
            "
.....
..##.
.#...
....#
.....
..#..
            "
            .trim()
        );

        // after tick 3
        game.tick();
        assert_eq!(game.num_ticks, 3);
        assert_eq!(
            game.to_debug_string(5, 6),
            "
..#..
....#
#....
....#
.....
..#..
            "
            .trim()
        );
        assert!(!game.is_complete);

        // final tick (game should not change)
        game.tick();
        // state should be same as the last tick
        assert_eq!(game.num_ticks, 3);
        assert_eq!(
            game.to_debug_string(5, 6),
            "
..#..
....#
#....
....#
.....
..#..
            "
            .trim()
        );

        // game should be marked as complete
        assert!(game.is_complete);
    }

    #[test]
    fn test_larger_example() {
        let initial_state = "
..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............
            "
        .trim();

        let mut game = Game::from_str(initial_state);
        assert_eq!(game.to_debug_string(14, 12), initial_state);

        // after tick 1
        game.tick();
        assert_eq!(game.num_ticks, 1);
        assert!(!game.is_complete);
        assert_eq!(
            game.to_debug_string(14, 12),
            "
..............
.......#......
.....#...#....
...#..#.#.....
.......#..#...
....#.#.##....
..#..#.#......
..#.#.#.##....
..............
....#..#......
..............
..............
            "
            .trim()
        );

        // after tick 2
        game.tick();
        assert_eq!(game.num_ticks, 2);
        assert!(!game.is_complete);
        assert_eq!(
            game.to_debug_string(14, 12),
            "
..............
.......#......
....#.....#...
...#..#.#.....
.......#...#..
...#..#.#.....
.#...#.#.#....
..............
..#.#.#.##....
....#..#......
..............
..............
            "
            .trim()
        );

        // after tick 3
        game.tick();
        assert_eq!(game.num_ticks, 3);
        assert!(!game.is_complete);
        assert_eq!(
            game.to_debug_string(14, 12),
            "
..............
.......#......
.....#....#...
..#..#...#....
.......#...#..
...#..#.#.....
.#..#.....#...
.......##.....
..##.#....#...
...#..........
.......#......
..............
            "
            .trim()
        );

        // after tick 4
        game.tick();
        assert_eq!(game.num_ticks, 4);
        assert!(!game.is_complete);
        assert_eq!(
            game.to_debug_string(14, 12),
            "
..............
.......#......
......#....#..
..#...##......
...#.....#.#..
.........#....
.#...###..#...
..#......#....
....##....#...
....#.........
.......#......
..............
            "
            .trim()
        );

        // after tick 5
        game.tick();
        assert_eq!(game.num_ticks, 5);
        assert!(!game.is_complete);
        assert_eq!(
            game.to_debug_string(14, 12),
            "
.......#......
..............
..#..#.....#..
.........#....
......##...#..
.#.#.####.....
...........#..
....##..#.....
..#...........
..........#...
....#..#......
..............
            "
            .trim()
        );

        // skip ahead to tick 10
        game.tick_until(10);
        assert_eq!(game.num_ticks, 10);
        assert!(!game.is_complete);
        assert_eq!(
            game.to_debug_string(14, 12),
            "
.......#......
...........#..
..#.#..#......
......#.......
...#.....#..#.
.#......##....
.....##.......
..#........#..
....#.#..#....
..............
....#..#..#...
..............
            "
            .trim()
        );

        assert_eq!(game.num_empty_ground_tiles(), 110);

        // tick forward to "end" state
        game.tick_until_complete();
        assert_eq!(game.num_ticks, 19);
        assert!(game.is_complete);
        assert_eq!(
            game.to_debug_string(14, 12),
            "
.......#......
....#......#..
..#.....#.....
......#.......
...#....#.#..#
#.............
....#.....#...
..#.....#.....
....#.#....#..
.........#....
....#......#..
.......#......
            "
            .trim()
        );
    }

    #[test]
    fn test_part2_example_expected_output() {
        let example_input = "
..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
.............."
            .trim();

        assert_eq!(challenge_part2(example_input), 20);
    }
}
