use super::*;

pub struct RotatingCube{}


impl Scene for RotatingCube{
    fn on_tick(&self, canvas: &mut Canvas<Window>,  delay: u32) {
       canvas.set_draw_color(Color::RGB(255, 0, 0));
       canvas.clear();
    }

    fn on_construction(&self) {
        
    }
}
