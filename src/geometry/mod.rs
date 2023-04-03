pub(crate) mod triangle;
pub(crate) mod vertex;
pub(crate) mod cube;

use sdl2::render::Canvas;
use sdl2::video::Window;


pub trait Drawable{
    fn draw(&self, canvas: &mut Canvas<Window>);
}

