use super::{Drawable, triangle::Triangle, triangle::Transform, vertex::Vertex, transform_matrix};

pub struct Cube {
    triangles: [Triangle; 12],
    translate: nalgebra_glm::Mat4,
    scale: nalgebra_glm::Mat4,
}

impl Cube {
    pub fn new() -> Cube{
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
        translate: nalgebra_glm::translate(m, v)!; 
        }
    }
}

impl Drawable for Cube {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.triangles.iter().map(|t|t.transform(&self.translate, &self.scale)).for_each(|t|t.draw(canvas));
    }
}
/*
            // Front
            Triangle(v0, v1, v2),
            Triangle(v0, v2, v3),
            // Back
            Triangle(v7, v6, v5),
            Triangle(v7, v5, v4),
            // Left

            Triangle(v0, v3, v7),
            Triangle(v0, v7, v4),
            // Right
            Triangle(v2, v1, v5),
            Triangle(v2, v5, v6),
            // Bottom
            Triangle(v3, v2, v6),
            Triangle(v3, v6, v7),
            // Top
            Triangle(v1, v0, v4),
            Triangle(v1, v4, v5),
*/
