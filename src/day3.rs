use core::str;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<(usize, usize)> {
    let mut rem = input;
    let mut mults = Vec::with_capacity(u8::MAX as usize);

    loop {
        rem = take_until(rem, "mul(").unwrap().1;
        eprintln!("{rem:?}");
        break;
    }

    mults
}

#[aoc(day3, part1)]
fn part1(input: &[(usize, usize)]) -> usize {
    todo!()
}

#[aoc(day3, part2)]
fn part2(input: &[(usize, usize)]) -> usize {
    todo!()
}

/// Forwards a string slice either until a match is found, or no input remains.
fn take_until<'a>(haystack: &'a str, needle: &str) -> Option<(usize, &'a str)> {
    let len = needle.len();

    let mut haystack = Some(haystack);
    let mut read = 0usize;

    loop {
        if let Some(haystack) = haystack {
            if haystack
                .get(..len)
                .inspect(|s| eprintln!("checking {s:?}"))
                .is_some_and(|s| s == needle)
            {
                break Some((read, haystack))
            }
        } else {
            break None
        }

        haystack = haystack.map(|s| s.get(1..)).flatten();
        read += 1;
    }
}
