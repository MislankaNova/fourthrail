// Traits used by subte

use fourthrail::*;

pub trait Agent {
    fn act(&self) -> Force;
}

pub trait Position {
    fn pos(&self) -> Coord;
    fn move_in(&mut self, Direction);
    fn move_to(&mut self, Coord);
}

pub trait Display {
    fn display(&self) -> Icon;
}

pub trait Named {
    fn nym(&self) -> &str;
}
