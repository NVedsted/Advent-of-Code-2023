use std::borrow::Borrow;
use std::fmt::Debug;
use std::io::Read;
use std::str::FromStr;
use std::time::{Duration, Instant};

pub fn run_day<P, F1, F2, I, I1, I2, U, V>(prepare: P, part1: F1, part2: F2)
    where P: FnOnce() -> I,
          I: Borrow<I1> + Borrow<I2>,
          F1: FnOnce(&I1) -> U,
          F2: FnOnce(&I2) -> V,
          I1: ?Sized, I2: ?Sized,
          U: Debug, V: Debug {
    let (input, prepare_duration) = time_function(prepare);
    let (solution1, part1_duration) = time_function(|| part1(input.borrow()));
    let (solution2, part2_duration) = time_function(|| part2(input.borrow()));

    eprintln!("Input preparation runtime: {prepare_duration:?}");
    eprintln!("Part 1: {solution1:?} ({part1_duration:?})");
    eprintln!("Part 2: {solution2:?} ({part2_duration:?})");

    let total_duration = prepare_duration + part1_duration + part2_duration;
    eprintln!("Total runtime: {total_duration:?}");
}

pub fn time_function<F: FnOnce() -> T, T>(f: F) -> (T, Duration) {
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    (result, elapsed)
}

pub fn read_input_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    input
}

pub fn parse_lines_from_stdin<T: FromStr>() -> Vec<T> where T::Err: Debug {
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