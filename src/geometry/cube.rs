use super::{Drawable, triangle::Triangle, vertex::Vertex};

pub struct Cube {
    triangles: [Triangle<4>; 12],
    translate: nalgebra_glm::Vec3,
    rotate: nalgebra_glm::Vec3,
}

impl Cube {
    pub fn new(center: nalgebra_glm::Vec3) -> Cube{
        let a= &Vertex::new(-1., -1., -1.);
        let b= &Vertex::new(1., -1., -1.);
        let c= &Vertex::new(1., 1., -1.);
        let d= &Vertex::new(-1., 1., -1.);
        let e= &Vertex::new(-1., -1., 1.);
        let f= &Vertex::new(1., -1., 1.);
        let g= &Vertex::new(1., 1., 1.);
        let h= &Vertex::new(-1., 1., 1.);

        Cube { triangles: [
            // Front
            Triangle::new_clone(a, b, c),
            Triangle::new_clone(a, c, d),
            // Back 
            Triangle::new_clone(h, g, f),
            Triangle::new_clone(h, f, e),
            // Left 
            Triangle::new_clone(a, d, h),
            Triangle::new_clone(a, h, e),
            // Right 
            Triangle::new_clone(c, b, f),
            Triangle::new_clone(c, f, g),
             // Bottom
            Triangle::new_clone(d, c, g),
            Triangle::new_clone(d, g, h),
            // Top 
            Triangle::new_clone(b, a, e),
            Triangle::new_clone(b, e, f),
        ], 
        translate: center,
        rotate: nalgebra_glm::Vec3::default(), 
        }
    }
    pub fn rotate(&mut self, rotate: nalgebra_glm::Vec3){
        self.rotate=rotate;
    }
    
    pub fn translate(&mut self, translate: nalgebra_glm::Vec3){
        self.translate=translate;
    }
}

impl Drawable for Cube {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        //self.triangles.iter().nth(4).unwrap().transform(&self.translate, &self.rotate).draw(canvas);
        self.triangles.iter().map(|t|t.transform(&self.translate, &self.rotate)).for_each(|t|t.draw(canvas));
    }

}
