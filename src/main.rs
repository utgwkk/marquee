extern crate ncurses;

use std::cmp::{min, max};
use std::{thread, time};
use ncurses::*;

fn get_terminal_size(win: WINDOW) -> (i32, i32) {
    let mut width = 0;
    let mut height = 0;
    getmaxyx(win, &mut height, &mut width);
    (width, height)
}

fn marquee_line(width: i32, line: &str, count: i32) {
    let line_length = line.len() as i32;
    let count = count % (width + line_length);
    let leftermost_offset = max(0, width - count);
    let visible_length =
        if leftermost_offset == 0 {
            line_length - (count - width)
        } else {
            min(line_length, width - leftermost_offset)
        } as usize;
    let sliced_line =
        if leftermost_offset == 0 {
            let slice_len = (count - width) as usize;
            &line[slice_len..]
        } else {
            &line[..visible_length]
        };
    let space = " ".repeat(leftermost_offset as usize);
    let showtext = format!("{}{}", space, sliced_line);
    addstr(showtext.as_str());
}

fn marquee(win: WINDOW, text: &str) {
    let (width, height) = get_terminal_size(win);
    let mut counter = 0;
    loop {
        marquee_line(width, text, counter);
        refresh();
        thread::sleep(time::Duration::from_millis(100));
        clear();
        refresh();
        counter += 1;
    }
}

fn main() {
    let window = initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    marquee(window, "Hello, world!");
    endwin();
}
