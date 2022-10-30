use std::io::{stdout, Write};

use crossterm::{
    cursor, execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

use crate::{
    ball::Ball,
    player::{self, Player},
};

pub(crate) const HEIGHT: usize = 20;
pub(crate) const WIDTH: usize = 50;

struct Drawer {}

pub fn init() {
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
}

pub fn dispose(clear: bool) {
    let mut stdout = stdout();
    disable_raw_mode().unwrap();
    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
    if clear {
        execute!(stdout, Clear(ClearType::All));
    }
}

pub fn clear() {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All)).unwrap();
}

pub fn draw_basic(player1: &Player, player2: &Player, ball: &Ball) {
    let mut stdout = stdout();

    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
    println!("{}", "-".repeat(WIDTH + 2));
    for n in 0..HEIGHT {
        let mut middle = " ".repeat(WIDTH);

        // let showp1 = is_here(n, player1.y, player1.height);
        // let showp2 = is_here(n, player2.y, player2.height);

        // if (showp1 && showp2) {
        //     middle = format!("|{}|", " ".repeat(WIDTH - 2));
        // } else if (showp1) {
        //     middle = format!("|{}", " ".repeat(WIDTH - 1));
        // } else if (showp2) {
        //     middle = format!("{}|", " ".repeat(WIDTH - 1));
        // }

        // if ball.y == n {
        //     middle.replace_range(ball.x..ball.x + 1, "*");
        // }

        let string: String = format!("{}{}{}", "|", middle, "|",);

        println!("{}", string);
    }
    println!("{}", "-".repeat(WIDTH + 2));

    for n in player1.y..player1.y + player1.height {
        execute!(stdout, cursor::MoveTo(1, n as u16 + 1)).unwrap();
        print!("|");
        stdout.flush();
    }

    for n in player2.y..player2.y + player2.height {
        execute!(stdout, cursor::MoveTo(WIDTH as u16 - 1, n as u16 + 1)).unwrap();
        print!("|");
        stdout.flush();
    }

    execute!(stdout, cursor::MoveTo(ball.x as u16, ball.y as u16 + 1)).unwrap();
    print!("*");
    stdout.flush();

    execute!(stdout, cursor::MoveTo(0, HEIGHT as u16 + 2)).unwrap();
}

// fn is_here(pos: usize, y: usize, height: usize) -> bool {
//     let range: Vec<usize> = (y..y + height).collect();

//     return range.contains(&pos);
// }
