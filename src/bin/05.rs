use sscanf::sscanf;
use std::thread::JoinHandle;
use std::{fs, thread};

fn main() {
    let input = fs::read_to_string("inputs/05.txt").expect("Failed to read input file");

    let answer1 = challenge_part1(&input);
    println!("Day 05, Part 1: {}", answer1);
    assert_eq!(answer1, 382895070);

    // FIXME too slow :(
    // let answer2 = challenge_part2(&input);
    // println!("Day 05, Part 2: {}", answer2);
    // assert_eq!(answer2, 17729182);
}

fn challenge_part1(input: &str) -> i64 {
    let mut input = input.split("\n\n");

    // "seeds: 1 2 3 4 5" -> vec![1, 2, 3, 4, 5]
    let seeds_str = sscanf!(input.next().unwrap().trim(), "seeds: {str}").unwrap();
    let seeds: Vec<i64> = seeds_str
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    // treat the rest of the file as mappings
    let mappings: Vec<Mapping> = input
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(Mapping::from_str)
        .collect();

    // push each seed through all the mapping layers
    let mapped_seeds = seeds.iter().map(|seed| {
        mappings
            .iter()
            .fold(*seed, |value, mapping| mapping.map_value(value))
    });

    // find the lowest final value
    mapped_seeds.min().unwrap()
}

#[allow(dead_code)]
fn challenge_part2(input: &str) -> i64 {
    // brute force approach - absolutely criminal, might fix later

    let mut input = input.split("\n\n");

    // "seeds: 1 2 3 4 5" -> vec![1, 2, 3, 4, 5]
    let seeds_str = sscanf!(input.next().unwrap().trim(), "seeds: {str}").unwrap();
    let seed_nums: Vec<i64> = seeds_str
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    // chunk seeds into pairs (convert to vec so we can clone later)
    let seed_pairs: Vec<Vec<i64>> = seed_nums.chunks(2).map(|chunk| chunk.to_vec()).collect();

    // treat the rest of the file as mappings
    let mappings: Vec<Mapping> = input
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(Mapping::from_str)
        .collect();

    // for each seed pair, spawn a thread to process it
    seed_pairs
        .iter()
        .map(|seed_pair| {
            let this_mappings = mappings.clone();
            let this_seed_pair = seed_pair.clone();

            thread::spawn(move || {
                let start = this_seed_pair[0];
                let length = this_seed_pair[1];
                let end = start + length;

                println!("[{start} - {end}] Processing {length} items");

                let lowest_in_range = (start..end)
                    .map(|j| {
                        this_mappings
                            .iter()
                            .fold(j, |value, mapping| mapping.map_value(value))
                    })
                    .min()
                    .unwrap();

                println!(
                    "[{start} - {end}] Completed. Lowest value in range: {}",
                    lowest_in_range
                );

                lowest_in_range
            })
        })
        .collect::<Vec<JoinHandle<i64>>>()
        .into_iter()
        .map(|jh| jh.join().unwrap())
        .min()
        .unwrap()
}

#[derive(Clone)]
struct MappingRange {
    src_start: i64, // inclusive
    src_end: i64,   // inclusive
    src_dst_delta: i64,
}

impl MappingRange {
    pub fn from_str(range_str: &str) -> MappingRange {
        let (dst_start, src_start, range_length) =
            sscanf!(range_str, "{i64} {i64} {i64}").expect("Failed to parse MappingRange");

        MappingRange {
            src_start,
            src_end: src_start + range_length - 1, // keep it inclusive
            src_dst_delta: dst_start - src_start,
        }
    }

    pub fn map_value(&self, source_value: i64) -> Option<i64> {
        if self.src_start <= source_value && source_value <= self.src_end {
            return Some(source_value + self.src_dst_delta);
        }

        None
    }
}

#[derive(Clone)]
struct Mapping {
    ranges: Vec<MappingRange>,
}

impl Mapping {
    pub fn from_str(mapping_str: &str) -> Mapping {
        Mapping {
            ranges: mapping_str
                .split('\n')
                .filter(|x| !x.is_empty())
                .skip(1)
                .map(MappingRange::from_str)
                .collect(),
        }
    }

    pub fn map_value(&self, source_value: i64) -> i64 {
        for range in &self.ranges {
            if let Some(mapped_value) = range.map_value(source_value) {
                return mapped_value;
            }
        }

        // no valid mapping range, return original value as-is
        source_value
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
    fn test_parse_mapping() {
        let example_input = "
seed-to-soil map:
50 98 2
52 50 48
        "
        .trim();

        let mapping = Mapping::from_str(example_input);

        assert_eq!(mapping.ranges.len(), 2);

        assert_eq!(mapping.ranges[0].src_start, 98);
        assert_eq!(mapping.ranges[0].src_end, 99);
        assert_eq!(mapping.ranges[0].src_dst_delta, -48);

        assert_eq!(mapping.map_value(0), 0);
        assert_eq!(mapping.map_value(1), 1);
        assert_eq!(mapping.map_value(49), 49);
        assert_eq!(mapping.map_value(50), 52);
        assert_eq!(mapping.map_value(51), 53);
        assert_eq!(mapping.map_value(96), 98);
        assert_eq!(mapping.map_value(97), 99);
        assert_eq!(mapping.map_value(98), 50);
        assert_eq!(mapping.map_value(99), 51);
    }

    #[test]
    fn test_example_input_1() {
        let example_input = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
        ";

        let answer = challenge_part1(example_input);
        assert_eq!(answer, 35);
    }

    #[test]
    fn test_example_input_2() {
        let example_input = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
        ";

        let answer = challenge_part2(example_input);
        assert_eq!(answer, 46);
    }
}
