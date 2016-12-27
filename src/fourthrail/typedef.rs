// Type aliases for subte

extern crate pancurses as curses;

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

// Helper with these types

pub fn input_to_direction(key: curses::Input) -> Option<Direction> {
    match key {
        curses::Input::Character('h')
        | curses::Input::KeyLeft => Some(Direction::W),

        curses::Input::Character('j')
        | curses::Input::KeyDown => Some(Direction::S),

        curses::Input::Character('k')
        | curses::Input::KeyUp => Some(Direction::N),

        curses::Input::Character('l')
        | curses::Input::KeyRight => Some(Direction::E),

        curses::Input::Character('u') => Some(Direction::NW),
        curses::Input::Character('i') => Some(Direction::NE),
        curses::Input::Character('n') => Some(Direction::SW),
        curses::Input::Character('m') => Some(Direction::SE),

        _ => None
    }
}

pub fn next_coord(coord: Coord, d: Direction) -> Coord {
    let (r, c) = coord;
    match d {
        Direction::W  => (r    , c - 1),
        Direction::NW => (r - 1, c - 1),
        Direction::N  => (r - 1, c    ),
        Direction::NE => (r - 1, c + 1),
        Direction::E  => (r    , c + 1),
        Direction::SE => (r + 1, c + 1),
        Direction::S  => (r + 1, c    ),
        Direction::SW => (r + 1, c - 1),
        _ => (r, c)
    }
}
