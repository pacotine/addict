use macroquad::prelude::rand;

pub enum Direction {
    Pos, //left to the right / top to the bottom
    Neg //right to the left / bottom to the top
}

impl Direction {
    pub fn value(&self) -> f32 {
        match self {
            Direction::Pos => 1.0,
            Direction::Neg => -1.0,
        }
    }

    pub fn switch(&mut self) {
        match self {
            Direction::Pos => *self = Direction::Neg,
            Direction::Neg => *self = Direction::Pos,
        }
    }

    pub fn random() -> Direction {
        if rand::gen_range(1, 2) == 1 {
            Direction::Neg
        } else {
            Direction::Pos
        }
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}