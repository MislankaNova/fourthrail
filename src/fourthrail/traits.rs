// Traits used by subte

use fourthrail::*;

use fourthrail::typedefs::*;

pub trait Agent {
    fn act(&self) -> types::Force;
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
