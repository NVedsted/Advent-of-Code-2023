fn main() {
    aoc2023::run_day(aoc2023::read_input_as_string, part1, part2);
}

fn part1(input: &str) -> u32 {
    input.lines().map(|l| {
        let mut digits = l.chars().filter_map(|c| c.to_digit(10));
        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);
        first * 10 + last
    }).sum()
}

const TEXT_DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn part2(input: &str) -> u32 {
    input.lines().map(|l| {
        let (first, last) = (0..l.len())
            .map(|i| &l[i..])
            .fold((None, None), |acc @ (first, _), w| parse_digit(w)
                .map(|digit| (first.or(Some(digit)), Some(digit)))
                .unwrap_or(acc));

        first.zip(last).map(|(first, last)| first * 10 + last).unwrap()
    }).sum()
}

fn parse_digit(input: &str) -> Option<u32> {
    let next_char = input.chars().next()?;
    next_char.to_digit(10).or_else(|| TEXT_DIGITS.iter()
        .enumerate()
        .filter(|(_, d)| input.starts_with(*d))
        .map(|(i, _)| (i + 1) as u32)
        .next())
}

#[cfg(test)]
mod tests {
    use crate::{parse_digit, part1, part2, TEXT_DIGITS};

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

    #[test]
    fn test_parse_digit() {
        for (i, digit) in TEXT_DIGITS.iter().enumerate() {
            assert_eq!(parse_digit(digit), Some(i as u32 + 1));
        }

        for i in 1..=9 {
            let digit = char::from_digit(i, 10).unwrap();
            assert_eq!(parse_digit(&format!("{}", digit)), Some(i));
        }

        assert_eq!(parse_digit("a1"), None);
        assert_eq!(parse_digit("1abcd"), Some(1));
        assert_eq!(parse_digit("oneabcd"), Some(1));
    }
}