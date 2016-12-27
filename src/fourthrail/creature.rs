use fourthrail::*;

// A creature in the world
pub struct Creature {
    name   : String,
    coord  : Coord,
    symbol : char,
    pair   : i16
}

impl Display for Creature {
    fn display(&self) -> Icon {
        (self.pair, self.symbol)
    }
}

impl Position for Creature {
    fn pos(&self) -> Coord {
        self.coord
    }
}

impl Named for Creature {
    fn nym(&self) -> &str {
        &self.name
    }
}

impl Agent for Creature {
    fn act(&self) -> Force {
        Force::Probe
    }
}

impl Move for Creature {
    fn get_coord(&self) -> Coord {
        self.coord
    }

    fn move_in(&mut self, d: Direction) {
        self.coord = next_coord(self.coord, d);
    }

    fn move_to(&mut self, c: Coord) {
        self.coord = c;
    }
}

pub struct CreatureBuilder {
    name   : String,
    coord  : Coord,
    symbol : char,
    pair   : i16
}

impl CreatureBuilder {
    pub fn new_player() -> Creature {
        Creature {
            name   : "You".to_string(),
            coord  : (2, 2),
            symbol : '@',
            pair   : 34
        }
    }
}
