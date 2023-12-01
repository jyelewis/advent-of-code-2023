use std::fs;

fn main() {
    let input_text = fs::read_to_string("./inputs/2022_04.txt").expect("Failed to read input file");

    // parse input into AssignmentPairs
    let assignment_pairs: Vec<AssignmentPair> = input_text
        .split("\n") // split by new line
        .filter(|x| !x.is_empty()) // drop empty lines
        .map(|line| AssignmentPair::from_str(line))
        .collect();

    // find which of these "fully contain" the other
    let answer1 = assignment_pairs
        .iter()
        .filter(|ap| ap.fully_contains())
        .count();

    println!("Day 4, part 1: {}", answer1);
    assert_eq!(answer1, 580);

    let answer2 = assignment_pairs.iter().filter(|ap| ap.overlaps()).count();
    println!("Day 4, part 2: {}", answer2);
    assert_eq!(answer2, 895);
}

struct AssignmentPair {
    // all start/ends are inclusive
    a_start: u32,
    a_end: u32,
    b_start: u32,
    b_end: u32,
}

impl AssignmentPair {
    pub fn from_str(assignment_pair_str: &str) -> Self {
        let nums: Vec<u32> = assignment_pair_str
            .split([',', '-'])
            .map(|num_str| num_str.parse().unwrap())
            .collect();

        assert_eq!(nums.len(), 4);

        AssignmentPair {
            a_start: nums[0],
            a_end: nums[1],
            b_start: nums[2],
            b_end: nums[3],
        }
    }

    pub fn fully_contains(&self) -> bool {
        // a: ......XXXXXX......
        // b: ........XXXX......
        (self.a_start <= self.b_start && self.a_end >= self.b_end) ||

        // a: ........XXXX......
        // b: ......XXXXXX......
        (self.b_start <= self.a_start && self.b_end >= self.a_end)
    }

    pub fn overlaps(&self) -> bool {
        // a: ......XXXXXX......
        // b: ........XXXXX.....
        (self.a_start <= self.b_start && self.b_start <= self.a_end) ||

        // a: ........XXXXX.....
        // b: ......XXXXXX......
        (self.b_start <= self.a_start && self.a_start <= self.b_end)
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
    fn test_ap_from_str() {
        let ap = AssignmentPair::from_str("1-2,3-4");

        assert_eq!(ap.a_start, 1);
        assert_eq!(ap.a_end, 2);
        assert_eq!(ap.b_start, 3);
        assert_eq!(ap.b_end, 4);
    }

    #[test]
    fn test_pair_fully_contains_other_pair() {
        assert_eq!(AssignmentPair::from_str("1-2,3-4").fully_contains(), false);
        assert_eq!(AssignmentPair::from_str("3-4,1-2").fully_contains(), false);
        assert_eq!(AssignmentPair::from_str("1-4,2-3").fully_contains(), true);
        assert_eq!(AssignmentPair::from_str("2-3,1-4").fully_contains(), true);
    }

    #[test]
    fn test_overlap() {
        // provided example cases
        assert_eq!(AssignmentPair::from_str("5-7,7-9").overlaps(), true);
        assert_eq!(AssignmentPair::from_str("2-8,3-7").overlaps(), true);
        assert_eq!(AssignmentPair::from_str("6-6,4-6").overlaps(), true);
        assert_eq!(AssignmentPair::from_str("2-6,4-8").overlaps(), true);

        // custom example cases
        assert_eq!(AssignmentPair::from_str("1-2,3-4").overlaps(), false);
        assert_eq!(AssignmentPair::from_str("3-4,1-2").overlaps(), false);

        assert_eq!(AssignmentPair::from_str("1-4,2-3").overlaps(), true);
        assert_eq!(AssignmentPair::from_str("2-3,1-4").overlaps(), true);

        assert_eq!(AssignmentPair::from_str("1-3,2-4").overlaps(), true);
        assert_eq!(AssignmentPair::from_str("2-4,1-3").overlaps(), true);
    }
}
