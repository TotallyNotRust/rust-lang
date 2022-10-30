use std::clone;

pub struct Player {
    pub y: usize,
    pub height: usize,
    pub gameheight: usize,
}

impl Player {
    pub fn contains_point(&self, point: usize) -> bool {
        return point == self.y || (point > self.y && point <= (self.y + self.height));
    }
}
