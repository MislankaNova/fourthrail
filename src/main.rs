// Entry point for subte

extern crate pancurses as curses;

extern crate fourthrail;

use fourthrail::fourthrail::*;

/* */

const INNER_HEIGHT : i32 = 24;
const INNER_WIDTH  : i32 = 80;

/* */

fn main () {
    // Initialise curses first.
    let window = curses::initscr();
    let (height, width) = window.get_max_yx();

    // Then check terminal size.
    match (height, width) {
        (h, w) if h < INNER_HEIGHT || w < INNER_WIDTH => {
            curses::endwin();
            println!("\nTerminal too small.\n");
            return;
        }
        (h, w) if h > INNER_HEIGHT => {
            window.mvprintw(
                (h - INNER_HEIGHT - 1) / 2,
                (w - INNER_WIDTH) / 2,
                "> Subte <"
                );
        }
        _ => { window.mvaddch(0, 0, ' '); }
    }

    // Then set the behaviour of the cursor.
    curses::noecho();
    curses::curs_set(0);
    curses::use_default_colors();
    curses::start_color();

    graphic::init_display();

    let neww = curses::newwin(
        INNER_HEIGHT,
        INNER_WIDTH,
        (height - INNER_HEIGHT + 1) / 2,
        (width - INNER_WIDTH) / 2
        );
    let mut fourthrail = main::Fourthrail::initialise(neww);

    curses::flash();
    window.getch();

    fourthrail.display();

    // Then enter the main loop
    loop {
        fourthrail.display();

        match (window.getch()) {
            Some(curses::Input::Character('q')) => {
                let exitw = curses::newwin(4, 40, 4, 4);
                exitw.mv(1,1);
                exitw.printw(langue::EXIT_REALLY);
                exitw.mv(2,1);
                exitw.printw(langue::YN);
                exitw.refresh();
                match (window.getch()) {
                    Some(curses::Input::Character('y')) => break,
                    _                                   => continue
                }
            }

            Some(k) => fourthrail.turn(k),

            _ => continue
        }
    }

    // We are done.
    curses::endwin();
}
