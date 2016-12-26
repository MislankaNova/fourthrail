// Type aliases for subte

use std::collections::*;

pub type Coord = (i32, i32);
pub type Coords = BTreeSet<Coord>;
pub type Icon = (i16, char);

// Unit enums

pub enum Direction {
    W,
    NW,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    Up,
    Down
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Visibility {
    Unseen,
    Seen,
    Visible
}
