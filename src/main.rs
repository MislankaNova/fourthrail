// Entry point for subte

extern crate pancurses as curses;

extern crate fourthrail;

use fourthrail::fourthrail::*;

/* */

fn main () {
    // Initialise curses first.
    let window = curses::initscr();
    let (height, width) = window.get_max_yx();

    // Then check terminal size.
    match (height, width) {
        (h, w) if h < graphic::INNER_HEIGHT || w < graphic::INNER_WIDTH => {
            curses::endwin();
            println!("\nTerminal too small.\n");
            return;
        }
        (h, w) if h > graphic::INNER_HEIGHT => {
            window.mvprintw(
                (h - graphic::INNER_HEIGHT - 1) / 2,
                (w - graphic::INNER_WIDTH) / 2,
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
        graphic::INNER_HEIGHT,
        graphic::INNER_WIDTH,
        (height - graphic::INNER_HEIGHT + 1) / 2,
        (width - graphic::INNER_WIDTH) / 2
        );
    let t1 = TileBuilder::new()
        .name(String::from("TileTile"))
        .symbol('.')
        .colour(1, curses::COLOR_WHITE, curses::COLOR_BLACK)
        .opaque(false)
        .solid(false)
        .finalise();
    let t2 = TileBuilder::new()
        .name(String::from("BadTile"))
        .symbol('#')
        .colour(2, curses::COLOR_WHITE, curses::COLOR_BLACK)
        .opaque(true)
        .solid(true)
        .finalise();
    let r = Resource { tile_defs: vec![t1, t2] };
    let mut fourthrail = main::Fourthrail::initialise(neww, &r);

    curses::flash();
    window.keypad(true);
    window.getch();

    fourthrail.update_graphic();

    // Then enter the main loop
    loop {
        fourthrail.update_graphic();

        match window.getch() {
            Some(curses::Input::Character('q')) => {
                let exitw = curses::newwin(4, 40, 4, 4);
                exitw.mv(1,1);
                exitw.printw(langue::EXIT_REALLY);
                exitw.mv(2,1);
                exitw.printw(langue::YN);
                exitw.refresh();
                match window.getch() {
                      Some(curses::Input::Character('y'))
                    | Some(curses::Input::Character('Y')) => break,
                    _                                     => continue
                }
            }

            Some(k) => fourthrail.turn(k),

            _ => continue
        }
    }

    // We are done.
    curses::endwin();
}
