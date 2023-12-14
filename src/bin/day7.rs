use std::collections::HashMap;
use itertools::Itertools;

fn main() { aoc2023::run_day(|| parse_input(&aoc2023::read_input_as_string()), part1, part2); }

fn part1(input: &Input) -> i32 { solver(input, hand_type1, |c| c.order_part1()) }

fn part2(input: &Input) -> i32 { solver(input, hand_type2, |c| c.order_part2()) }

fn solver<FT, FC>(input: &Input, hand_type: FT, card_order: FC) -> i32
    where FT: Fn(&Hand) -> HandType,
          FC: Fn(&Card) -> usize {
    let mut hands = input.with_type(hand_type);
    hands.sort_by_cached_key(|(t, h, _)| (*t, h.map(|c| card_order(&c))));
    hands.iter().enumerate().map(|(i, h)| (i + 1) as i32 * h.2).sum()
}

fn parse_input(input: &str) -> Input {
    Input {
        hands: input.lines()
            .map(|l| {
                let (hand, bid) = l.split_whitespace().next_tuple().unwrap();
                (hand.chars().map_into().collect_vec().try_into().unwrap(), bid.parse().unwrap())
            }).collect()
    }
}

type Hand = [Card; 5];

struct Input {
    hands: Vec<(Hand, i32)>,
}

impl Input {
    fn with_type<F: Fn(&Hand) -> HandType>(&self, f: F) -> Vec<(HandType, Hand, i32)> {
        self.hands.iter()
            .cloned()
            .map(|(h, b)| (f(&h), h, b))
            .collect()
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum Card {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn order_part1(&self) -> usize {
        match self {
            Card::C2 => 0,
            Card::C3 => 1,
            Card::C4 => 2,
            Card::C5 => 3,
            Card::C6 => 4,
            Card::C7 => 5,
            Card::C8 => 6,
            Card::C9 => 7,
            Card::T => 8,
            Card::J => 9,
            Card::Q => 10,
            Card::K => 11,
            Card::A => 12,
        }
    }

    fn order_part2(&self) -> usize {
        match self {
            Card::J => 0,
            _ => self.order_part1() + 1,
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::C2,
            '3' => Card::C3,
            '4' => Card::C4,
            '5' => Card::C5,
            '6' => Card::C6,
            '7' => Card::C7,
            '8' => Card::C8,
            '9' => Card::C9,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("invalid")
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn hand_type1(hand: &Hand) -> HandType { compute_hand_type(hand.iter().cloned().counts(), 0) }

fn hand_type2(hand: &Hand) -> HandType {
    let mut counts = hand.iter().cloned().counts();
    let jokers = counts.remove(&Card::J).unwrap_or(0);
    compute_hand_type(counts, jokers)
}

fn compute_hand_type(counts: HashMap<Card, usize>, jokers: usize) -> HandType {
    let counts = counts.into_values().sorted().rev().collect_vec();

    let first_max = counts.first().cloned().unwrap_or(0);
    let second_max = counts.get(1).cloned().unwrap_or(0);

    if first_max + jokers >= 5 {
        HandType::FiveOfAKind
    } else if first_max + jokers >= 4 {
        HandType::FourOfAKind
    } else if first_max + jokers >= 3 {
        let remaining_jokers = jokers - (3 - first_max);
        if second_max + remaining_jokers >= 2 {
            HandType::FullHouse
        } else {
            HandType::ThreeOfAKind
        }
    } else if first_max + jokers >= 2 {
        let remaining_jokers = jokers - (2 - first_max);
        if second_max + remaining_jokers >= 2 {
            HandType::TwoPair
        } else {
            HandType::OnePair
        }
    } else {
        HandType::HighCard
    }
}

#[cfg(test)]
mod tests {
    use Card::*;
    use crate::{Card, hand_type1, hand_type2, HandType, parse_input, part1, part2};

    const EXAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE_INPUT);
        assert_eq!(part1(&input), 6440);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE_INPUT);
        assert_eq!(part2(&input), 5905);
    }

    #[test]
    fn test_parse() {
        let input = parse_input(EXAMPLE_INPUT);

        let expected = [
            ([C3, C2, T, C3, K], 765),
            ([T, C5, C5, J, C5], 684),
            ([K, K, C6, C7, C7], 28),
            ([K, T, J, J, T], 220),
            ([Q, Q, Q, J, A], 483),
        ];

        assert_eq!(input.hands, expected);
    }

    #[test]
    fn test_hand_type1() {
        assert_eq!(hand_type1(&[A, A, A, A, A]), HandType::FiveOfAKind);
        assert_eq!(hand_type1(&[A, A, C8, A, A]), HandType::FourOfAKind);
        assert_eq!(hand_type1(&[C2, C3, C3, C3, C2]), HandType::FullHouse);
        assert_eq!(hand_type1(&[T, T, T, C9, C8]), HandType::ThreeOfAKind);
        assert_eq!(hand_type1(&[C2, C3, C4, C3, C2]), HandType::TwoPair);
        assert_eq!(hand_type1(&[A, C2, C3, A, C4]), HandType::OnePair);
        assert_eq!(hand_type1(&[C2, C3, C4, C5, C6]), HandType::HighCard);
    }

    #[test]
    fn test_hand_type2() {
        assert_eq!(hand_type2(&[A, A, A, A, A]), HandType::FiveOfAKind);
        assert_eq!(hand_type2(&[J, J, J, J, J]), HandType::FiveOfAKind);
        assert_eq!(hand_type2(&[C3, C2, T, C3, K]), HandType::OnePair);
        assert_eq!(hand_type2(&[T, C5, C5, J, C5]), HandType::FourOfAKind);
        assert_eq!(hand_type2(&[K, K, C6, C7, C7]), HandType::TwoPair);
        assert_eq!(hand_type2(&[K, T, J, J, T]), HandType::FourOfAKind);
        assert_eq!(hand_type2(&[Q, Q, Q, J, A]), HandType::FourOfAKind);
    }
}