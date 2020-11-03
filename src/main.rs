extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

// WINDOW CONTROLLER
use glutin_window::GlutinWindow;
// SHAPES, COLORS AND ALL THAT STUFF
use opengl_graphics::{GlGraphics, OpenGL};
// EVENTS, WE NEED A WINDOW CONTROLLER FOR THAT
use piston::event_loop::{EventSettings, Events};
// EVENT ARGUMENTS FOR RENDERING, DONT WORRY JUST USE IT
use piston::input::{
    Button, ButtonEvent, ButtonState, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
// WINDOW SETTINGS, TO CREATE THE WINDOW CONTROLLER
use piston::window::WindowSettings;

use lib::{Direction, Snake};

struct Game {
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {}

    fn update(&mut self, args: &UpdateArgs) {}

    fn key_press(&mut self, key: Button) {}
}

fn main() {
    let opengl = OpenGL::V3_1;

    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [400, 400])
        .fullscreen(true)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .expect("Error building the window");

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake::new(0.0, 0.0),
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(arg) = e.render_args() {}

        if let Some(arg) = e.update_args() {}

        if let Some(arg) = e.button_args() {
            if let ButtonState::Press = arg.state {
                game.key_press(arg.button);
            }
        }
    }
}
