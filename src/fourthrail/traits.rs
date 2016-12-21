// Traits used by subte

use fourthrail::*;

use fourthrail::typedefs::*;

pub trait Agent {
    fn act(&self) -> types::Force;
}

pub trait Move {
    fn move_in(&mut self, typedefs::Direction);
    fn move_to(&mut self, Coord);
}

pub trait Display {
    fn display(&self) -> Icon;
}

pub trait Position {
    fn pos(&self) -> Coord;
}

pub trait Named {
    fn nym(&self) -> &str;
}
