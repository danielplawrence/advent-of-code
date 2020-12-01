use std::collections::HashSet;
use ::day_01;

fn main () {
    let input = parse_input(include_str!("input.txt"));
    let result = day_01::two_sum(&input, &2020);
    println!("{:?}", result);
}

pub fn parse_input(input: &str) -> HashSet<u32> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}