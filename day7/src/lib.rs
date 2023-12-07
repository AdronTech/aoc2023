use std::collections::HashMap;

mod debug;

// possible card types
// A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum CardType {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl CardType {
    fn from_char(c: char) -> Option<CardType> {
        match c {
            'A' => Some(CardType::Ace),
            'K' => Some(CardType::King),
            'Q' => Some(CardType::Queen),
            'J' => Some(CardType::Jack),
            'T' => Some(CardType::Ten),
            '9' => Some(CardType::Nine),
            '8' => Some(CardType::Eight),
            '7' => Some(CardType::Seven),
            '6' => Some(CardType::Six),
            '5' => Some(CardType::Five),
            '4' => Some(CardType::Four),
            '3' => Some(CardType::Three),
            '2' => Some(CardType::Two),
            _ => None,
        }
    }

    fn from_char_joker(c: char) -> Option<CardType> {
        match c {
            'A' => Some(CardType::Ace),
            'K' => Some(CardType::King),
            'Q' => Some(CardType::Queen),
            'T' => Some(CardType::Ten),
            '9' => Some(CardType::Nine),
            '8' => Some(CardType::Eight),
            '7' => Some(CardType::Seven),
            '6' => Some(CardType::Six),
            '5' => Some(CardType::Five),
            '4' => Some(CardType::Four),
            '3' => Some(CardType::Three),
            '2' => Some(CardType::Two),
            'J' => Some(CardType::Joker),
            _ => None,
        }
    }
}

fn parse_cards(input: &str) -> [CardType; 5] {
    let cards = input
        .chars()
        .map(|c| CardType::from_char(c).unwrap())
        .collect::<Vec<_>>();
    let cards = cards.as_slice();
    let mut hand = [CardType::Ace; 5];
    for i in 0..5 {
        hand[i] = cards[i];
    }
    hand
}

fn parse_cards_joker(input: &str) -> [CardType; 5] {
    let cards = input
        .chars()
        .map(|c| CardType::from_char_joker(c).unwrap())
        .collect::<Vec<_>>();
    let cards = cards.as_slice();
    let mut hand = [CardType::Ace; 5];
    for i in 0..5 {
        hand[i] = cards[i];
    }
    hand
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn new(cards: [CardType; 5]) -> HandType {
        let mut counts = HashMap::new();
        for card in cards.iter() {
            let count = counts.entry(card).or_insert(0);
            *count += 1;
        }
        let num_jokers = counts.get(&CardType::Joker).unwrap_or(&0).clone();
        // remove jokers
        counts.remove(&CardType::Joker);

        let mut counts = counts.values().cloned().collect::<Vec<_>>();
        // sort by count
        counts.sort();
        counts.reverse();

        // add jokers back
        if counts.len() == 0 {
            counts.push(0);
        }
        counts[0] += num_jokers;

        match counts.as_slice() {
            [5, ..] => HandType::FiveOfAKind,
            [4, ..] => HandType::FourOfAKind,
            [3, 2, ..] => HandType::FullHouse,
            [3, ..] => HandType::ThreeOfAKind,
            [2, 2, ..] => HandType::TwoPairs,
            [2, ..] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn from_str(input: &str) -> HandType {
        let hand = parse_cards(input);
        HandType::new(hand)
    }

    fn from_str_joker(input: &str) -> HandType {
        let hand = parse_cards_joker(input);
        HandType::new(hand)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [CardType; 5],
    bid_amount: u32,
}

impl Hand {
    fn new(cards: &str, bid_amount: u32) -> Hand {
        Hand {
            cards: parse_cards(cards),
            bid_amount,
        }
    }

    fn get_hand_type(&self) -> HandType {
        HandType::new(self.cards)
    }

    fn parse(input: &str) -> Hand {
        let input = input.split_whitespace().collect::<Vec<_>>();

        Hand {
            cards: parse_cards(input[0]),
            bid_amount: input[1].parse::<u32>().unwrap(),
        }
    }

    fn parse_joker(input: &str) -> Hand {
        let input = input.split_whitespace().collect::<Vec<_>>();

        Hand {
            cards: parse_cards_joker(input[0]),
            bid_amount: input[1].parse::<u32>().unwrap(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_hand_type = self.get_hand_type();
        let other_hand_type = other.get_hand_type();
        if self_hand_type != other_hand_type {
            return self_hand_type.cmp(&other_hand_type);
        }

        // loop through cards and compare
        for i in 0..5 {
            if self.cards[i] != other.cards[i] {
                return self.cards[i].cmp(&other.cards[i]);
            }
        }

        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn calculate_total_winning(input: &str) -> u32 {
    let mut hands = input
        .lines()
        .map(|line| Hand::parse(line))
        .collect::<Vec<_>>();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid_amount * (rank as u32 + 1))
        .sum()
}

fn calculate_total_winning_joker(input: &str) -> u32 {
    let mut hands = input
        .lines()
        .map(|line| Hand::parse_joker(line))
        .collect::<Vec<_>>();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid_amount * (rank as u32 + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cards() {
        let input = "KA538";
        let expected = [
            CardType::King,
            CardType::Ace,
            CardType::Five,
            CardType::Three,
            CardType::Eight,
        ];
        let result = parse_cards(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hand_type() {
        assert_eq!(HandType::from_str("AAAAA"), HandType::FiveOfAKind);
        assert_eq!(HandType::from_str("AA2AA"), HandType::FourOfAKind);
        assert_eq!(HandType::from_str("A2A2A"), HandType::FullHouse);
        assert_eq!(HandType::from_str("A2AA3"), HandType::ThreeOfAKind);
        assert_eq!(HandType::from_str("2AA32"), HandType::TwoPairs);
        assert_eq!(HandType::from_str("23A4A"), HandType::OnePair);
        assert_eq!(HandType::from_str("A2345"), HandType::HighCard);
    }

    #[test]
    fn test_hand_comparison() {
        let hand1 = Hand::new("AAAAA", 100);
        let hand2 = Hand::new("AA2AA", 100);
        assert!(hand1 > hand2);

        let hand1 = Hand::new("A2A2A", 100);
        let hand2 = Hand::new("A2A2A", 100);
        assert!(hand1 == hand2);

        let hand1 = Hand::new("2AA32", 100);
        let hand2 = Hand::new("AAKAK", 100);
        assert!(hand1 < hand2);
    }

    #[test]
    fn small_input() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(calculate_total_winning(input), 6440)
    }

    #[test]
    fn large_input() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(calculate_total_winning(&file), 253313241)
    }

    #[test]
    fn small_input_joker() {
        // The easiest way to open the data is to include it into the generated binary.
        let input = include_str!("../input/small.txt");
        assert_eq!(calculate_total_winning_joker(input), 5905)
    }

    #[test]
    fn large_input_joker() {
        // You can also read the file completely into memory
        let file = std::fs::read_to_string("input/big.txt").expect("Could not open input file");
        assert_eq!(calculate_total_winning_joker(&file), 253362743)
    }
    // wrong answers:
    // 252727006
    // 252956322
}
