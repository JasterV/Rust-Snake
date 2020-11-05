extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

// WINDOW CONTROLLER
use glutin_window::GlutinWindow;
// SHAPES, COLORS AND ALL THAT STUFF
use opengl_graphics::{GlGraphics, OpenGL};
// EVENTS, WE NEED A WINDOW CONTROLLER FOR THAT
use piston::event_loop::{EventLoop, EventSettings, Events};
// EVENT ARGUMENTS FOR RENDERING, DONT WORRY JUST USE IT
use piston::input::{Button, ButtonEvent, ButtonState, Key, RenderArgs, RenderEvent, UpdateEvent};
// WINDOW SETTINGS, TO CREATE THE WINDOW CONTROLLER
use piston::window::WindowSettings;
use rand::{thread_rng, Rng};

use lib::{Direction, Renderable, Snake, Square};

const SCREEN_SIZE: f64 = 800.0;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0; 4];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

struct Game {
    gl: GlGraphics,
    snake: Snake,
    apple: Square,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        clear(BLACK, &mut self.gl);
        self.snake.render(&mut self.gl, &args, WHITE);
        self.apple.render(&mut self.gl, &args, RED);
    }

    fn update(&mut self) {
        self.snake.move_forwards();
        let head = self.snake.head();
        if head.collision(&self.apple) {
            self.apple.x = thread_rng().gen_range(0.0 + 20.0, SCREEN_SIZE - 20.0);
            self.apple.y = thread_rng().gen_range(0.0 + 20.0, SCREEN_SIZE - 20.0);
            self.snake.add_square();
        }
    }

    fn key_press(&mut self, key: Button) {
        match key {
            Button::Keyboard(Key::D) => self.snake.rotate(Direction::Right),
            Button::Keyboard(Key::A) => self.snake.rotate(Direction::Left),
            Button::Keyboard(Key::W) => self.snake.rotate(Direction::Up),
            Button::Keyboard(Key::S) => self.snake.rotate(Direction::Down),
            _ => (),
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_1;

    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [SCREEN_SIZE; 2])
        .fullscreen(true)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .expect("Error building the window");

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake::new(200.0, 200.0),
        apple: Square {
            x: thread_rng().gen_range(0.0 + 20.0, SCREEN_SIZE - 20.0),
            y: thread_rng().gen_range(0.0 + 20.0, SCREEN_SIZE - 20.0),
            size: 20.0,
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(20);

    while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.update_args() {
            game.update();
        }

        if let Some(arg) = e.render_args() {
            game.render(&arg);
        }

        if let Some(arg) = e.button_args() {
            if let ButtonState::Press = arg.state {
                game.key_press(arg.button);
            }
        }
    }
}
