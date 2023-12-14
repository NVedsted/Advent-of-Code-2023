use std::collections::HashMap;
use std::str::FromStr;
use itertools::{FoldWhile, Itertools};
use num::Integer;
use crate::Direction::{Left, Right};

fn main() {
    aoc2023::run_day(|| aoc2023::read_input_as_string().parse::<Context>().unwrap(), part1, part2);
}

fn part1(context: &Context) -> usize {
    solve_one(context, ['A', 'A', 'A'], |c| c == ['Z', 'Z', 'Z'])
}

fn solve_one<F: Fn([char; 3]) -> bool>(context: &Context, start: Name, end_condition: F) -> usize {
    context.directions.iter()
        .cycle()
        .fold_while((start, 0), |(current, i), direction| {
            let (left, right) = &context.junctions[&current];

            let next = match direction {
                Left => *left,
                Right => *right,
            };

            if end_condition(next) {
                FoldWhile::Done((next, i + 1))
            } else {
                FoldWhile::Continue((next, i + 1))
            }
        }).into_inner().1
}

fn part2(context: &Context) -> usize {
    context.junctions.keys()
        .filter(|k| k[2] == 'A')
        .cloned()
        .map(|start| solve_one(context, start, |c| c[2] == 'Z'))
        .reduce(|acc, e| acc.lcm(&e)).unwrap()
}

struct Context {
    directions: Directions,
    junctions: Junctions,
}

impl FromStr for Context {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let directions = lines.next().unwrap().chars().map_into().collect::<Directions>();

        let junctions = lines.skip(1)
            .map(|l| (
                l[0..3].chars().collect_vec().try_into().unwrap(),
                (l[7..10].chars().collect_vec().try_into().unwrap(), l[12..15].chars().collect_vec().try_into().unwrap())))
            .collect::<Junctions>();

        Ok(Context { directions, junctions })
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Left,
            'R' => Right,
            _ => panic!("invalid"),
        }
    }
}

type Directions = Vec<Direction>;
type Name = [char; 3];
type Junctions = HashMap<Name, (Name, Name)>;

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let input1 = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(&input1.parse().unwrap()), 2);

        let input2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(&input2.parse().unwrap()), 6);
    }

    #[test]
    fn test_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(part2(&input.parse().unwrap()), 6);
    }
}