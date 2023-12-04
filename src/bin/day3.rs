fn main() {
    let input = aoc2023::read_input_as_string();
    let (parts, labels) = parse_schematic(&input);

    println!("Part 1: {}", part1(&parts, &labels));
    println!("Part 2: {}", part2(&parts, &labels));
}

fn part1(parts: &[Part], labels: &[Label]) -> usize {
    labels.iter().filter(|l| l.has_associated_part(parts)).map(|l| l.number).sum()
}

fn part2(parts: &[Part], labels: &[Label]) -> usize {
    parts.iter().filter_map(|p| p.gear_power(labels)).sum()
}

fn parse_schematic(input: &str) -> (Vec<Part>, Vec<Label>) {
    let mut labels = vec![];
    let mut parts = vec![];

    for (row, line) in input.lines().enumerate() {
        let mut column = 0;

        while column < line.len() {
            let current_char = line[column..].chars().next().unwrap();

            if current_char.is_ascii_digit() {
                let (length, number) = try_read_digits(&line[column..]).unwrap();
                labels.push(Label { number, row, column, length });
                column += length;
                continue;
            } else if current_char != '.' {
                parts.push(Part { row, column });
            }

            column += 1;
        }
    }

    (parts, labels)
}

#[derive(Debug)]
struct Part {
    row: usize,
    column: usize,
}

impl Part {
    fn gear_power(&self, labels: &[Label]) -> Option<usize> {
        let mut associated_labels = labels.iter().filter(|l| l.is_associated_with_part(self));
        let first = associated_labels.next()?;
        let second = associated_labels.next()?;

        if associated_labels.next().is_none() {
            Some(first.number * second.number)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Label {
    number: usize,
    row: usize,
    column: usize,
    length: usize,
}

impl Label {
    fn is_associated_with_part(&self, part: &Part) -> bool {
        part.row >= self.row.saturating_sub(1)
            && part.row <= self.row + 1
            && part.column <= self.column + self.length
            && part.column >= self.column.saturating_sub(1)
    }

    fn has_associated_part(&self, parts: &[Part]) -> bool {
        parts.iter().any(|p| self.is_associated_with_part(p))
    }
}

fn try_read_digits(input: &str) -> Option<(usize, usize)> {
    let digit_length = input.chars().take_while(|c| c.is_ascii_digit()).count();

    if digit_length > 0 {
        Some((digit_length, input[..digit_length].parse().unwrap()))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{try_read_digits, parse_schematic, part1, part2, Label, Part};

    #[test]
    fn test_parse_input() {
        assert_eq!(try_read_digits("123"), Some((3, 123)));
        assert_eq!(try_read_digits("123bad"), Some((3, 123)));
        assert_eq!(try_read_digits("bad123"), None);
    }

    #[test]
    fn test1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let (parts, labels) = parse_schematic(input);
        assert_eq!(part1(&parts, &labels), 4361);
    }

    #[test]
    fn test2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let (parts, labels) = parse_schematic(input);
        assert_eq!(part2(&parts, &labels), 467835);
    }

    #[test]
    fn test_has_associated_part() {
        let label = Label {
            number: 3,
            row: 1,
            column: 1,
            length: 1,
        };

        assert!(label.has_associated_part(&[Part { row: 1, column: 1 }]));
        assert!(!label.has_associated_part(&[Part { row: 5, column: 5 }]));
    }

    #[test]
    fn test_gear_power() {
        let part = Part { row: 1, column: 1 };

        let labels = [
            Label {
                number: 3,
                row: 0,
                column: 0,
                length: 1,
            },
            Label {
                number: 5,
                row: 2,
                column: 2,
                length: 1,
            },
            Label {
                number: 7,
                row: 1,
                column: 2,
                length: 1,
            }
        ];

        assert_eq!(part.gear_power(&labels[..2]), Some(5 * 3));
        assert_eq!(part.gear_power(&[]), None);
        assert_eq!(part.gear_power(&labels), None);
    }

    #[test]
    fn test_boundary() {
        let label = Label {
            number: 123,
            row: 2,
            column: 2,
            length: 3,
        };

        for row in 1..=3 {
            for column in 1..=5 {
                assert!(label.is_associated_with_part(&Part { row, column }));
            }
        }

        for row in [0, 4] {
            for column in 0..=6 {
                assert!(!label.is_associated_with_part(&Part { row, column }));
            }
        }

        for column in [0, 6] {
            for row in 0..=4 {
                assert!(!label.is_associated_with_part(&Part { row, column }));
            }
        }
    }
}