use std::cmp::max;
use std::str::FromStr;

fn main() {
    let games = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().parse::<Game>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&games));
    println!("Part 2: {}", part2(&games));
}

fn part1(games: &[Game]) -> usize {
    let super_set = Reveal { red: 12, green: 13, blue: 14 };

    games.iter()
        .filter(|g| g.possible_with(&super_set))
        .map(|g| g.id)
        .sum()
}

fn part2(games: &[Game]) -> usize {
    games.iter().map(|g| g.minimum_set().power()).sum()
}

#[derive(Debug, Eq, PartialEq)]
struct Game {
    id: usize,
    reveals: Vec<Reveal>,
}

impl Game {
    fn possible_with(&self, reveal: &Reveal) -> bool {
        self.reveals.iter().all(|r| r.is_subset_of(&reveal))
    }

    fn minimum_set(&self) -> Reveal {
        self.reveals.iter().fold(Reveal::default(), |acc, r| Reveal {
            red: max(acc.red, r.red),
            green: max(acc.green, r.green),
            blue: max(acc.blue, r.blue),
        })
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (header, reveals) = s.split_once(": ").unwrap();

        Ok(Game {
            id: header[5..].parse().unwrap(),
            reveals: reveals.split("; ").map(|r| r.parse().unwrap()).collect(),
        })
    }
}

#[derive(Debug, Eq, PartialEq, Default)]
struct Reveal {
    red: usize,
    green: usize,
    blue: usize,
}

impl Reveal {
    fn is_subset_of(&self, super_set: &Reveal) -> bool {
        self.red <= super_set.red && self.green <= super_set.green && self.blue <= super_set.blue
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl FromStr for Reveal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut reveal = Reveal::default();

        s.split(", ").for_each(|p| {
            let (amount, color) = p.split_once(' ').unwrap();
            let amount = amount.parse::<usize>().unwrap();
            match color {
                "red" => reveal.red += amount,
                "green" => reveal.green += amount,
                "blue" => reveal.blue += amount,
                _ => panic!("unknown color")
            }
        });

        Ok(reveal)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Game, part1, part2, Reveal};

    const EXAMPLE_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test1() {
        let games = EXAMPLE_INPUT.lines().map(|l| l.parse::<Game>().unwrap()).collect::<Vec<_>>();
        assert_eq!(8, part1(&games));
    }

    #[test]
    fn test2() {
        let games = EXAMPLE_INPUT.lines().map(|l| l.parse::<Game>().unwrap()).collect::<Vec<_>>();
        assert_eq!(2286, part2(&games));
    }

    #[test]
    fn test_parse_reveal() {
        assert_eq!(Reveal { red: 4, blue: 3, green: 0 }, "3 blue, 4 red".parse().unwrap());
        assert_eq!(Reveal { red: 1, blue: 6, green: 2 }, "1 red, 2 green, 6 blue".parse().unwrap());
        assert_eq!(Reveal { red: 0, blue: 0, green: 2 }, "2 green".parse().unwrap());
    }

    #[test]
    fn test_reveal_subset() {
        let super_set = Reveal { red: 12, green: 13, blue: 14 };

        assert!(Reveal { red: 1, green: 2, blue: 6 }.is_subset_of(&super_set));
        assert!(Reveal { red: 12, green: 13, blue: 14 }.is_subset_of(&super_set));
        assert!(!Reveal { red: 13, green: 13, blue: 14 }.is_subset_of(&super_set));
        assert!(!Reveal { red: 12, green: 14, blue: 14 }.is_subset_of(&super_set));
        assert!(!Reveal { red: 12, green: 13, blue: 15 }.is_subset_of(&super_set));
    }

    #[test]
    fn test_parse_game() {
        assert_eq!(
            Game {
                id: 13,
                reveals: vec![
                    Reveal { red: 4, green: 0, blue: 3 },
                    Reveal { red: 1, green: 2, blue: 6 },
                    Reveal { red: 0, green: 2, blue: 0 },
                ],
            },
            "Game 13: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".parse().unwrap()
        );
    }
}