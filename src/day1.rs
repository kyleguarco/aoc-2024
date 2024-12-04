use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashMap;

#[aoc_generator(day1)]
pub fn generate_lists(input: &str) -> (Vec<usize>, Vec<usize>) {
    // Generate a Vec with a large capacity to avoid extra allocations
    let (mut left, mut right) = (
        Vec::with_capacity(u16::MAX.into()),
        Vec::with_capacity(u16::MAX.into())
    );

    // Map each line of the input to a tuple of two integers....
    let lines = input.lines().map(|line| {
        // ...by skipping the whitespace between both numbers...
        let mut iter = line.split_whitespace();
        // ...and parsing the string representation into a number.
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    });

    // Add each pair of numbers from the lazy iterator `lines` to both lists.
    for (n1, n2) in lines {
        left.push(n1);
        right.push(n2);
    }

    // Both lists are expected to be the same size!
    assert!(left.len() == right.len());

    // Sort both lists using std's implementation of ipnsort
    // See https://github.com/Voultapher/sort-research-rs/tree/main/ipnsort
    left.sort_unstable();
    right.sort_unstable();

    (left, right)
}

#[aoc(day1, part1)]
pub fn sum_dufferences((left, right): &(Vec<usize>, Vec<usize>)) -> usize {
    // Transform the numbers from both lists into a sum of differences by...
    left.iter()
        // ...zipping the first list's iterator with the second list's iterator...
        // (makes next() return `(isize, isize)` instead of `isize`)
        .zip(right.iter())
        // ...and taking the difference of each element from the first list with the
        // corresponding element from the second list...
        // (makes next() return `usize` instead of `(isize, isize)`)
        .map(|(a, b)| a.abs_diff(*b))
        // ...and adding each difference together.
        .sum()
}

#[aoc(day1, part2)]
pub fn similarity_score((left, right): &(Vec<usize>, Vec<usize>)) -> usize {
    // Tracks the number of occurences in the `right` list for numbers in the `left` list.
    let mut map = HashMap::new();

    // Counts the occurences of numbers in `right`.
    // IT IS ASSUMED THAT NUMBERS IN `right` CAN APPEAR IN `left`.
    for n2 in right {
        match map.get(n2) {
            Some(val) => map.insert(n2, val + 1),
            None => map.insert(n2, 1usize),
        };
    }

    // An iterator that returns the number of occurences of an element in an iterator
    // IT IS ASSUMED THAT THE ITERATOR IS SORTED (i.e `Iterator::sorted` is true)
    let occurrence_iter = {
        // Create an iterator that allows us to look at the next element without consuming it.
        let mut iter = left.iter().peekable();
        std::iter::from_fn(move || {
            // Start counting `num`...
            let num = iter.next()?;
            // ...and assume there is at least one occurrence if `num` is `Some`...
            let mut occurrence = 1usize;
            // ...and continue to count occurrences so long as the next numer is equal to `num`...
            while iter.peek().eq(&Some(&num)) {
                occurrence += 1;
                // ...and consume the next element in the iterator if it is.
                iter.next();
            }
            Some((*num, occurrence))
        })
    };

    // Finally, reduce the values returned by the `occurrence_iter` by...
    occurrence_iter.fold(0, |acc, (num, occurrence)| {
        // ...multiplying the number of occurrences of `num` in `right` by the numbers
        // of occurrences in `left`.
        acc + (num * map.get(&num).unwrap_or(&0)) * occurrence
    })
}

