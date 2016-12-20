// Traits used by subte

use fourthrail::typedefs::*;

pub trait Agent {
    // Force is yet to be defined.
    //fn act(&self) -> Force;
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
