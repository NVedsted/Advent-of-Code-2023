use itertools::Itertools;

fn main() {
    aoc2023::run_day(|| prepare_input(&aoc2023::read_input_as_string()), part1, part2);
}

fn prepare_input(input: &str) -> Vec<Vec<Vec<i32>>> {
    input.lines()
        .map(|l| stabilize(l.split_whitespace().map(|n| n.parse().unwrap()).collect_vec()))
        .collect()
}

fn part1(input: &[Vec<Vec<i32>>]) -> i32 {
    input.iter()
        .map(|l| l.iter().rev().map(|l| l.last().unwrap()).sum::<i32>())
        .sum()
}

fn part2(input: &[Vec<Vec<i32>>]) -> i32 {
    input.iter()
        .map(|l| l.iter().rev().map(|l| l.first().unwrap()).cloned().reduce(|acc, e| e - acc).unwrap())
        .sum()
}

fn stabilize(start: Vec<i32>) -> Vec<Vec<i32>> {
    let mut rows = vec![start];

    while rows.last().unwrap().iter().any(|n| *n != 0) {
        let mut next = Vec::with_capacity(rows.last().unwrap().len() - 1);

        for window in rows.last().unwrap().windows(2) {
            next.push(window[1] - window[0]);
        }

        rows.push(next);
    }

    rows
}

#[cfg(test)]
mod tests {
    use crate::{prepare_input, part1, part2};

    const EXAMPLE_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&prepare_input(EXAMPLE_INPUT)), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&prepare_input(EXAMPLE_INPUT)), 2);
    }
}