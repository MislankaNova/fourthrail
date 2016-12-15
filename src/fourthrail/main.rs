// Main logic for subte

extern crate pancurses as curses;

use fourthrail::*;

/* */

pub struct Fourthrail {
    window    : curses::Window,
    coherency : i32,
    map       : types::Map
}

impl Fourthrail {
    pub fn initialise(win : curses::Window) -> Fourthrail {
        let s = String::from("North Acton Station");
        let t = types::TileBuilder::new()
            .name("TileTile")
            .symbol('.')
            .colour(1, curses::COLOR_BLACK, curses::COLOR_WHITE)
            .opaque(false)
            .solid(false)
            .finalise();
        Fourthrail {
            window    : win,
            coherency : -10,
            map       : types::Map {
                height : 120,
                width  : 120,
                level  : 1,
                name   : s,
                tiles  : [t; 14400]
            }
        }
    }

    pub fn turn(&mut self, key : curses::Input) {
        self.coherency -= 1;
    }

    pub fn update_graphic(&self) {
        // This is a hack
        // Without this the map cannot update properly
        self.window.border(' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ');
        graphic::put_map(&self.window, &self.map);
        graphic::put_stats(&self.window, &self.coherency);
        self.window.refresh();
    }
}
