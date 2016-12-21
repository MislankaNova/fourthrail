// Main logic for subte

extern crate pancurses as curses;

use fourthrail::*;

use fourthrail::traits::Move;

/* */

pub struct Fourthrail<'trip> {
    window    : curses::Window,
    resource  : &'trip types::Resource,

    coherency : i32,
    map       : types::Map<'trip>,

    player    : types::Creature
}

impl<'trip> Fourthrail<'trip> {
    pub fn initialise(win : curses::Window, r : &'trip types::Resource)
            -> Fourthrail<'trip> {
        let s = String::from("North Acton Station");
        Fourthrail {
            window    : win,
            resource  : r,

            coherency : -10,
            map       : types::Map {
                height : 120,
                width  : 120,
                level  : 1,
                name   : s,
                tiles  : [&(r.tile_defs[0]); 14400]
            },

            player    : types::CreatureBuilder::new_player()
        }
    }

    pub fn turn(&mut self, key : curses::Input) {
        self.coherency -= 1;
    }

    pub fn update_graphic(&self) {
        // This is a hack
        // Without this the map cannot update properly
        self.window.border(' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ');
        graphic::put_map(&self.window, (0, 0), &self.map);
        graphic::put_stats(&self.window, &self.coherency);
        graphic::put_creature(&self.window, (0, 0), &self.player);
        self.window.refresh();
    }
}
