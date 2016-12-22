use fourthrail::*;

// The map of a level
pub struct Map<'trip> {
    pub height : i32,
    pub width  : i32,
    pub level  : i32,
    pub name   : String,
    pub tiles  : [&'trip Tile ; 14400]
}
