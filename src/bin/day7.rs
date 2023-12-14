use std::cmp::Ordering;
use std::str::FromStr;
use itertools::Itertools;

fn main() {
    aoc2023::run_day(aoc2023::read_input_as_string, part1, part2);
}

fn part1(input: &str) -> i32 {
    let mut hands = parse_hands_with_bid(input);
    hands.sort_by(|x, y| x.hand.cmp(&y.hand));
    hands.iter().enumerate().map(|(i, h)| (i + 1) as i32 * h.bid).sum()
}

fn part2(input: &str) -> i32 {
    let mut hands = parse_hands_with_bid2(input);
    hands.sort_by(|x, y| x.hand.cmp(&y.hand));
    hands.iter().enumerate().map(|(i, h)| (i + 1) as i32 * h.bid).sum()
}

fn parse_hands_with_bid(input: &str) -> Vec<HandWithBid> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn parse_hands_with_bid2(input: &str) -> Vec<HandWithBid2> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug, Hash)]
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

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Card2(Card);

impl PartialOrd for Card2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card2 {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.0, other.0) {
            (Card::J, Card::J) => Ordering::Equal,
            (_, Card::J) => Ordering::Greater,
            (Card::J, _) => Ordering::Less,
            _ => self.0.cmp(&other.0)
        }
    }
}

impl From<Card> for Card2 {
    fn from(value: Card) -> Self {
        Card2(value)
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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
}

impl Hand {
    fn new(cards: [Card; 5]) -> Self {
        Self {
            cards,
            hand_type: Hand::compute_hand_type(&cards),
        }
    }

    fn compute_hand_type(cards: &[Card; 5]) -> HandType {
        let counts = cards.iter().counts();

        if counts.values().any(|c| *c == 5) {
            HandType::FiveOfAKind
        } else if counts.values().any(|c| *c == 4) {
            HandType::FourOfAKind
        } else if counts.values().any(|c| *c == 3) {
            if counts.values().any(|c| *c == 2) {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else if counts.values().filter(|c| **c == 2).count() == 2 {
            HandType::TwoPair
        } else if counts.values().any(|c| *c == 2) {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hand::new(s.chars().map_into().collect_vec().try_into().unwrap()))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct HandWithBid {
    hand: Hand,
    bid: i32,
}

impl FromStr for HandWithBid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_whitespace().next_tuple().unwrap();
        Ok(HandWithBid {
            hand: hand.parse().unwrap(),
            bid: bid.parse().unwrap(),
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Hand2 {
    hand_type: HandType,
    cards: [Card2; 5],
}

impl FromStr for Hand2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hand2::new(s.chars().map_into::<Card>().map_into().collect_vec().try_into().unwrap()))
    }
}

impl Hand2 {
    fn new(cards: [Card2; 5]) -> Self {
        Self {
            cards,
            hand_type: Hand2::compute_hand_type(&cards),
        }
    }

    fn compute_hand_type(cards: &[Card2; 5]) -> HandType {
        let (counts, jokers) = {
            let mut counts = cards.iter().cloned().counts();
            let jokers = counts.remove(&Card::J.into()).unwrap_or(0);

            (counts, jokers)
        };

        if counts.is_empty() {
            return HandType::FiveOfAKind;
        }

        let (max_card, max) = counts.iter()
            .max_by(|(_, c1), (_, c2)| c1.cmp(c2))
            .map(|(c, n)| (*c, *n))
            .unwrap();
        let next_max = counts.iter()
            .filter(|(c, _)| **c != max_card)
            .map(|(_, n)| *n).max().unwrap_or(0);

        if max + jokers >= 5 {
            HandType::FiveOfAKind
        } else if max + jokers >= 4 {
            HandType::FourOfAKind
        } else if max + jokers >= 3 {
            let remaining_jokers = jokers - (3 - max);
            if next_max + remaining_jokers >= 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else if max + jokers >= 2 {
            let remaining_jokers = jokers - (2 - max);
            if next_max + remaining_jokers >= 2 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        } else {
            HandType::HighCard
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct HandWithBid2 {
    hand: Hand2,
    bid: i32,
}

impl FromStr for HandWithBid2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_whitespace().next_tuple().unwrap();
        Ok(HandWithBid2 {
            hand: hand.parse().unwrap(),
            bid: bid.parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use Card::*;
    use crate::{Card, Card2, Hand, Hand2, HandType, HandWithBid, parse_hands_with_bid, part1, part2};

    const EXAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 5905);
    }

    #[test]
    fn test_parse_hand_with_bid() {
        let hands_with_bids = parse_hands_with_bid(EXAMPLE_INPUT);

        let expected = [
            HandWithBid { hand: Hand::new([C3, C2, T, C3, K]), bid: 765 },
            HandWithBid { hand: Hand::new([T, C5, C5, J, C5]), bid: 684 },
            HandWithBid { hand: Hand::new([K, K, C6, C7, C7]), bid: 28 },
            HandWithBid { hand: Hand::new([K, T, J, J, T]), bid: 220 },
            HandWithBid { hand: Hand::new([Q, Q, Q, J, A]), bid: 483 },
        ];

        assert_eq!(hands_with_bids, expected);
    }

    #[test]
    fn test_hand_type_and_order() {
        let five_of_a_kind = Hand::new([A, A, A, A, A]);
        assert_eq!(five_of_a_kind.hand_type, HandType::FiveOfAKind);
        let four_of_a_kind = Hand::new([A, A, C8, A, A]);
        assert_eq!(four_of_a_kind.hand_type, HandType::FourOfAKind);
        let full_house = Hand::new([C2, C3, C3, C3, C2]);
        assert_eq!(full_house.hand_type, HandType::FullHouse);
        let three_of_a_kind = Hand::new([T, T, T, C9, C8]);
        assert_eq!(three_of_a_kind.hand_type, HandType::ThreeOfAKind);
        let two_pair = Hand::new([C2, C3, C4, C3, C2]);
        assert_eq!(two_pair.hand_type, HandType::TwoPair);
        let one_pair = Hand::new([A, C2, C3, A, C4]);
        assert_eq!(one_pair.hand_type, HandType::OnePair);
        let high_card = Hand::new([C2, C3, C4, C5, C6]);
        assert_eq!(high_card.hand_type, HandType::HighCard);

        assert!(high_card < one_pair);
        assert!(one_pair < two_pair);
        assert!(two_pair < three_of_a_kind);
        assert!(three_of_a_kind < full_house);
        assert!(full_house < four_of_a_kind);
        assert!(four_of_a_kind < five_of_a_kind);
    }

    #[test]
    fn test_same_type_hand_order() {
        let stronger_hand1 = Hand::new([C3, C3, C3, C3, C2]);
        let weaker_hand1 = Hand::new([C2, A, A, A, A]);
        assert_eq!(stronger_hand1.hand_type, HandType::FourOfAKind);
        assert_eq!(weaker_hand1.hand_type, HandType::FourOfAKind);
        assert!(stronger_hand1 > weaker_hand1);

        let stronger_hand2 = Hand::new([C7, C7, C8, C8, C8]);
        let weaker_hand2 = Hand::new([C7, C7, C7, C8, C8]);
        assert_eq!(stronger_hand2.hand_type, HandType::FullHouse);
        assert_eq!(weaker_hand2.hand_type, HandType::FullHouse);
        assert!(stronger_hand2 > weaker_hand2);
    }

    #[test]
    fn test_card2_order() {
        assert_eq!(Card2::from(J).cmp(&C2.into()), Ordering::Less);
        assert_eq!(Card2::from(J).cmp(&J.into()), Ordering::Equal);
        assert_eq!(Card2::from(C2).cmp(&J.into()), Ordering::Greater);
    }

    #[test]
    fn test_hand_type_and_order2() {
        assert_eq!(Hand2::new([Card2::from(C3), Card2::from(C2), Card2::from(T), Card2::from(C3), Card2::from(K)]).hand_type, HandType::OnePair);
        assert_eq!(Hand2::new([Card2::from(T), Card2::from(C5), Card2::from(C5), Card2::from(J), Card2::from(C5)]).hand_type, HandType::FourOfAKind);
        assert_eq!(Hand2::new([Card2::from(K), Card2::from(K), Card2::from(C6), Card2::from(C7), Card2::from(C7)]).hand_type, HandType::TwoPair);
        assert_eq!(Hand2::new([Card2::from(K), Card2::from(T), Card2::from(J), Card2::from(J), Card2::from(T)]).hand_type, HandType::FourOfAKind);
        assert_eq!(Hand2::new([Card2::from(Q), Card2::from(Q), Card2::from(Q), Card2::from(J), Card2::from(A)]).hand_type, HandType::FourOfAKind);
    }
}