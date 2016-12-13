// Graphic manager of subte

extern crate pancurses as curses;

use std::cmp::*;

use fourthrail::*;

/* */


const MAP_DISPLAY_WIDTH  : i32 = 60;
const MAP_DISPLAY_HEIGHT : i32 = 22;

const DISPLAY_MAP_NAME_COLOUR  : i16 = 21;
const DISPLAY_STAT_NAME_COLOUR : i16 = 31;
const DISPLAY_STAT_CAL_COLOUR  : i16 = 32;
const DISPLAY_STAT_CAP_COLOUR  : i16 = 33;
const DISPLAY_COHERENCY_COLOUR : i16 = 34;

/* */

pub fn init_display() {
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

pub fn display_tile(win: &curses::Window, t: &types::Tile) {
    if let &types::Tile::Tile {pair: p, symbol: s, ..} = t {
        win.color_set(p);
        win.addch(s);
    }
}

pub fn display_map(win: &curses::Window, map: &types::Map) {
    // This is a hack
    // You must
    win.border('x', 'x', 'x', 'x', 'x', 'x', 'x', 'x');
    for r in 0..min(map.height, MAP_DISPLAY_HEIGHT) {
        win.mv(r, 0);
        for c in 0..min(map.width, MAP_DISPLAY_WIDTH) {
            display_tile(win, &map.tiles[(120 * r + c) as usize]);
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

pub fn display_stats(win    : &curses::Window,
                     coh    : &i32,
                     //status : &types::Status
                     ) {
    let s_coh = coh.to_string();
    let start = MAP_DISPLAY_WIDTH + 1;
    win.attron(curses::A_BOLD);
    win.color_set(DISPLAY_COHERENCY_COLOUR);
    win.mvaddstr(1, start, langue::COHERENCY);
    win.mvaddstr(2, start, &s_coh);
    win.mvaddstr(4, start, langue::STAMINA);
}
