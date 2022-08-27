//! A library for manipulating 2d grids
//!
//! Grids are given as vecs of rows which are vecs of cells

#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::enum_glob_use,
    clippy::many_single_char_names,
    clippy::must_use_candidate
)]
#![forbid(missing_docs)]

/// A type of topology
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Topology {
    /// A bounded grid, with no wrap-around
    Bounded,

    /// A grid that wraps around, preserving the axis not moved in. e.g. Pacman
    Torus,
}

use Topology::*;

/// One of the four cardinal directions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    /// North.
    North,

    /// South.
    South,

    /// East.
    East,

    /// West.
    West,
}

use Direction::*;

/// Neighborhoods around a point. They do not contain the point itself
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Neighborhood {
    /// The neighborhood consisting of the points directly North, South, East, and West of a point.
    Orthogonal,

    /// The neighborhood consisting of the points directly diagonal to a point.
    Diagonal,

    /// The neighborhood consisting of the square directly around the point.
    Square,
}

use Neighborhood::*;

/// Get the adjacent point to a point in a given direction
pub fn adjacent_cell(
    t: Topology,
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    d: Direction,
) -> Option<(usize, usize)> {
    match t {
        Bounded => match d {
            North => Some((x, y.checked_sub(1)?)),
            South => {
                if y + 1 < height {
                    Some((x, y + 1))
                } else {
                    None
                }
            }
            East => {
                if x + 1 < width {
                    Some((x + 1, y))
                } else {
                    None
                }
            }
            West => Some((x.checked_sub(1)?, y)),
        },
        Torus => match d {
            North => Some((x, y.checked_sub(1).unwrap_or(height - 1))),
            South => Some((x, (y + 1) % width)),
            East => Some(((x + 1) % width, y)),
            West => Some((x.checked_sub(1).unwrap_or(width - 1), y)),
        },
    }
}

/// Is a given point on an edge of a grid
pub fn is_edge(t: Topology, width: usize, height: usize, x: usize, y: usize) -> bool {
    t == Bounded && (x == 0 || x + 1 == width || y == 0 || y + 1 == height)
}

/// Is a given point a corner of a grid
pub fn is_corner(t: Topology, width: usize, height: usize, x: usize, y: usize) -> bool {
    t == Bounded && (x == 0 || x + 1 == width) && (y == 0 || y + 1 == height)
}

/// Returns an iterator over the points of a grid
pub fn points(width: usize, height: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..width).flat_map(move |x| (0..height).map(move |y| (x, y)))
}

/// Returns an iterator over the points in a neighborhood around a point
pub fn neighborhood(
    t: Topology,
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    n: Neighborhood,
) -> impl Iterator<Item = (usize, usize)> {
    match n {
        Orthogonal => vec![
            adjacent_cell(t, width, height, x, y, North),
            adjacent_cell(t, width, height, x, y, South),
            adjacent_cell(t, width, height, x, y, East),
            adjacent_cell(t, width, height, x, y, West),
        ],
        Diagonal => {
            let n = adjacent_cell(t, width, height, x, y, North);
            let s = adjacent_cell(t, width, height, x, y, South);
            vec![
                n.and_then(|(x, y)| adjacent_cell(t, width, height, x, y, East)),
                s.and_then(|(x, y)| adjacent_cell(t, width, height, x, y, East)),
                n.and_then(|(x, y)| adjacent_cell(t, width, height, x, y, West)),
                s.and_then(|(x, y)| adjacent_cell(t, width, height, x, y, West)),
            ]
        }
        Square => {
            let n = adjacent_cell(t, width, height, x, y, North);
            let s = adjacent_cell(t, width, height, x, y, South);
            vec![
                adjacent_cell(t, width, height, x, y, North),
                adjacent_cell(t, width, height, x, y, South),
                adjacent_cell(t, width, height, x, y, East),
                adjacent_cell(t, width, height, x, y, West),
                n.and_then(|(x, y)| adjacent_cell(t, width, height, x, y, East)),
                s.and_then(|(x, y)| adjacent_cell(t, width, height, x, y, East)),
                n.and_then(|(x, y)| adjacent_cell(t, width, height, x, y, West)),
                s.and_then(|(x, y)| adjacent_cell(t, width, height, x, y, West)),
            ]
        }
    }
    .into_iter()
    .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacent_bounded() {
        assert_eq!(adjacent_cell(Bounded, 3, 3, 1, 0, North), None);
        assert_eq!(adjacent_cell(Bounded, 3, 3, 1, 1, North), Some((1, 0)));

        assert_eq!(adjacent_cell(Bounded, 3, 3, 2, 2, South), None);
        assert_eq!(adjacent_cell(Bounded, 3, 3, 0, 0, South), Some((0, 1)));

        assert_eq!(adjacent_cell(Bounded, 3, 3, 2, 2, East), None);
        assert_eq!(adjacent_cell(Bounded, 3, 3, 1, 1, East), Some((2, 1)));

        assert_eq!(adjacent_cell(Bounded, 3, 3, 0, 0, West), None);
        assert_eq!(adjacent_cell(Bounded, 3, 3, 1, 1, West), Some((0, 1)));
    }

    #[test]
    fn adjacent_torus() {
        assert_eq!(adjacent_cell(Torus, 3, 3, 1, 0, North), Some((1, 2)));
        assert_eq!(adjacent_cell(Torus, 3, 3, 1, 1, North), Some((1, 0)));

        assert_eq!(adjacent_cell(Torus, 3, 3, 2, 2, South), Some((2, 0)));
        assert_eq!(adjacent_cell(Torus, 3, 3, 0, 0, South), Some((0, 1)));

        assert_eq!(adjacent_cell(Torus, 3, 3, 2, 2, East), Some((0, 2)));
        assert_eq!(adjacent_cell(Torus, 3, 3, 1, 1, East), Some((2, 1)));

        assert_eq!(adjacent_cell(Torus, 3, 3, 0, 0, West), Some((2, 0)));
        assert_eq!(adjacent_cell(Torus, 3, 3, 1, 1, West), Some((0, 1)));
    }

    #[test]
    fn edge() {
        assert!(is_edge(Bounded, 3, 3, 1, 0));
        assert!(is_edge(Bounded, 3, 3, 0, 1));
        assert!(is_edge(Bounded, 3, 3, 1, 2));
        assert!(is_edge(Bounded, 3, 3, 2, 1));

        assert!(!is_edge(Bounded, 3, 3, 1, 1));

        assert!(!is_edge(Torus, 3, 3, 2, 1));
    }

    #[test]
    fn pts() {
        assert_eq!(points(3, 3).count(), 9);
    }

    #[test]
    fn neighborino() {
        assert_eq!(
            neighborhood(Torus, 5, 5, 0, 0, Square).collect::<Vec<(usize, usize)>>(),
            [
                (0, 4),
                (0, 1),
                (1, 0),
                (4, 0),
                (1, 4),
                (1, 1),
                (4, 4),
                (4, 1)
            ],
        );
        assert_eq!(
            neighborhood(Bounded, 5, 5, 0, 0, Square).collect::<Vec<(usize, usize)>>(),
            [(0, 1), (1, 0), (1, 1)],
        );
    }
}
