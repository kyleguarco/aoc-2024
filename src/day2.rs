use std::{cmp::Ordering, num::{NonZero, NonZeroUsize}};

use aoc_runner_derive::{aoc, aoc_generator};

// My particular input has reports no larger than eight entries.
const REPORT_MAX_SIZE: usize = 8;

// My input *also* only has 1000 entries in it!
const INPUT_LINE_COUNT: usize = 1000;

//const TOTAL_ELEM: usize = INPUT_LINE_COUNT * REPORT_MAX_SIZE;

// None of the numbers in my input are zero, sooo.... niche optimization, anyone?
type Level = Option<NonZero<usize>>;
type Report = [Level; REPORT_MAX_SIZE];
type Grid = [Report; INPUT_LINE_COUNT];

#[aoc_generator(day2)]
fn parse(input: &str) -> Grid {
    let iter = input.lines()
        .enumerate()
        .map(|(line_idx, line)| {
            line.split_whitespace()
                .enumerate()
                .map(move |(lvl_idx, n)| {
                    (line_idx, lvl_idx, n.parse::<usize>().unwrap())
                })
            })
        .flatten();

    let mut grid = {
        use core::array::from_fn;
        let init_rep = || from_fn::<_, REPORT_MAX_SIZE, _>(|_| NonZeroUsize::new(0));
        from_fn::<_, INPUT_LINE_COUNT, _>(|_| init_rep())
    };

    for (line_idx, lvl_idx, value) in iter {
        grid[line_idx][lvl_idx] = NonZeroUsize::new(value);
    }

    grid
}

#[aoc(day2, part1)]
fn count_safe_reports(grid: &Grid) -> usize {
    grid.iter()
        .enumerate()
        .inspect(|(idx, report)| eprintln!("\nReport {idx} {report:?} "))
        .map(|(_, report)| is_report_safe(report))
        .inspect(|is_safe| eprintln!("Safe: {is_safe}"))
        .filter(|is_safe| *is_safe)
        .count()
}

#[aoc(day2, part2)]
fn part2(_grid: &Grid) -> usize {
    todo!()
}

fn is_report_safe(report: &Report) -> bool {
    /// Responsible for making sure:
    /// * All levels are either all increasing or all decreasing (see parameter `prev_slope`)
    /// * Adjacent levels have an absolute difference greater than one but no more than three
    fn level_health_check(
        prev_slope: &mut Option<Ordering>,
        &[a, b]: &[Level; 2]
    ) -> Option<bool> {
        // If there are no values to compare, we've reached the end of a report.
        if a.is_none() && b.is_none() {
            return Some(true)
        }

        let cur_slope = match (b, a) {
            (None, None) => Some(Ordering::Equal),
            (None, Some(_)) => *prev_slope,
            (Some(_), None) => *prev_slope,
            (Some(b), Some(a)) => Some(b.cmp(&a)),
        };

        // `prev_slope` is only `None` on the very first comparison for a report.
        let mut is_safe = if prev_slope.is_some() {
            // Checks to make sure `prev_slope` is still either increasing or decreasing.
            // Checking if `cur_slope` is equal to the `prev_slope`.
            eprintln!("({a:?}, {b:?}) slope {cur_slope:?} to {prev_slope:?}");
            prev_slope.zip(cur_slope).is_some_and(|(p, c)| {
                p.eq(&c) && p.ne(&Ordering::Equal)
            })
        } else {
            eprintln!("({a:?}, {b:?}) slope {cur_slope:?}");
            // Return true if this is the first slope comparison (level is automatically safe)
            true
        };

        if !is_safe {
            // Short-circuit if the above comparison failed.
            return Some(false)
        }

        // Checks if the adjacent level is within the limit of absolute difference [1, 3]
        // Since `a` and `b` might be `None`, we much do the comparisons with `Option`s
        let diff = a.map(usize::from)
            .zip(b.map(usize::from))
            .map(|(a, b)| usize::abs_diff(a, b));

        is_safe &= if diff.is_some() {
            eprintln!("({a:?}, {b:?}) difference {diff:?}");
            diff.is_some_and(|diff| diff >= 1 && diff <= 3)
        } else {
            eprintln!("({a:?}, {b:?}) no difference");
            true
        };

        *prev_slope = cur_slope;
        Some(is_safe)
    }

    report.windows(2)
        .flat_map(<&[Level; 2]>::try_from)
        .scan(None, level_health_check)
        .all(|a| a)
}

