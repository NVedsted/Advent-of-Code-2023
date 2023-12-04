use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    let cards = aoc2023::parse_lines_from_stdin::<ScratchCard>();
    aoc2023::print_day(part1(&cards), part2(&cards));
}

fn part1(cards: &[ScratchCard]) -> u32 {
    cards.iter().map(ScratchCard::score).sum()
}

fn part2(cards: &[ScratchCard]) -> usize {
    let mut cards_with_count = cards.iter()
        .map(|c| (c, 1))
        .collect::<Vec<_>>();

    let mut total_cards = 0;
    for i in 0..cards_with_count.len() {
        let won_count = cards_with_count[i].0.won_count();
        let current_count = cards_with_count[i].1;
        total_cards += current_count;

        for (_, count) in &mut cards_with_count[i + 1..=i + won_count] {
            *count += current_count;
        }
    }

    total_cards
}

#[derive(Debug)]
struct ScratchCard(usize);

impl ScratchCard {
    fn won_count(&self) -> usize {
        self.0
    }

    fn score(&self) -> u32 {
        match self.won_count() as u32 {
            0 => 0,
            n => 2u32.pow(n - 1)
        }
    }
}

impl FromStr for ScratchCard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, body) = s.split_once(": ").unwrap();
        let (winning_numbers, numbers) = body.split_once(" | ").unwrap();

        let winning_numbers = winning_numbers.split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();

        Ok(ScratchCard(numbers.split_whitespace()
            .map(|n| n.parse().unwrap())
            .filter(|n| winning_numbers.contains(n))
            .count()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, ScratchCard};

    const EXAMPLE_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test1() {
        assert_eq!(part1(&aoc2023::parse_lines_from_str(EXAMPLE_INPUT)), 13);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&aoc2023::parse_lines_from_str(EXAMPLE_INPUT)), 30);
    }

    #[test]
    fn test_scratch_card_score() {
        let expected_score = [8, 2, 2, 1, 0, 0];
        let cards = aoc2023::parse_lines_from_str::<ScratchCard>(EXAMPLE_INPUT);

        for (card, score) in cards.iter().zip(&expected_score) {
            assert_eq!(card.score(), *score);
        }
    }
}