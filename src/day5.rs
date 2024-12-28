use std::{collections::HashMap, ops::Not};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse(input: &str) -> (Vec<Rule>, Vec<Update>) {
    /// The input for day five has two sections. `ParseState` describes those two
    /// states better than a `bool` ever would.
    enum ParseState {
        Rule,
        Update,
    }
    let mut state = ParseState::Rule;

    let mut rules = Vec::with_capacity(1000);
    let mut updates = Vec::with_capacity(100);
    for line in input.lines() {
        if line.is_empty() {
            state = ParseState::Update;
        }

        match state {
            ParseState::Rule => {
                if let Some(rule) = parse_rule(line) {
                    rules.push(rule);
                }
            }
            ParseState::Update => {
                if let Some(update) = parse_update(line) {
                    updates.push(update);
                }
            }
        }
    }

    (rules, updates)
}

#[aoc(day5, part1)]
fn part1((rules, updates): &(Vec<Rule>, Vec<Update>)) -> usize {
    let ordering = determine_ordering(rules);

    let mut sum_of_middles = 0usize;

    for update in updates {
        // NIGHTLY ONLY: feature(array_windows)
        let mut is_ordered = true;
        for [page, before_page] in update.array_windows() {
            if ordering
                .is_page_before(*page, *before_page)
                .is_some_and(|is_before| is_before)
                .not()
            {
                is_ordered = false;
                break;
            }
        }

        if is_ordered {
            // SAFETY: Each update is guaranteed to have at least two elements
            sum_of_middles += update.get(update.len().div_euclid(2)).unwrap();
        }
    }

    sum_of_middles
}

#[aoc(day5, part2)]
fn part2((_rules, _updates): &(Vec<Rule>, Vec<Update>)) -> usize { todo!() }

type Page = usize;
type Rule = (Page, Page);
type Update = Vec<Page>;

fn parse_rule(input: &str) -> Option<Rule> {
    let opt = input
        .split_once('|')
        .map(|(before, after)| (before.parse().ok(), after.parse().ok()));

    match opt {
        Some((Some(one), Some(two))) => Some((one, two)),
        _ => None,
    }
}

fn parse_update(input: &str) -> Option<Update> {
    input.split(',').map(|o| o.parse().ok()).collect()
}

#[derive(Debug, Clone)]
struct PageOrdering(HashMap<Page, Vec<Page>>);

fn determine_ordering(rules: &[Rule]) -> PageOrdering {
    let mut ordering = HashMap::new();

    for rule in rules {
        let page_order = ordering.get_mut(&rule.0);

        if page_order.is_none() {
            let mut pages = Vec::new();
            pages.push(rule.1);
            ordering.insert(rule.0, pages);
            continue;
        }

        // SAFETY: We checked for `None` in the body above.
        let page_order = page_order.unwrap();
        page_order.push(rule.1);
    }

    PageOrdering(ordering)
}

impl PageOrdering {
    /// Determines if `page` should be printed `before_page`.
    fn is_page_before(&self, page: Page, before_page: Page) -> Option<bool> {
        self.0
            .get(&page)
            .map(|page_order| page_order.contains(&before_page))
    }
}
