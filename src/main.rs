extern crate pancurses as curses;

/* */

fn check_terminal_size (win : &curses::Window) -> bool {
    match win.get_max_yx() {
        (y, x) if x >= 80 && y >= 24 => true,
        _                            => false
    }
}


fn main () {
    // Initialise curses first.
    let window = curses::initscr();

    // Then check terminal size.
    if !check_terminal_size(&window) {
        curses::endwin();
        println!("\nTerminal too small.\n");
        return;
    }

    // Then set the behaviour of the cursor.
    curses::noecho();
    curses::curs_set(0);

    // Then enter the main loop
    window.mvprintw(10, 10, "Multitail");
    window.getch();
    curses::flash();
    window.getch();

    // We are done.
    curses::endwin();
}
