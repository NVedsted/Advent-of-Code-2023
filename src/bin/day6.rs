use itertools::Itertools;

fn main() {
    aoc2023::run_day(aoc2023::read_input_as_string, part1, part2);
}

fn part1(input: &str) -> i64 {
    let races = parse_races_part1(input);
    races.iter().map(Race::win_condition_count).product()
}

fn part2(input: &str) -> i64 {
    let race = parse_race_part2(input);
    race.win_condition_count()
}

#[derive(Debug, Eq, PartialEq)]
struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn win_hold_interval(&self) -> (i64, i64) {
        let winning_distance = self.distance + 1;
        let discriminant = ((self.time.pow(2) - 4 * winning_distance) as f64).sqrt();
        let lower = -((-self.time as f64 + discriminant) / 2.0).floor() as i64;
        let upper = -((-self.time as f64 - discriminant) / 2.0).ceil() as i64;

        (lower, upper)
    }

    fn win_condition_count(&self) -> i64 {
        let (lower, upper) = self.win_hold_interval();

        upper - lower + 1
    }
}

fn parse_races_part1(input: &str) -> Vec<Race> {
    let (times, distances) = input.lines()
        .map(|l| l.split_whitespace().skip(1).map(|n| n.parse().unwrap()))
        .next_tuple()
        .unwrap();

    times.zip_eq(distances).map(|(time, distance)| Race { time, distance }).collect()
}

fn parse_race_part2(input: &str) -> Race {
    let (time, distance) = input.lines()
        .map(|l| l[11..].replace(' ', ""))
        .next_tuple()
        .unwrap();

    Race { time: time.parse().unwrap(), distance: distance.parse().unwrap() }
}

#[cfg(test)]
mod tests {
    use crate::{parse_races_part1, parse_race_part2, part1, Race, part2};

    const EXAMPLE_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 71503);
    }

    #[test]
    fn test_parse_races_part1() {
        let actual = parse_races_part1(EXAMPLE_INPUT);
        let expected = [
            Race { time: 7, distance: 9 },
            Race { time: 15, distance: 40 },
            Race { time: 30, distance: 200 },
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_races_part2() {
        assert_eq!(parse_race_part2(EXAMPLE_INPUT), Race { time: 71530, distance: 940200 });
    }

    #[test]
    fn test_win_condition_count() {
        assert_eq!(Race { time: 7, distance: 9 }.win_condition_count(), 4);
        assert_eq!(Race { time: 15, distance: 40 }.win_condition_count(), 8);
        assert_eq!(Race { time: 30, distance: 200 }.win_condition_count(), 9);
    }
}