use core::str;

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<(usize, usize)> {
    let regex = Regex::new(r"mul\(([0-9]*),([0-9]*)\)").unwrap();

    let mut mults = Vec::with_capacity(1000);
    for (num1, num2) in regex.captures_iter(input).map(|c| {
        let parse = |s: &str| s.parse::<usize>().unwrap();
        let (_, [num1, num2]) = c.extract();
        (parse(num1), parse(num2))
    }) {
        mults.push((num1, num2));
    }

    mults
}

#[aoc(day3, part1)]
fn part1(input: &[(usize, usize)]) -> usize {
    input.iter()
        .map(|(num1, num2)| num1 * num2)
        .sum()
}

#[aoc(day3, part2)]
fn part2(_input: &[(usize, usize)]) -> usize { todo!() }
