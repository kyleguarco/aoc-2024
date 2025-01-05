use std::io::Write;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn parse(input: &str) -> Field {
    let mut field = Vec::new();
    let mut start: Option<StartingSpot> = None;

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();

        for (x, c) in line.chars().enumerate() {
            let space = Space::try_from(c).expect("That space character wasn't expected.");

            // Don't set `start` if the starting position was found. There should only be one.
            if start.is_none() {
                start = match space {
                    Space::Guard(dir) => Some(((x, y), dir)),
                    _ => None,
                };
            } else if start.is_some() && space.is_guard() {
                panic!("More than one starting space for the guard was found.");
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

    // TODO: This is a quirk of using cargo-aoc: The input cannot be mutated because it is not an owned type.
    let mut field = field.clone();

    let (coord, mut dir) = field.starting_coord();
    // SAFETY: `coord` is guaranteed to be valid by the input generator.
    let mut spot = field.spot_at_mut(coord).unwrap();
    // Mark the initial spot as visited.
    spot.mark();

    let mut visited_spots = 1usize;

    loop {
        // If this lands outside the grid, break out of the loop.
        let Some(mut obstacle) = SpotMut::direction_fn(dir)(spot) else {
            break;
        };

        if obstacle.is_occupied() {
            // Do a 180 and turn around.
            // SAFETY: We would have broken out of the loop if `obstacle`` was outside the grid (see above body).
            spot = SpotMut::direction_fn(dir.rotate_right().rotate_right())(obstacle).unwrap();
            // Turn right on the next iteration.
            dir = dir.rotate_right();
            continue;
        }

        if !obstacle.is_marked() {
            obstacle.mark();
            visited_spots += 1;
        }

        spot = obstacle;
    }

    visited_spots
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
    /// The space has been visited before (an 'X').
    Marked,
}

impl TryFrom<char> for Space {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Space::Empty),
            '#' => Ok(Space::Obstruction),
            '^' => Ok(Space::Guard(Direction::North)),
            '>' => Ok(Space::Guard(Direction::East)),
            '<' => Ok(Space::Guard(Direction::West)),
            'v' => Ok(Space::Guard(Direction::South)),
            'X' => Ok(Space::Marked),
            _ => Err(()),
        }
    }
}

impl From<Space> for char {
    fn from(value: Space) -> Self {
        match value {
            Space::Empty => '.',
            Space::Obstruction => '#',
            Space::Guard(Direction::North) => '^',
            Space::Guard(Direction::East) => '>',
            Space::Guard(Direction::West) => '<',
            Space::Guard(Direction::South) => 'v',
            Space::Marked => 'X',
        }
    }
}

impl Space {
    fn is_guard(&self) -> bool {
        match self {
            Space::Guard(_) => true,
            _ => false,
        }
    }
}

/// Represents a `(x, y)` coordinate pair.
type Coord = (usize, usize);

type StartingSpot = (Coord, Direction);

/// Imperitive representation of the puzzle input.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Field {
    field: Vec<Vec<Space>>,
    start: StartingSpot,
}

impl Field {
    /// Returns the starting coordinates and direction of the guard in the current `Field`.
    fn starting_coord(&self) -> StartingSpot { self.start }

    /// Returns a pinned location for easy traversal in the current `Field`.
    ///
    /// Returns `None` if `(x, y)` are invalid coordinates.
    fn spot_at(&self, (x, y): Coord) -> Option<Spot> {
        let row = self.field.get(y)?;
        let _ = row.get(x)?;
        Some(Spot {
            field: self,
            coord: (x, y),
        })
    }

    /// Returns `Some(true)` if the spot at `(x, y)` is [Space::Obstruction].
    ///
    /// Returns `None` if `(x, y)` are invalid coordinates.
    fn is_occupied_at(&self, (x, y): Coord) -> Option<bool> {
        self.field
            .get(y)
            .map(|row| row.get(x).is_some_and(|spot| *spot == Space::Obstruction))
    }

    /// Returns `Some(true)` if the spot at `(x, y)` is [Space::Marked].
    ///
    /// Returns `None` if `(x, y)` are invalid coordinates.
    fn is_marked_at(&self, (x, y): Coord) -> Option<bool> {
        self.field
            .get(y)
            .map(|row| row.get(x).is_some_and(|spot| *spot == Space::Marked))
    }

    /// Returns a pinned location for easy traversal in the current `Field`.
    ///
    /// Returns `None` if `(x, y)` are invalid coordinates.
    fn spot_at_mut(&mut self, (x, y): Coord) -> Option<SpotMut> {
        let row = self.field.get(y)?;
        let _ = row.get(x)?;
        Some(SpotMut {
            field: self,
            coord: (x, y),
        })
    }

    /// Marks the spot at `(x, y)` as visited. The value returned by `Some` indicates whether the spot was marked successfully:
    /// * `Some(true)` is returned if the spot was not already marked and not an obstruction.
    /// * `Some(false)` is returned otherwise.
    ///
    /// Returns `None` if `(x, y)` is outside the grid.
    fn mark(&mut self, (x, y): Coord) -> Option<bool> {
        let row = self.field.get_mut(y)?;
        let spot = row.get_mut(x)?;
        match *spot {
            Space::Marked | Space::Obstruction => Some(false),
            _ => {
                *spot = Space::Marked;
                Some(true)
            }
        }
    }

    /// Marks the spot at `(x, y)` as not visited. The value returned by `Some` indicates whether the spot was unmarked successfully:
    /// * `Some(true)` is returned if the spot is not an obstruction.
    /// * `Some(false)` is returned otherwise.
    ///
    /// Returns `None` if `(x, y)` is outside the grid.
    fn unmark(&mut self, (x, y): Coord) -> Option<bool> {
        let row = self.field.get_mut(y)?;
        let spot = row.get_mut(x)?;
        match *spot {
            Space::Obstruction => Some(false),
            _ => {
                *spot = Space::Empty;
                Some(true)
            }
        }
    }
}

/// A pinned location for easy traversal inside a [Field].
struct Spot<'f> {
    field: &'f Field,
    coord: Coord,
}

type DirectionFn<'f> = fn(Spot<'f>) -> Option<Spot<'f>>;

impl<'f> std::fmt::Debug for Spot<'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.coord.0, self.coord.1)
    }
}

impl<'f> Clone for Spot<'f> {
    fn clone(&self) -> Self {
        Self {
            // This reference to `field` is explicitly cloned.
            field: self.field,
            coord: self.coord,
        }
    }
}

impl<'f> Spot<'f> {
    fn direction_fn(dir: Direction) -> DirectionFn<'f> {
        match dir {
            Direction::North => Self::north,
            Direction::East => Self::east,
            Direction::South => Self::south,
            Direction::West => Self::west,
        }
    }

    fn north(self) -> Option<Self> {
        let coord = (self.coord.0, self.coord.1 - 1);
        self.field.spot_at(coord)
    }

    fn east(self) -> Option<Self> {
        let coord = (self.coord.0 + 1, self.coord.1);
        self.field.spot_at(coord)
    }

    fn south(self) -> Option<Self> {
        let coord = (self.coord.0, self.coord.1 + 1);
        self.field.spot_at(coord)
    }

    fn west(self) -> Option<Self> {
        let coord = (self.coord.0 - 1, self.coord.1);
        self.field.spot_at(coord)
    }

    fn is_occupied(&self) -> bool {
        // SAFETY: This spot holds a valid reference to `field` as guaranteed by the directional methods.
        self.field.is_occupied_at(self.coord).unwrap()
    }
}

/// A cursor for easy traversal inside a [Field]. It leaves a trail around the `Field` as it traverses.
struct SpotMut<'f> {
    field: &'f mut Field,
    coord: Coord,
}

type DirectionFnMut<'f> = fn(SpotMut<'f>) -> Option<SpotMut<'f>>;

impl<'f> std::fmt::Debug for SpotMut<'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.coord.0, self.coord.1)
    }
}

impl<'f> SpotMut<'f> {
    fn direction_fn(dir: Direction) -> DirectionFnMut<'f> {
        match dir {
            Direction::North => Self::north,
            Direction::East => Self::east,
            Direction::South => Self::south,
            Direction::West => Self::west,
        }
    }

    fn north(self) -> Option<Self> {
        let coord = (self.coord.0, self.coord.1 - 1);
        self.field.spot_at_mut(coord)
    }

    fn east(self) -> Option<Self> {
        let coord = (self.coord.0 + 1, self.coord.1);
        self.field.spot_at_mut(coord)
    }

    fn south(self) -> Option<Self> {
        let coord = (self.coord.0, self.coord.1 + 1);
        self.field.spot_at_mut(coord)
    }

    fn west(self) -> Option<Self> {
        let coord = (self.coord.0 - 1, self.coord.1);
        self.field.spot_at_mut(coord)
    }

    fn is_occupied(&self) -> bool {
        // SAFETY: This spot is within the field, as guaranteed by [Field::spot_at_mut].
        self.field.is_occupied_at(self.coord).unwrap()
    }

    fn is_marked(&self) -> bool {
        // SAFETY: This spot is within the field, as guaranteed by [Field::spot_at_mut].
        self.field.is_marked_at(self.coord).unwrap()
    }

    fn mark(&mut self) -> bool {
        // SAFETY: This spot is within the field, as guaranteed by [Field::spot_at_mut].
        self.field.mark(self.coord).unwrap()
    }
}

/// Writes the `field` to a file with all marks. Overwrites the file at `path` if it exists.
#[allow(dead_code)]
fn write_field_to(field: &Field, path: &str) -> std::io::Result<()> {
    let byteiter = field
        .field
        .iter()
        .map(|row| row.iter().map(|space| char::from(*space) as u8));

    // The `+1` here is for the newlines at the end of each row.
    let bufsize = field.field.iter().map(|row| row.len() + 1).sum();
    let mut buf: Vec<u8> = Vec::with_capacity(bufsize);

    for row in byteiter {
        for space in row {
            buf.push(space);
        }
        buf.push(b'\n');
    }

    let mut file = std::fs::File::create(path)?;
    file.write_all(&buf)
}
