use std::str::FromStr;
use itertools::Itertools;

fn main() {
    aoc2023::run_day(
        || aoc2023::read_input_as_string().parse::<InputContext>().unwrap(),
        part1, part2);
}

fn part1(input: &InputContext) -> usize {
    input.seeds.iter().map(|s| input.map_to_location(*s)).min().unwrap()
}

fn part2(input: &InputContext) -> usize {
    input.seeds.iter()
        .tuples()
        .flat_map(|(a, b)| (*a..).take(*b))
        .map(|s| input.map_to_location(s)).min().unwrap()
}

struct InputContext {
    seeds: Vec<usize>,
    maps: Vec<RangeCollection>,
}

impl InputContext {
    fn map_to_location(&self, seed: usize) -> usize {
        self.maps.iter().fold(seed, |acc, m| m.map(acc))
    }
}

impl FromStr for InputContext {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (seeds, maps) = s.split_once("\n\n").unwrap();

        Ok(InputContext {
            seeds: seeds[7..].split_whitespace().map(|s| s.parse().unwrap()).collect(),
            maps: maps.split("\n\n").map(|r| r.parse().unwrap()).collect(),
        })
    }
}

struct RangeCollection {
    ranges: Vec<Range>,
}

impl RangeCollection {
    fn new<T: IntoIterator<Item=Range>>(iterator: T) -> Self {
        let mut ranges = iterator.into_iter().collect::<Vec<_>>();
        ranges.sort_by(|a, b| a.source_start.cmp(&b.source_start));
        RangeCollection { ranges }
    }

    fn map(&self, source: usize) -> usize {
        let index = match self.ranges.binary_search_by_key(&source, |r| r.source_start) {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };
        self.ranges[index].map(source).unwrap_or(source)
        // self.ranges.iter().filter_map(|r| r.map(source)).next().unwrap_or(source)
    }
}

impl FromStr for RangeCollection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RangeCollection::new(s.lines()
            .skip(1)
            .map(|l| l.parse().unwrap())))
    }
}

#[derive(Debug)]
struct Range {
    source_start: usize,
    destination_start: usize,
    length: usize,
}

impl Range {
    fn map(&self, source: usize) -> Option<usize> {
        if source >= self.source_start && source < self.source_start + self.length {
            Some(self.destination_start + (source - self.source_start))
        } else {
            None
        }
    }
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (destination_start, source_start, length) = s.split(' ')
            .map(|n| n.parse().unwrap())
            .next_tuple().unwrap();
        Ok(Range { destination_start, source_start, length })
    }
}

#[cfg(test)]
mod tests {
    use crate::{InputContext, part1, part2, Range, RangeCollection};

    const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_range() {
        let range1 = "50 98 2".parse::<Range>().unwrap();
        assert_eq!(range1.map(97), None);
        assert_eq!(range1.map(98), Some(50));
        assert_eq!(range1.map(99), Some(51));
        assert_eq!(range1.map(100), None);

        let range2 = "52 50 48".parse::<Range>().unwrap();
        assert_eq!(range2.map(49), None);
        (50..).take(48).for_each(|i| assert_eq!(range2.map(i), Some(i - 50 + 52)));
        assert_eq!(range2.map(98), None);
    }

    #[test]
    fn test_range_collection() {
        let input = "seed-to-soil map:
50 98 2
52 50 48";
        let range_collection = input.parse::<RangeCollection>().unwrap();
        assert_eq!(range_collection.map(0), 0);
        assert_eq!(range_collection.map(1), 1);
        assert_eq!(range_collection.map(13), 13);
        assert_eq!(range_collection.map(14), 14);
        assert_eq!(range_collection.map(48), 48);
        assert_eq!(range_collection.map(49), 49);
        assert_eq!(range_collection.map(50), 52);
        assert_eq!(range_collection.map(51), 53);
        assert_eq!(range_collection.map(55), 57);
        assert_eq!(range_collection.map(79), 81);
        assert_eq!(range_collection.map(96), 98);
        assert_eq!(range_collection.map(97), 99);
        assert_eq!(range_collection.map(98), 50);
        assert_eq!(range_collection.map(99), 51);
    }

    #[test]
    fn test_input_context() {
        let input = EXAMPLE_INPUT.parse::<InputContext>().unwrap();
        assert_eq!(input.map_to_location(79), 82);
        assert_eq!(input.map_to_location(14), 43);
        assert_eq!(input.map_to_location(55), 86);
        assert_eq!(input.map_to_location(13), 35);
    }

    #[test]
    fn test_part1() {
        let input = EXAMPLE_INPUT.parse::<InputContext>().unwrap();
        assert_eq!(part1(&input), 35);
    }

    #[test]
    fn test_part2() {
        let input = EXAMPLE_INPUT.parse::<InputContext>().unwrap();
        assert_eq!(part2(&input), 46);
    }
}