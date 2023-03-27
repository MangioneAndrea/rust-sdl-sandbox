use crate::geometry::{triangle::Triangle, vertex::Vertex, Drawable};

use super::*;

pub struct RotatingCube{}


impl Scene for RotatingCube{
    fn on_tick(&self, canvas: &mut Canvas<Window>,  delay: u32) {
        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let t=Triangle::new(
            Vertex::new(50., 100., 0.),
            Vertex::new(75., 100., 0.),
            Vertex::new(30., 200., 0.),
        );
        t.draw(canvas);
    }

    fn on_construction(&self) {
        
    }
}
