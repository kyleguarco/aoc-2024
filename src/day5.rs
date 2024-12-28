use std::{collections::HashMap, ops::Not};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse(input: &str) -> (PageOrdering, Vec<Update>) {
    /// The input for day five has two sections. `ParseState` describes those two
    /// states better than a `bool` ever would.
    enum ParseState {
        Rule,
        Update,
    }
    let mut state = ParseState::Rule;

    let mut ordering = HashMap::new();
    let mut updates = Vec::with_capacity(100);
    for line in input.lines() {
        if line.is_empty() {
            state = ParseState::Update;
        }

        match state {
            ParseState::Rule => {
                let Some(rule) = parse_rule(line) else {
                    continue;
                };

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
            ParseState::Update => {
                if let Some(update) = parse_update(line) {
                    updates.push(update);
                }
            }
        }
    }

    let ordering = PageOrdering(ordering);

    for update in &mut updates {
        update.determine_order(&ordering);
    }

    (ordering, updates)
}

#[aoc(day5, part1)]
fn part1((_, updates): &(PageOrdering, Vec<Update>)) -> usize {
    let mut sum_of_middles = 0usize;

    for update in updates {
        if update.is_ordered() {
            sum_of_middles += update.middle();
        }
    }

    sum_of_middles
}

#[aoc(day5, part2)]
fn part2((ordering, updates): &(PageOrdering, Vec<Update>)) -> usize {
    let mut sum_of_middles = 0usize;

    for idx in 0..updates.len() {
        // SAFETY: We don't step over the bounds of `updates`
        let update = updates.get(idx).unwrap();

        // This part tells us not to use updates that have proper order.
        if update.is_ordered() {
            continue;
        }

        // TODO: Having to clone here is a quirk of using cargo-aoc. The solver functions
        // cannot take owned values for some reason, which disallows for mutation.
        let mut update = update.clone();
        update.reorder(&ordering);

        sum_of_middles += update.middle();
    }

    sum_of_middles
}

type Page = usize;
type Rule = (Page, Page);

#[derive(Debug, Clone)]
struct PageOrdering(HashMap<Page, Vec<Page>>);

impl PageOrdering {
    /// Determines if `page` should be printed `before_page`.
    fn is_page_before(&self, page: Page, before_page: Page) -> Option<bool> {
        self.0
            .get(&page)
            .map(|page_order| page_order.contains(&before_page))
    }
}

#[derive(Debug, Clone)]
struct Update {
    pages: Vec<Page>,
    is_ordered: bool,
}

impl Update {
    /// Re-orders the contents of this update to fit the rules defined in `ordering`.
    fn reorder(&mut self, ordering: &PageOrdering) {
        self.pages.sort_unstable_by(|page, before_page| {
            match ordering.is_page_before(*page, *before_page) {
                Some(true) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Less,
            }
        });
    }

    /// Determines whether the contents of this update fit the rules defined in `ordering`.
    ///
    /// This method short-circuits if the contents are not in order.
    ///
    /// The value of this call is returned by [Self::is_ordered].
    fn determine_order(&mut self, ordering: &PageOrdering) {
        // NIGHTLY ONLY: feature(array_windows)
        for [page, before_page] in self.pages.array_windows() {
            if ordering
                .is_page_before(*page, *before_page)
                .is_some_and(|is_before| is_before)
                .not()
            {
                self.is_ordered = false;
                return;
            }
        }
        self.is_ordered = true;
    }

    /// Returns the page number in the middle of the update's list of content.
    fn middle(&self) -> Page {
        // SAFETY: Each update is guaranteed to have at least two elements.
        *self.pages.get(self.pages.len().div_euclid(2)).unwrap()
    }

    /// Returns the status of the last call to [Self::determine_order].
    ///
    /// Guaranteed to return `true` if [Self::reorder] was called recently.
    fn is_ordered(&self) -> bool { self.is_ordered }
}

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
    let pages = input
        .split(',')
        .map(|o| o.parse::<Page>().ok())
        .collect::<Option<_>>()?;
    Some(Update {
        pages,
        is_ordered: false,
    })
}
