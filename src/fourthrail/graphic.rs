// Graphic manager of subte

extern crate pancurses as curses;

use std::cmp::*;

use fourthrail::*;

/* */


pub const MAP_DISPLAY_WIDTH  : i32 = 60;
pub const MAP_DISPLAY_HEIGHT : i32 = 22;
pub const MAP_DISPLAY_STEP   : i32 = 8 ;

const DISPLAY_NONE             : i16 = 0;
const DISPLAY_MAP_NAME_COLOUR  : i16 = 21;
const DISPLAY_STAT_NAME_COLOUR : i16 = 31;
const DISPLAY_STAT_CAL_COLOUR  : i16 = 32;
const DISPLAY_STAT_CAP_COLOUR  : i16 = 33;
const DISPLAY_COHERENCY_COLOUR : i16 = 34;

/* */

pub fn init_display() {
    curses::init_pair(
        DISPLAY_NONE,
        -1,
        -1
        );
    curses::init_pair(
        DISPLAY_MAP_NAME_COLOUR,
        curses::COLOR_YELLOW,
        -1
        );
    curses::init_pair(
        DISPLAY_COHERENCY_COLOUR,
        curses::COLOR_CYAN,
        -1
        );
}

pub fn put_tile(win: &curses::Window, t: &Tile) {
    let (p, c) = t.display();
    win.color_set(p);
    win.addch(c);
}

pub fn put_creature(win: &curses::Window, start: Coord, cr: &Creature) {
    let (sr, sc) = start;
    let (p, s) = cr.display();
    let (y, x) = cr.pos();
    let r = y - sr;
    let c = x - sc;
    if r >= MAP_DISPLAY_HEIGHT
        || r < 0
        || c >= MAP_DISPLAY_WIDTH
        || c < 0 {
            return;
    }
    win.mv(r, c);
    win.color_set(p);
    win.addch(s);
}

pub fn put_map(win: &curses::Window, start: Coord, map: &Map<&Tile>) {
    let (sr, sc) = start;
    for r in 0..min(MAP_HEIGHT - sr, MAP_DISPLAY_HEIGHT) {
        win.mv(r, 0);
        for c in 0..min(MAP_WIDTH - sc, MAP_DISPLAY_WIDTH) {
            put_tile(win, map.get_tile((r + sr, c + sc)));
        }
    }
}

pub fn put_stats(win: &curses::Window, coh: &i32) {
    let s_coh = coh.to_string();
    let start = MAP_DISPLAY_WIDTH + 1;
    win.attron(curses::A_BOLD);
    win.color_set(DISPLAY_COHERENCY_COLOUR);
    win.mvaddstr(1, start, langue::COHERENCY);
    win.mvaddstr(2, start, &s_coh);
    win.mvaddstr(4, start, langue::STAMINA);
}
