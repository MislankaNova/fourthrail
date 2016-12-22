extern crate pancurses as curses;

use fourthrail::*;

// A single map tile
pub enum Tile {
    Empty,
    Tile {
        name   : String,
        symbol : char,
        fore   : i16,
        back   : i16,
        pair   : i16,
        opaque : bool,
        solid  : bool
    }
}

impl Display for Tile {
    fn display(&self) -> Icon {
        if let &Tile::Tile {pair: p, symbol: c, ..} = self {
            (p, c)
        } else {
            (-1, ' ')
        }
    }
}

impl Named for Tile {
    fn nym(&self) -> &str {
        if let &Tile::Tile {name: ref n, ..} = self {
            &n
        } else {
            langue::VOID
        }
    }
}

pub struct TileBuilder {
    name   : String,
    symbol : char,
    fore   : i16,
    back   : i16,
    attr   : i16,
    pair   : i16,
    opaque : bool,
    solid  : bool
}

impl Tile {
    pub fn is_opaque(&self) -> bool {
        if let &Tile::Tile { opaque: o, ..} = self {
            o
        } else {
            false
        }
    }

    pub fn is_solid(&self) -> bool {
        if let &Tile::Tile { solid: s, .. } = self {
            s
        } else {
            true
        }
    }
}

impl TileBuilder {
    pub fn new() -> TileBuilder {
        TileBuilder {
            name: String::from("No Name"),
            symbol: 'x',
            fore: 0,
            back: 0,
            attr: 0,
            pair: 0,
            opaque: false,
            solid: false
        }
    }

    pub fn finalise(&self) -> Tile {
        Tile::Tile {
            name: self.name.clone(),
            symbol: self.symbol,
            fore: self.fore,
            back: self.back,
            pair: self.pair,
            opaque: self.opaque,
            solid: self.solid
        }
    }

    pub fn name(&mut self, name: String)
            -> &mut TileBuilder {
        self.name = name;
        self
    }

    pub fn symbol(&mut self, symbol: char)
            -> &mut TileBuilder {
        self.symbol = symbol;
        self
    }

    pub fn colour(&mut self,
                  pair: i16,
                  fore: i16,
                  back: i16)
            -> &mut TileBuilder {
        self.fore = fore;
        self.back = back;
        self.pair = pair;
        curses::init_pair(pair, fore, back);
        self
    }

    pub fn opaque(&mut self, opaque: bool)
            -> &mut TileBuilder {
        self.opaque = opaque;
        self
    }

    pub fn solid(&mut self, solid: bool)
            -> &mut TileBuilder {
        self.solid = solid;
        self
    }
}
