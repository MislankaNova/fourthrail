// Traits used by subte

pub trait Agent {
    // Force is yet to be defined.
    //fn act(&self) -> Force;
}

pub trait Display {
    fn display(&self) -> (i16, char);
}

pub trait Position {
    fn pos(&self) -> (i32, i32);
}

pub trait Named {
    fn nym(&self) -> &str;
}
