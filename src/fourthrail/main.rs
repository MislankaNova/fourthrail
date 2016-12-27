// Main logic for subte

extern crate pancurses as curses;

use std::cmp::*;

use fourthrail::*;

/* */

pub struct Fourthrail<'trip> {
    window    : curses::Window,
    resource  : &'trip Resource,

    coherency : i32,
    map       : Map<&'trip Tile>,
    map_memory: Map<Visibility>,

    player    : Creature
}

impl<'trip> Fourthrail<'trip> {
    pub fn initialise(win : curses::Window, r : &'trip Resource)
            -> Fourthrail<'trip> {
        let s = String::from("North Acton Station");
        let mut map = Map::new(&(r.tile_defs[0]));
        for row in 3..8 {
            for col in 4..6 {
                map.set_tile((row * 3, col * 4), &(r.tile_defs[1]));
                map.set_tile((row * 3, col * 4 + 1), &(r.tile_defs[1]));
            }
        }
        Fourthrail {
            window    : win,
            resource  : r,

            coherency : -10,
            map       : map,
            map_memory: Map::new(Visibility::Unseen),

            player    : CreatureBuilder::new_player()
        }
    }

    pub fn turn(&mut self, key : curses::Input) {
        if let Some(d) = input_to_direction(key) {
            self.player.move_in(d);
        }

        self.update_memory();
        self.update_visibility();
    }

    pub fn update_memory(&mut self) {
        for r in 0..MAP_HEIGHT {
            for c in 0..MAP_WIDTH {
                if self.map_memory.get_tile((r, c)) == Visibility::Visible {
                    self.map_memory.set_tile((r, c), Visibility::Seen);
                }
            }
        }
    }

    pub fn update_visibility(&mut self) {
        let current = self.player.get_coord();
        let coords = shadow::circle(&current, 6);
        for c in coords {
            let line = shadow::line(&current, &c);
            for d in line {
                self.map_memory.set_tile(d, Visibility::Visible);
                if self.map.get_tile(d).is_opaque() {
                    break;
                }
            }
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
        graphic::put_map(&self.window, start, &self.map, &self.map_memory);
        graphic::put_stats(&self.window, &self.coherency);
        graphic::put_creature(&self.window, start, &self.player);
        self.window.refresh();
    }
}
