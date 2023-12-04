use sscanf::sscanf;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/04.txt").expect("Failed to read input file");

    let answer1 = challenge_part1(&input);
    println!("Day 04, Part 1: {}", answer1);
    assert_eq!(answer1, 25231);

    let answer2 = challenge_part2(&input);
    println!("Day 04, Part 2: {}", answer2);
    assert_eq!(answer2, 9721255);
}

fn challenge_part1(input: &str) -> usize {
    input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(ScratchCard::from_str)
        .map(|scratch_card| scratch_card.score())
        .sum()
}

fn challenge_part2(input: &str) -> usize {
    let mut cards: Vec<ScratchCard> = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(ScratchCard::from_str)
        .collect();

    // loop through all cards, count how many we won, increase our count for each
    for card in cards.clone().iter() {
        // loop through all the cards we've won
        for card_won_index in card.card_number..(card.card_number + card.count_winning_numbers()) {
            // increase the number of cards possessed for each card won
            // we increase this for every card we own of this number

            cards[card_won_index as usize].number_of_cards_possessed +=
                // gotta look up the card (rather than use &card) because we may have mutated it earlier
                // and the loop is running off a clone of the card set
                cards[card.card_number as usize - 1].number_of_cards_possessed;
        }
    }

    // sum up the total cards we have
    cards
        .iter()
        .map(|card| card.number_of_cards_possessed)
        .sum()
}

#[derive(Clone, Debug)]
struct ScratchCard {
    card_number: u32,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
    number_of_cards_possessed: usize,
}

impl ScratchCard {
    pub fn from_str(card_str: &str) -> ScratchCard {
        // "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
        let (_, card_number, winning_numbers_str, my_numbers_str) =
            sscanf!(card_str, "Card{str}{u32}: {str} | {str}").unwrap();

        ScratchCard {
            card_number,
            winning_numbers: read_nums(winning_numbers_str),
            my_numbers: read_nums(my_numbers_str),
            number_of_cards_possessed: 1,
        }
    }

    pub fn count_winning_numbers(&self) -> u32 {
        self.my_numbers
            .iter()
            .filter(|num| self.winning_numbers.contains(num))
            .count() as u32
    }

    pub fn score(&self) -> usize {
        if self.count_winning_numbers() == 0 {
            return 0;
        }
        i32::pow(2, self.count_winning_numbers() - 1) as usize
    }
}

fn read_nums(nums_str: &str) -> Vec<u32> {
    nums_str
        .split_whitespace()
        .map(|num_str| num_str.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_example_cards() {
        assert_eq!(
            ScratchCard::from_str("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").score(),
            8
        );
        assert_eq!(
            ScratchCard::from_str("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19").score(),
            2
        );
        assert_eq!(
            ScratchCard::from_str("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1").score(),
            2
        );
        assert_eq!(
            ScratchCard::from_str("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83").score(),
            1
        );
        assert_eq!(
            ScratchCard::from_str("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36").score(),
            0
        );
        assert_eq!(
            ScratchCard::from_str("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11").score(),
            0
        );
    }

    #[test]
    fn test_example_input_part_1() {
        let example_input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "
        .trim();

        assert_eq!(challenge_part1(example_input), 13);
    }

    #[test]
    fn test_example_input_part_2() {
        let example_input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "
        .trim();

        assert_eq!(challenge_part2(example_input), 30);
    }
}
