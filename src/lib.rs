use std::ops::Neg;

use aoc_runner_derive::*;

#[aoc_generator(day1)]
pub fn generate_lists(input: &str) -> (Vec<isize>, Vec<isize>) {
    let (mut list1, mut list2) = (
        Vec::with_capacity(u16::MAX.into()),
        Vec::with_capacity(u16::MAX.into())
    );

    let lines = input.lines().map(|line| {
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<isize>().unwrap(),
            iter.next().unwrap().parse::<isize>().unwrap(),
        )
    });

    for (n1, n2) in lines {
        list1.push(n1);
        list1.sort_unstable();

        list2.push(n2);
        list2.sort_unstable();
    }

    (list1, list2)
}

#[aoc(day1, part1)]
pub fn sum_dufferences((list1, list2): &(Vec<isize>, Vec<isize>)) -> usize {
    list1.iter()
        .zip(list2.iter())
        .map(|(a, b)| a.abs_diff(*b) as usize)
        .sum()
}

aoc_lib!{ year = 2024 }

