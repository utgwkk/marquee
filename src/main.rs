extern crate ncurses;

use ncurses::*;

fn get_terminal_size(win: WINDOW) -> (i32, i32) {
    let mut width = 0;
    let mut height = 0;
    getmaxyx(win, &mut height, &mut width);
    (width, height)
}

fn main() {
    let window = initscr();
    let (width, height) = get_terminal_size(window);
    addstr(format!("width = {}, height = {}", width, height).as_str());
    refresh();
    getch();
    endwin();
}
