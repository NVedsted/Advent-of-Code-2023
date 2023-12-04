use std::fmt::Debug;
use std::io::Read;
use std::str::FromStr;

pub fn print_day<U: Debug, V: Debug>(solution1: U, solution2: V) {
    println!("Part 1: {:?}", solution1);
    println!("Part 2: {:?}", solution2);
}

pub fn read_input_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    input
}

pub fn parse_input_lines<T: FromStr>() -> Vec<T> where T::Err: Debug {
    std::io::stdin()
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
}

pub fn parse_lines_from_str<T: FromStr>(input: &'static str) -> Vec<T> where T::Err: Debug {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}