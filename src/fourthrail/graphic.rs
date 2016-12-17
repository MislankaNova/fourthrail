// Graphic manager of subte

extern crate pancurses as curses;

use std::cmp::*;

use fourthrail::*;

use fourthrail::types::Display;

/* */


const MAP_DISPLAY_WIDTH  : i32 = 60;
const MAP_DISPLAY_HEIGHT : i32 = 22;

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

pub fn put_tile(win: &curses::Window, t: &types::Tile) {
    let (p, c) = t.display();
    win.color_set(p);
    win.addch(c);
}

pub fn put_map(win: &curses::Window, map: &types::Map, start: (i32, i32)) {
    let (sr, sc) = start;
    for r in 0..min(map.height - sr, MAP_DISPLAY_HEIGHT) {
        win.mv(r, 0);
        for c in 0..min(map.width - sc, MAP_DISPLAY_WIDTH) {
            put_tile(win, &map.tiles[(120 * (r + sr) + c + sc) as usize]);
        }
    }
    /* Then display level detail */
    win.color_set(DISPLAY_MAP_NAME_COLOUR);
    win.attron(curses::A_BOLD);
    win.mv(MAP_DISPLAY_HEIGHT, 0);
    win.addstr("> ");
    win.addstr(&map.name);
    win.addstr(" <");
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
