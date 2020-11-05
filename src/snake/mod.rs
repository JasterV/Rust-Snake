use crate::renderable::Renderable;
use graphics::types::Color;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use std::collections::LinkedList;
use std::iter::FromIterator;

#[derive(PartialEq, Clone, Debug)]
pub struct Square {
    pub x: f64,
    pub y: f64,
    pub size: f64,
}

impl Square {
    pub fn collision(&self, other: &Square) -> bool {
        self.x + self.size >= other.x
            && self.x <= other.x + other.size
            && self.y + self.size >= other.y
            && self.y <= other.y + self.size
    }
}

#[derive(PartialEq, Clone)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

pub struct Snake {
    pub body: LinkedList<Square>,
    direction: Direction,
}

impl Snake {
    pub const SIZE: f64 = 20.0;

    pub fn new(x: f64, y: f64) -> Self {
        let body = LinkedList::from_iter(vec![Square {
            x,
            y,
            size: Self::SIZE,
        }]);
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
            _ => (),
        }
    }

    pub fn move_forwards(&mut self) {
        let mut head: Square = self.body.front().unwrap().clone();
        
        match self.direction {
            Direction::Up => head.y -= Snake::SIZE,
            Direction::Down => head.y += Snake::SIZE,
            Direction::Left => head.x -= Snake::SIZE,
            Direction::Right => head.x += Snake::SIZE,
        }

        self.body.pop_back();
        self.body.push_front(head);
    }

    pub fn add_square(&mut self) {
        let tail = self.body.back().unwrap();
        let new_part = Square {
            x: tail.x,
            y: tail.y,
            size: Self::SIZE,
        };
        self.body.push_back(new_part);
    }

    pub fn head(&self) -> &Square {
        self.body.front().unwrap()
    }
}

impl Renderable for Square {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, color: Color) {
        gl.draw(args.viewport(), |c, gl| {
            use graphics::*;
            let square = rectangle::square(self.x, self.y, self.size);
            let transform = c.transform.trans(-self.size / 2.0, -self.size / 2.0);
            rectangle(color, square, transform, gl);
        });
    }
}

impl Renderable for Snake {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, color: Color) {
        gl.draw(args.viewport(), |_, gl| {
            self.body
                .iter_mut()
                .for_each(|square| square.render(gl, args, color));
        });
    }
}
