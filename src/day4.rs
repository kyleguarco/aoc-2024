use std::fmt::Debug;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse(input: &str) -> Grid {
    let mut rows = Vec::with_capacity(1000);

    for line in input.lines() {
        rows.push(line.chars().collect());
    }

    Grid { rows }
}

#[aoc(day4, part1)]
fn part1(grid: &Grid) -> usize {
    let mut matches = 0usize;

    std::thread::scope(|scope| {
        for (y, line) in grid.rows.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                // If the current character isn't an 'X', loop again!
                // We don't care about any other letters.
                if c != &'X' {
                    continue;
                }

                // A collection of scoped thread handles. When the loop below exits,
                // we join the eight threads below and check their return value.
                let mut handles = [const { None }; 8];

                // Search for the remaining letters of "XMAS" ('MAS' since we already checked for 'X').
                // Spawn threads that search in the eight cardinal directions to speed things up.
                for (dir, handle) in [
                    Spot::north,
                    Spot::north_east,
                    Spot::east,
                    Spot::south_east,
                    Spot::south,
                    Spot::south_west,
                    Spot::west,
                    Spot::north_west,
                ]
                .iter()
                .zip(handles.iter_mut())
                {
                    let prev = handle.replace(scope.spawn(move || {
                        let mut spot = grid.spot_at(x, y).unwrap();

                        // The 'X' is skipped, since checking for the 'X' is what starts this thread.
                        let mut check = Some("MAS");

                        while let Some(curr) = dir(spot) {
                            let Some(curr_check) = check.map(|s| s.chars().nth(0)).flatten() else {
                                break;
                            };

                            if !curr.value_is(&curr_check) {
                                break;
                            }

                            spot = curr;
                            check = check.map(|s| s.get(1..)).flatten();
                        }

                        // We've succeeded if there are no more characters in `check`.
                        // If `check` is `None`, then we failed to get a character, likely
                        // past the border of the word search grid.
                        check.is_some_and(|s| s.is_empty())
                    }));
                    assert!(prev.is_none());
                }

                for handle in &mut handles {
                    let Some(handle) = handle.take() else {
                        continue;
                    };
                    matches += handle.join().unwrap_or(false) as usize;
                }
            }
        }
    });

    matches
}

#[aoc(day4, part2)]
fn part2(_grid: &Grid) -> usize { todo!() }

#[derive(Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self { Self { x, y } }

    fn north(self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn east(self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn south(self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn west(self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
struct Spot<'grid> {
    grid: &'grid Grid,
    loc: Pos,
    value: &'grid char,
}

impl<'grid> Spot<'grid> {
    fn north(self) -> Option<Self> {
        let Pos { x, y } = self.loc.north();
        self.grid.spot_at(x, y)
    }

    fn north_east(self) -> Option<Self> {
        let Pos { x, y } = self.loc.east().north();
        self.grid.spot_at(x, y)
    }

    fn east(self) -> Option<Self> {
        let Pos { x, y } = self.loc.east();
        self.grid.spot_at(x, y)
    }

    fn south_east(self) -> Option<Self> {
        let Pos { x, y } = self.loc.east().south();
        self.grid.spot_at(x, y)
    }

    fn south(self) -> Option<Self> {
        let Pos { x, y } = self.loc.south();
        self.grid.spot_at(x, y)
    }

    fn south_west(self) -> Option<Self> {
        let Pos { x, y } = self.loc.west().south();
        self.grid.spot_at(x, y)
    }

    fn west(self) -> Option<Self> {
        let Pos { x, y } = self.loc.west();
        self.grid.spot_at(x, y)
    }

    fn north_west(self) -> Option<Self> {
        let Pos { x, y } = self.loc.west().north();
        self.grid.spot_at(x, y)
    }

    fn value_is(&self, other: &char) -> bool { self.value == other }
}

#[derive(Debug, Clone)]
struct Grid {
    rows: Vec<Vec<char>>,
}

impl Grid {
    fn spot_at(&self, x: usize, y: usize) -> Option<Spot<'_>> {
        let value = self.rows.get(y).map(|row| row.get(x)).flatten()?;
        Some(Spot {
            grid: self,
            loc: Pos::new(x, y),
            value,
        })
    }
}
