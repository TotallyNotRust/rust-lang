mod ball;
mod graphic;
mod input;
use core::panic;
use std::time::Duration;

use chrono;
use crossterm::event::{poll, read, Event, KeyCode};
use graphic::{draw_basic, init};
use timer;

use crate::graphic::dispose;
mod player;

const FRAME_PER_MOVE: i32 = 32;

fn main() {
    init();

    let timer = timer::Timer::new();

    let mut player1 = player::Player {
        y: 0,
        height: 3,
        gameheight: graphic::HEIGHT,
    };
    let mut player2 = player::Player {
        y: 3,
        height: 3,
        gameheight: graphic::HEIGHT,
    };

    let mut ball = ball::Ball {
        x: graphic::WIDTH / 2,
        y: graphic::HEIGHT / 2,
        velx: 1,
        vely: 1,
    };

    let x = timer.schedule_repeating(chrono::Duration::milliseconds(32), move || {
        // println!("{},{}", ball.x, ball.y);
        draw_basic(&player1, &player2, &ball);

        let mut frame = 0;

        loop {
            match poll(Duration::from_millis(1)) {
                Ok(n) => {
                    if !n {
                        break;
                    }
                    match read() {
                        Ok(n) => {
                            // panic!("{:?} -- {:?}", n, Event::Key(KeyCode::Esc.into()));
                            println!("{:?}", n);
                            if n == Event::Key(KeyCode::Esc.into()) {
                                dispose(true);
                                panic!("Exited");
                            }
                            if n == Event::Key(KeyCode::PageUp.into()) {
                                graphic::clear();
                            }

                            input::handle_input(n, &mut player1, &mut player2);
                        }
                        Err(_) => break,
                    };
                }
                Err(_) => {}
            }
        }
        if frame % FRAME_PER_MOVE == 0 {
            // ball.Move(&player1, &player2);
        }
        frame += 1;
    });
    loop {}
}
