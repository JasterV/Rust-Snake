use std::collections::LinkedList;
use std::iter::FromIterator;

#[derive(PartialEq, Clone)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

pub struct Snake {
    pub body: LinkedList<(f64, f64)>,
    direction: Direction,
}

impl Snake {
    pub const VELOCITY: f64 = 8.0;
    pub const SIZE: f64 = 20.0;

    pub fn new(x: f64, y: f64) -> Self {
        let body: LinkedList<(f64, f64)> = LinkedList::from_iter(vec![(x, y)].into_iter());
        Snake {
            body,
            direction: Direction::Right,
        }
    }

    pub fn rotate(&mut self, direction: Direction) {
        match direction {
            Direction::Up if self.direction != Direction::Down => self.direction = direction,
            Direction::Down if self.direction != Direction::Up => self.direction = direction,
            Direction::Left if self.direction != Direction::Right => self.direction = direction,
            Direction::Right if self.direction != Direction::Left => self.direction = direction,
            _ => ()
        }
    }

    pub fn move_forwards(&mut self) {
        let mut head: (f64, f64) = self
            .body
            .front()
            .expect("Cannot move an empty snake")
            .clone();
        match self.direction {
            Direction::Up => head.1 -= Snake::VELOCITY,
            Direction::Down => head.1 += Snake::VELOCITY,
            Direction::Left => head.0 -= Snake::VELOCITY,
            Direction::Right => head.0 += Snake::VELOCITY,
        }
        self.body.pop_back();
        self.body.push_front(head);
    }

    pub fn add_square(&mut self) {
        let back = self.body.back().unwrap();
        let new_part = (back.0, back.1);
        self.body.push_back(new_part);
    }
}
