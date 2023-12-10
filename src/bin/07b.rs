use sscanf::sscanf;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

// TODO: merge with 07a, toggle 'parseJAsJoker' flag

fn main() {
    let input = fs::read_to_string("inputs/07.txt").expect("Failed to read input file");

    let answer = challenge(&input);

    println!("Day 07, Part 2: {}", answer);
    assert_eq!(answer, 254494947);
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
    Joker,
    Number(i32),
    Queen,
    King,
    Ace,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
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
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_type_ordering = self.hand_type().cmp(&other.hand_type());

        if hand_type_ordering == Ordering::Equal {
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
                    'J' => Card::Joker,
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
        let num_jokers = *card_counts.get(&Card::Joker).unwrap_or(&0);

        // keeping track of jokers independently, so clear the count here
        // (otherwise jokers get counted twice)
        card_counts.insert(&Card::Joker, 0);

        if card_counts
            .iter()
            .any(|(_, count)| (*count + num_jokers) == 5)
        {
            return HandType::FiveOfAKind;
        }

        if card_counts
            .iter()
            .any(|(_, count)| (*count + num_jokers) == 4)
        {
            return HandType::FourOfAKind;
        }

        if card_counts.iter().any(|(card, count)| {
            // this card with jokers is 3
            (*count + num_jokers) == 3
                    // and we have a DIFFERENT card that has 2, not including jokers
                    && card_counts.iter().any(|(card2, count2)| {
                        card != card2 && *count2 == 2
                    })
        }) {
            return HandType::FullHouse;
        }
        if card_counts.iter().any(|(card, count)| {
            // this card without jokers is 3
            *count == 3
                // and we have a DIFFERENT card that has 2, including jokers
                && card_counts.iter().any(|(card2, count2)| {
                card != card2 && (*count2 + num_jokers) == 2
            })
        }) {
            return HandType::FullHouse;
        }

        if card_counts
            .iter()
            .any(|(_, count)| (*count + num_jokers) == 3)
        {
            return HandType::ThreeOfAKind;
        }

        if card_counts
            .iter()
            .filter(|(_, count)| (**count + num_jokers) == 2)
            .count()
            == 2
        {
            return HandType::TwoPair;
        }

        if card_counts
            .iter()
            .any(|(_, count)| (*count + num_jokers) == 2)
        {
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
        assert_eq!(challenge(example_input), 5905);
    }

    #[test]
    fn test_sort_cards() {
        let mut cards = vec![
            Card::Joker,
            Card::Ace,
            Card::Number(2),
            Card::King,
            Card::Number(5),
        ];
        cards.sort();

        assert_eq!(
            cards,
            vec![
                Card::Joker,
                Card::Number(2),
                Card::Number(5),
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
                Hand::from_str("KK677 3"),
                Hand::from_str("T55J5 2"),
                Hand::from_str("QQQJA 5"),
                Hand::from_str("KTJJT 4"),
            ]
        );
    }

    #[test]
    fn test_hand_types() {
        assert_eq!(Hand::from_str("32T3K 1").hand_type(), HandType::OnePair);
        assert_eq!(Hand::from_str("KK677 1").hand_type(), HandType::TwoPair);
        assert_eq!(Hand::from_str("T55J5 1").hand_type(), HandType::FourOfAKind);
        assert_eq!(Hand::from_str("KTJJT 1").hand_type(), HandType::FourOfAKind);
        assert_eq!(Hand::from_str("QQQJA 1").hand_type(), HandType::FourOfAKind);

        assert_eq!(Hand::from_str("55555 1").hand_type(), HandType::FiveOfAKind);
        assert_eq!(Hand::from_str("55J55 1").hand_type(), HandType::FiveOfAKind);
        assert_eq!(Hand::from_str("555JJ 1").hand_type(), HandType::FiveOfAKind);
        assert_eq!(Hand::from_str("JJJJJ 1").hand_type(), HandType::FiveOfAKind);
        assert_eq!(Hand::from_str("JJJJ1 1").hand_type(), HandType::FiveOfAKind);

        assert_eq!(Hand::from_str("55551 1").hand_type(), HandType::FourOfAKind);
        assert_eq!(Hand::from_str("55J51 1").hand_type(), HandType::FourOfAKind);
        assert_eq!(Hand::from_str("555J1 1").hand_type(), HandType::FourOfAKind);

        assert_eq!(
            Hand::from_str("12333 1").hand_type(),
            HandType::ThreeOfAKind
        );
        assert_eq!(
            Hand::from_str("12J33 1").hand_type(),
            HandType::ThreeOfAKind
        );
        assert_eq!(Hand::from_str("333AA 1").hand_type(), HandType::FullHouse);
        assert_eq!(Hand::from_str("A33AA 1").hand_type(), HandType::FullHouse);
        assert_eq!(
            Hand::from_str("33JAA 1").hand_type(),
            HandType::FullHouse // 3 3s, 2 As
        );
        assert_eq!(
            Hand::from_str("3JJAA 1").hand_type(),
            HandType::FourOfAKind // should snap to being the 'A'
        );
        assert_eq!(Hand::from_str("JJJAA 1").hand_type(), HandType::FiveOfAKind);
    }
}
