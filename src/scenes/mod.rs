use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub(crate) mod rotating_cube;

pub trait Scene{
    fn on_tick(&mut self,canvas: &mut Canvas<Window>, delay: u32);
    fn on_construction(& mut self);
}

pub struct Static{}

impl Scene for Static {
   fn on_tick(&mut self,canvas: &mut Canvas<Window>, delay: u32) {
       canvas.set_draw_color(Color::RGB(0, 0, 0));
       canvas.clear();
   } 

   fn on_construction(&mut self) {
       
   }
}
