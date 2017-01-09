use fourthrail::*;

// An item in the world
pub struct Item {
    name   : String,
    coord  : Coord,
    symbol : char,
    pair   : i16
}

impl Display for Item {
    fn display(&self) -> Icon {
        (self.pair, self.symbol)
    }
}

impl Named for Item {
    fn nym(&self) -> &str {
        &self.name
    }
}

impl Agent for Item {
    fn act(&self) -> Force {
        Force::Probe
    }
}

impl Position for Item {
    fn pos(&self) -> Coord {
        self.coord
    }

    fn move_in(&mut self, d: Direction) {
        self.coord = next_coord(self.coord, d);
    }

    fn move_to(&mut self, c: Coord) {
        self.coord = c;
    }
}

pub struct ItemBuilder {
    name   : String,
    coord  : Coord,
    symbol : char,
    pair   : i16
}

impl ItemBuilder {
    pub fn new_sicp() -> Item {
        Item {
            name   : "SICP".to_string(),
            coord  : (4, 4),
            symbol : 'λ',
            pair   : 34
        }
    }
}
