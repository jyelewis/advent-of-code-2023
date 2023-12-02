use std::fs;

fn main() {
    let input_text = fs::read_to_string("./inputs/2022_01.txt").expect("Failed to read input file");

    let mut elf_calories: Vec<u32> = input_text
        .split("\n\n") // split into groups of lines
        .map(|elf_str|
            // calculate the sum of each item in the group
            elf_str
                .split('\n')
                .filter(|x| !x.is_empty()) // split each group into lines
                .map(|line| line.trim().parse::<u32>().unwrap()) // parse as numbers
                .sum())
        .collect();

    // sort elfs by total cals (largest to smallest)
    elf_calories.sort();
    elf_calories.reverse();

    // --- answer 1 = total cals of largest elf
    let answer1 = elf_calories[0];
    println!("Answer 1: {:?}", answer1);
    assert_eq!(answer1, 71924);

    // --- answer 2 = top 3 elfs total cals summed
    // take the largest 3 elfs & sum their total cals
    let answer2: u32 = elf_calories.iter().take(3).sum();
    println!("Answer 2: {:?}", answer2);
    assert_eq!(answer2, 210406);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
