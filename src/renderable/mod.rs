use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use graphics::types::Color;
pub trait Renderable{
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, color: Color);
}

