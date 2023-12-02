use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    input.lines().map(|l| {
        let mut digits = l.chars().filter_map(|c| c.to_digit(10));
        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);
        first * 10 + last
    }).sum()
}

fn part2(input: &str) -> u32 {
    const TEXT_DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    input.lines().map(|l| {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;

        let mut window = l;
        while let Some(next_char) = window.chars().next() {
            if let Some(digit) = next_char.to_digit(10)
                .or_else(|| TEXT_DIGITS.iter()
                    .enumerate()
                    .filter(|(_, d)| window.starts_with(*d))
                    .map(|(i, _)| (i + 1) as u32)
                    .next()) {
                if first.is_none() {
                    first = Some(digit);
                }
                last = Some(digit);
            }

            window = &window[1..];
        }

        first.unwrap() * 10 + last.unwrap()
    }).sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(142, part1(input));
    }

    #[test]
    fn test2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(281, part2(input));
    }
}