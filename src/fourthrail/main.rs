// Main logic for subte

extern crate pancurses as curses;

use std::cmp::*;

use fourthrail::*;

/* */

pub struct Fourthrail<'trip> {
    window    : curses::Window,
    resource  : &'trip Resource,

    coherency : i32,
    map       : Map<'trip>,

    player    : Creature
}

impl<'trip> Fourthrail<'trip> {
    pub fn initialise(win : curses::Window, r : &'trip Resource)
            -> Fourthrail<'trip> {
        let s = String::from("North Acton Station");
        Fourthrail {
            window    : win,
            resource  : r,

            coherency : -10,
            map       : Map {
                height : 120,
                width  : 120,
                level  : 1,
                name   : s,
                tiles  : [&(r.tile_defs[0]); 14400]
            },

            player    : CreatureBuilder::new_player()
        }
    }

    pub fn turn(&mut self, key : curses::Input) {
        match key {
            curses::Input::Character('h')
            | curses::Input::KeyLeft => self.player.move_in(Direction::W),

            curses::Input::Character('j')
            | curses::Input::KeyDown => self.player.move_in(Direction::S),

            curses::Input::Character('k')
            | curses::Input::KeyUp => self.player.move_in(Direction::N),

            curses::Input::Character('l')
            | curses::Input::KeyRight => self.player.move_in(Direction::E),

            curses::Input::Character('u') => self.player.move_in(Direction::NW),
            curses::Input::Character('i') => self.player.move_in(Direction::NE),
            curses::Input::Character('n') => self.player.move_in(Direction::SW),
            curses::Input::Character('m') => self.player.move_in(Direction::SE),

            _ => ()
        }
    }

    pub fn update_graphic(&self) {
        let (mut pr, mut pc) = self.player.get_coord();
        pr -= (graphic::MAP_DISPLAY_HEIGHT - graphic::MAP_DISPLAY_STEP) / 2;
        pc -= (graphic::MAP_DISPLAY_WIDTH - graphic::MAP_DISPLAY_STEP) / 2;
        pr = min(max(pr, 0), 120 - graphic::MAP_DISPLAY_WIDTH);
        pc = min(max(pc, 0), 120 - graphic::MAP_DISPLAY_HEIGHT);
        pr = graphic::MAP_DISPLAY_STEP * (pr / graphic::MAP_DISPLAY_STEP);
        pc = graphic::MAP_DISPLAY_STEP * (pc / graphic::MAP_DISPLAY_STEP);
        let start = (pr, pc);
        
        // This is a hack
        // Without this the map cannot update properly
        self.window.border(' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ');
        graphic::put_map(&self.window, start, &self.map);
        graphic::put_stats(&self.window, &self.coherency);
        graphic::put_creature(&self.window, start, &self.player);
        self.window.refresh();
    }
}
