use sscanf::sscanf;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

fn main() {
    let input = fs::read_to_string("inputs/07.txt").expect("Failed to read input file");

    let answer = challenge(&input);

    println!("Day 07, Part 1: {}", answer);
    assert_eq!(answer, 253866470);
}

fn challenge(input: &str) -> u32 {
    let mut hands: Vec<Hand> = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(Hand::from_str)
        .collect();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx + 1) as u32)
        .sum()
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
enum Card {
    Number(i32),
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let hand_type_ordering = self.hand_type().cmp(&other.hand_type());

        if hand_type_ordering == std::cmp::Ordering::Equal {
            self.cards.cmp(&other.cards)
        } else {
            hand_type_ordering
        }
    }
}
// not sure why this can't be auto derived (clippy complains)
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn from_str(input: &str) -> Hand {
        let (cards_str, bid) = sscanf!(input, "{String} {u32}").unwrap();

        Hand {
            cards: cards_str
                .chars()
                .map(|x| match x {
                    'J' => Card::Jack,
                    'Q' => Card::Queen,
                    'K' => Card::King,
                    'A' => Card::Ace,
                    'T' => Card::Number(10),
                    _ => Card::Number(x.to_digit(10).unwrap() as i32),
                })
                .collect(),
            bid,
        }
    }

    fn hand_type(&self) -> HandType {
        let mut card_counts: HashMap<&Card, usize> = HashMap::new();
        for card in self.cards.iter() {
            let count = card_counts.entry(card).or_insert(0);
            *count += 1;
        }

        if card_counts.iter().any(|(_, count)| *count == 5) {
            return HandType::FiveOfAKind;
        }

        if card_counts.iter().any(|(_, count)| *count == 4) {
            return HandType::FourOfAKind;
        }

        if card_counts.iter().any(|(_, count)| *count == 3)
            && card_counts.iter().any(|(_, count)| *count == 2)
        {
            return HandType::FullHouse;
        }

        if card_counts.iter().any(|(_, count)| *count == 3) {
            return HandType::ThreeOfAKind;
        }

        if card_counts.iter().filter(|(_, count)| **count == 2).count() == 2 {
            return HandType::TwoPair;
        }

        if card_counts.iter().any(|(_, count)| *count == 2) {
            return HandType::OnePair;
        }

        HandType::HighCard
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
    fn test_example_input() {
        let example_input = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
        "
        .trim();
        assert_eq!(challenge(example_input), 6440);
    }

    #[test]
    fn test_sort_cards() {
        let mut cards = vec![
            Card::Jack,
            Card::Ace,
            Card::Number(2),
            Card::King,
            Card::Number(5),
        ];
        cards.sort();

        assert_eq!(
            cards,
            vec![
                Card::Number(2),
                Card::Number(5),
                Card::Jack,
                Card::King,
                Card::Ace
            ]
        );
    }

    #[test]
    fn test_sort_hands() {
        let mut hands = vec![
            Hand::from_str("32T3K 1"),
            Hand::from_str("T55J5 2"),
            Hand::from_str("KK677 3"),
            Hand::from_str("KTJJT 4"),
            Hand::from_str("QQQJA 5"),
        ];
        hands.sort();

        assert_eq!(
            hands,
            vec![
                Hand::from_str("32T3K 1"),
                Hand::from_str("KTJJT 4"),
                Hand::from_str("KK677 3"),
                Hand::from_str("T55J5 2"),
                Hand::from_str("QQQJA 5"),
            ]
        );
    }
}
