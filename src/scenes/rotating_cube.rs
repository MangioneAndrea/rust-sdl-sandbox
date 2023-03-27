use crate::geometry::{triangle::Triangle, vertex::Vertex, Drawable, cube::Cube};

use super::*;

pub struct RotatingCube{}


impl Scene for RotatingCube{
    fn on_tick(&self, canvas: &mut Canvas<Window>,  delay: u32) {
        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let c=Cube::new();
        c.draw(canvas);
    }

    fn on_construction(&self) {
        
    }
}
