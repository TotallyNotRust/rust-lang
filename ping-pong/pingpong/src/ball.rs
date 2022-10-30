use std::io::stdout;

use crossterm::{cursor, execute};

use crate::{
    graphic::{self, HEIGHT, WIDTH},
    player::Player,
};

pub struct Ball {
    pub x: usize,
    pub y: usize,
    pub velx: i32,
    pub vely: i32,
}

impl Ball {
    pub fn Move(&mut self, p1: &Player, p2: &Player) {
        println!("{},{} {},{}", self.velx, self.vely, self.x, self.y);

        if self.velx > 0 {
            self.x += self.velx as usize;
        } else if self.x >= (self.velx * -1) as usize {
            self.x -= (self.velx * -1) as usize;
        }

        if self.vely > 0 {
            self.y -= self.vely as usize;
        } else if self.y >= (self.vely * -1) as usize {
            self.y += (self.vely * -1) as usize;
        }

        if self.x >= WIDTH || self.x <= 0 {
            self.velx = self.velx * -1
        }

        if self.y >= HEIGHT || self.y <= 0 {
            self.vely = self.vely * -1;
        }

        if self.x <= 0 {
            if !(p1.contains_point(self.y)) {
                graphic::dispose(false);
                // panic!("Player two won");
                execute!(stdout(), cursor::MoveTo(0, 70)).unwrap();
                // println!("Player two won {:?}", self.x)
            }
        } else if self.x >= WIDTH {
            if !(p2.contains_point(self.y)) {
                graphic::dispose(false);
                execute!(stdout(), cursor::MoveTo(0, 70)).unwrap();
                // println!("Player one won {:?}", self.x)
            }
        }

        // println!("{},{}", self.x, self.y);
    }
}
