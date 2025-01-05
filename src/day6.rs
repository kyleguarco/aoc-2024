use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn parse(input: &str) -> Field {
    let mut field = Vec::new();
    let mut start: Option<StartingSpot> = None;

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();

        for (x, c) in line.chars().enumerate() {
            let space = parse_space(c).expect("That space character wasn't expected.");

            // Don't set `start` if the starting position was found. There should only be one.
            if start.is_none() {
                start = match space {
                    Space::Guard(dir) => Some(StartingSpot { dir, x, y }),
                    _ => None,
                };
            }

            row.push(space);
        }

        field.push(row);
    }

    let start = start.expect("The starting point was not found.");

    Field { field, start }
}

#[aoc(day6, part1)]
fn part1(field: &Field) -> usize {
    // The rules for part one are:
    // * If there is something directly in front of you, turn right 90 degrees.
    // * Otherwise, take a step forward.
    let (mut spot, mut dir) = field.spot_at_start();

    let mut visited_spots = HashSet::new();

    loop {
        // If this lands outside the grid, break out of the loop.
        let Some(obstacle) = Spot::direction_fn(dir)(spot.clone()) else {
            break;
        };

        if obstacle.is_occupied() {
            dir = dir.rotate_right();
        } else {
            visited_spots.insert(obstacle.clone());
            spot = obstacle;
        }
    }

    visited_spots.len()
}

#[aoc(day6, part2)]
fn part2(_input: &Field) -> usize { 0 }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Rotate the direction 90 degrees.
    fn rotate_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

/// The state of a space in the [Field].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Space {
    /// A spot in the [Field] that is a `.` (no obstacle).
    Empty,
    /// A spot in the [Field] that is a `#` (an obstacle).
    Obstruction,
    /// The starting position for a guard in a [Field] '^'.
    Guard(Direction),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct StartingSpot {
    dir: Direction,
    x: usize,
    y: usize,
}

/// Imperitive representation of the puzzle input.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Field {
    field: Vec<Vec<Space>>,
    start: StartingSpot,
}

impl Field {
    /// Returns a pinned location for easy traversal in the current `Field`.
    ///
    /// Returns `None` if `(x, y)` are invalid coordinates.
    fn spot_at(&self, x: usize, y: usize) -> Option<Spot> {
        let row = self.field.get(y)?;
        let _ = row.get(x)?;
        Some(Spot { field: self, x, y })
    }

    fn spot_at_start(&self) -> (Spot, Direction) {
        let StartingSpot { dir, x, y } = self.start;
        // SAFETY: `(x, y)` are guaranteed to be valid by the parsing function.
        (self.spot_at(x, y).unwrap(), dir)
    }

    /// Returns `Some(true)` if the spot at `(x, y)` is [Space::Occupied].
    ///
    /// Returns `None` if `(x, y)` are invalid coordinates.
    fn is_spot_occupied_at(&self, x: usize, y: usize) -> Option<bool> {
        self.field
            .get(y)
            .map(|row| row.get(x).is_some_and(|spot| *spot == Space::Obstruction))
    }
}

/// A pinned location for easy traversal inside a [Field].
#[derive(Hash)]
struct Spot<'f> {
    field: &'f Field,
    x: usize,
    y: usize,
}

type DirectionFn<'f> = fn(Spot<'f>) -> Option<Spot<'f>>;

impl<'f> std::fmt::Debug for Spot<'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<'f> Clone for Spot<'f> {
    fn clone(&self) -> Self {
        Self {
            // This reference to `field` is explicitly cloned.
            field: self.field,
            x: self.x,
            y: self.y,
        }
    }
}

impl PartialEq for Spot<'_> {
    // TODO: Might not be equal if the field references aren't the same.
    fn eq(&self, other: &Self) -> bool { self.x == other.x && self.y == other.y }
}

impl Eq for Spot<'_> {}

impl<'f> Spot<'f> {
    fn direction_fn(dir: Direction) -> DirectionFn<'f> {
        match dir {
            Direction::North => Self::north,
            Direction::East => Self::east,
            Direction::South => Self::south,
            Direction::West => Self::west,
        }
    }

    fn north(self) -> Option<Self> { self.field.spot_at(self.x, self.y - 1) }

    fn east(self) -> Option<Self> { self.field.spot_at(self.x + 1, self.y) }

    fn south(self) -> Option<Self> { self.field.spot_at(self.x, self.y + 1) }

    fn west(self) -> Option<Self> { self.field.spot_at(self.x - 1, self.y) }

    fn is_occupied(&self) -> bool {
        // SAFETY: This spot holds a valid reference to `field` as guaranteed by the directional methods.
        self.field.is_spot_occupied_at(self.x, self.y).unwrap()
    }
}

fn parse_space(c: char) -> Option<Space> {
    match c {
        '.' => Some(Space::Empty),
        '#' => Some(Space::Obstruction),
        '^' => Some(Space::Guard(Direction::North)),
        '>' => Some(Space::Guard(Direction::East)),
        '<' => Some(Space::Guard(Direction::West)),
        'v' => Some(Space::Guard(Direction::South)),
        _ => None,
    }
}
