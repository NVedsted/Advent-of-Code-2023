use std::str::FromStr;
use itertools::Itertools;
use Direction::*;
use crate::Tile::*;

fn main() {
    aoc2023::run_day(|| aoc2023::read_input_as_string().parse::<Map>().unwrap(), part1, |_: &Map| ());
}

fn part1(map: &Map) -> usize {
    let start = map.start();

    let (mut runner1, mut runner2) = [North, East, West, South].into_iter()
        .filter_map(|d| {
            let position = map.move_position(start, d)?;
            map.map[position.0][position.1].output_direction(d).map(|d| (position, d))
        })
        .next_tuple().unwrap();

    let mut i = 1;
    while runner1.0 != runner2.0 {
        runner1.0 = map.move_position(runner1.0, runner1.1).unwrap();
        runner1.1 = map.map[runner1.0.0][runner1.0.1].output_direction(runner1.1).unwrap();

        runner2.0 = map.move_position(runner2.0, runner2.1).unwrap();
        runner2.1 = map.map[runner2.0.0][runner2.0.1].output_direction(runner2.1).unwrap();

        i += 1;
    }

    i
}

type Position = (usize, usize);

#[derive(Eq, PartialEq, Copy, Clone)]
enum Tile {
    Ground,
    VerticalPipe,
    HorizontalPipe,
    NorthEastPipe,
    NorthWestPipe,
    SouthEastPipe,
    SouthWestPipe,
    Start,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => VerticalPipe,
            '-' => HorizontalPipe,
            'L' => NorthEastPipe,
            'J' => NorthWestPipe,
            '7' => SouthWestPipe,
            'F' => SouthEastPipe,
            '.' => Ground,
            'S' => Start,
            _ => panic!("invalid"),
        }
    }
}

impl Tile {
    fn output_direction(&self, direction: Direction) -> Option<Direction> {
        match self {
            VerticalPipe if direction.is_vertical() => Some(direction),
            HorizontalPipe if direction.is_horizontal() => Some(direction),
            NorthEastPipe => match direction {
                South => Some(East),
                West => Some(North),
                _ => None,
            }
            NorthWestPipe => match direction {
                South => Some(West),
                East => Some(North),
                _ => None,
            }
            SouthEastPipe => match direction {
                North => Some(East),
                West => Some(South),
                _ => None,
            }
            SouthWestPipe => match direction {
                North => Some(West),
                East => Some(South),
                _ => None,
            }
            _ => None,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn is_horizontal(&self) -> bool { matches!(self, East | West) }

    fn is_vertical(&self) -> bool { matches!(self, North | South) }
}

struct Map {
    map: Vec<Vec<Tile>>,
}

impl Map {
    fn start(&self) -> Position {
        self.map.iter()
            .enumerate()
            .filter_map(|(r, l)| l.iter().position(|t| *t == Start).map(|c| (r, c)))
            .next()
            .unwrap()
    }

    fn move_position(&self, (row, col): Position, direction: Direction) -> Option<Position> {
        match direction {
            North if row > 0 => Some((row - 1, col)),
            South if row < self.map.len() - 1 => Some((row + 1, col)),
            East  if col < self.map.first().unwrap().len() - 1 => Some((row, col + 1)),
            West if col > 0 => Some((row, col - 1)),
            _ => None
        }
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map {
            map: s.lines()
                .map(|l| l.chars().map_into::<Tile>().collect_vec())
                .collect_vec()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Direction::*, Map, part1};

    const EXAMPLE_INPUT_1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const EXAMPLE_INPUT_2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn test_part1() {
        let map1 = EXAMPLE_INPUT_1.parse::<Map>().unwrap();
        assert_eq!(part1(&map1), 4);

        let map2 = EXAMPLE_INPUT_2.parse::<Map>().unwrap();
        assert_eq!(part1(&map2), 8);
    }

    #[test]
    fn test_map_start() {
        let map1 = EXAMPLE_INPUT_1.parse::<Map>().unwrap();
        assert_eq!(map1.start(), (1, 1));

        let map2 = EXAMPLE_INPUT_2.parse::<Map>().unwrap();
        assert_eq!(map2.start(), (2, 0));
    }

    #[test]
    fn test_map_move_direction() {
        let map = EXAMPLE_INPUT_1.parse::<Map>().unwrap();
        assert_eq!(map.move_position((1, 1), North), Some((0, 1)));
        assert_eq!(map.move_position((1, 1), South), Some((2, 1)));
        assert_eq!(map.move_position((1, 1), East), Some((1, 2)));
        assert_eq!(map.move_position((1, 1), West), Some((1, 0)));

        assert_eq!(map.move_position((0, 1), North), None);
        assert_eq!(map.move_position((4, 1), South), None);
        assert_eq!(map.move_position((1, 4), East), None);
        assert_eq!(map.move_position((1, 0), West), None);
    }

    #[test]
    fn test_pipe_directions() {
        let map = EXAMPLE_INPUT_1.parse::<Map>().unwrap();

        assert_eq!(map.map[1][1].output_direction(North), None);
        assert_eq!(map.map[1][1].output_direction(South), None);
        assert_eq!(map.map[1][1].output_direction(West), None);
        assert_eq!(map.map[1][1].output_direction(East), None);

        assert_eq!(map.map[0][0].output_direction(North), None);
        assert_eq!(map.map[0][0].output_direction(South), None);
        assert_eq!(map.map[0][0].output_direction(West), None);
        assert_eq!(map.map[0][0].output_direction(East), None);

        // Clock-wise
        assert_eq!(map.map[1][2].output_direction(East), Some(East));
        assert_eq!(map.map[1][3].output_direction(East), Some(South));
        assert_eq!(map.map[2][3].output_direction(South), Some(South));
        assert_eq!(map.map[3][3].output_direction(South), Some(West));
        assert_eq!(map.map[3][2].output_direction(West), Some(West));
        assert_eq!(map.map[3][1].output_direction(West), Some(North));
        assert_eq!(map.map[2][1].output_direction(North), Some(North));
    }
}