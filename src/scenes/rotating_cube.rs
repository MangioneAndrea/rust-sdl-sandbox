use crate::geometry::{triangle::Triangle, vertex::Vertex, Drawable, cube::Cube};

use super::*;

pub struct RotatingCube{
    cube:Option<Cube>,
    idx: f32
}

impl RotatingCube {
    pub fn new() -> Box<RotatingCube> {
        let cube= Some(Cube::new(nalgebra_glm::vec3(0.,0., 15.)));
        Box::new(RotatingCube { cube, idx: 0.  })
    }
}


impl Scene for RotatingCube{
    fn on_tick(&mut self, canvas: &mut Canvas<Window>,  delay: u32) {
        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.idx += 0.01;
        self.cube.as_mut().unwrap().rotate(nalgebra_glm::vec3(1.,1.,1.) * self.idx);
        self.cube.as_ref().unwrap().draw(canvas);
    }

    fn on_construction(& mut self) {
        
    }
}
