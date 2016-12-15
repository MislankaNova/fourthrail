// Types used by subte

extern crate std;

extern crate pancurses as curses;

use std::*;
use fourthrail::*;

/* Type names */

type Display = (i16, char);

/* Traits */

pub trait CanDisplay {
    fn display(&self) -> Display;
}

/* Structs & Enums */

// A single map tile
#[derive (Clone, Copy)]
pub enum Tile {
    Empty,
    Tile {
        name   : &'static str,
        symbol : char,
        fore   : i16,
        back   : i16,
        pair   : i16,
        opaque : bool,
        solid  : bool
    }
}

impl CanDisplay for Tile {
    fn display(&self) -> Display {
        if let &Tile::Tile {pair: p, symbol: c, ..} = self {
            (p, c)
        } else {
            (-1, ' ')
        }
    }
}

pub struct TileBuilder {
    name   : &'static str,
    symbol : char,
    fore   : i16,
    back   : i16,
    attr   : i16,
    pair   : i16,
    opaque : bool,
    solid  : bool
}

impl Tile {
    pub fn isOpaque(&self) -> bool {
        if let &Tile::Tile(opaque: o, ..) = self {
            o
        } else {
            false
        }
    }

    pub fn isSolid(&self) -> bool {
        if let &Tile::Tile(solid: s, ..) = self {
            s
        } else {
            true
        }
    }
}

impl TileBuilder {
    pub fn new() -> TileBuilder {
        TileBuilder {
            name: "No Name",
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
            name: self.name,
            symbol: self.symbol,
            fore: self.fore,
            back: self.back,
            pair: self.pair,
            opaque: self.opaque,
            solid: self.solid
        }
    }

    pub fn name(&mut self, name: &'static str)
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

pub struct Map {
    pub height : i32,
    pub width  : i32,
    pub level  : i32,
    pub name   : String,
    pub tiles  : [types::Tile ; 14400]
}

/* */

// The status of an agent
pub struct Status {
    stamina     : i32,
    stamina_max : i32,
    strength    : i32,
    dexterity   : i32,
    status      : collections::HashMap<&'static str, i32>
}
