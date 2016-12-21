// Type aliases for subte

pub type Coord = (i32, i32);
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
