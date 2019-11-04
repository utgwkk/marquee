extern crate ncurses;

use std::{thread, time};
use ncurses::*;

fn get_terminal_size(win: WINDOW) -> (i32, i32) {
    let mut width = 0;
    let mut height = 0;
    getmaxyx(win, &mut height, &mut width);
    (width, height)
}

fn marquee(text: &str) {
    let mut counter = 0;
    loop {
        let showtext = format!("{} {}", text, counter);
        addstr(showtext.as_str());
        refresh();
        thread::sleep(time::Duration::from_millis(100));
        clear();
        refresh();
        counter += 1;
    }
}

fn main() {
    let window = initscr();
    marquee("Hello, world!");
    endwin();
}
