use fourthrail::*;

pub const MAP_WIDTH: i32 = 120;
pub const MAP_HEIGHT: i32 = 120;

// The map of a level
pub struct Map<T: Copy> {
    tiles  : [T ; (MAP_WIDTH * MAP_HEIGHT) as usize]
}

impl<T: Copy> Map<T> {
    pub fn new(t: T) -> Map<T> {
        Map {tiles: [t ; (MAP_WIDTH * MAP_HEIGHT) as usize]}
    }

    pub fn get_tile(&self, coord: Coord) -> T {
        let (r, c) = coord;
        self.tiles[(120 * r + c) as usize]
    }

    pub fn set_tile(&mut self, coord: Coord, t: T) {
        let (r, c) = coord;
        self.tiles[(120 * r + c) as usize] = t;
    }
}
