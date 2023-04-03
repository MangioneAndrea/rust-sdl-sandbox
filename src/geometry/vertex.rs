use nalgebra_glm::Mat4;

use crate::{WIDTH, HEIGHT};

#[derive(Clone, Copy)]
pub struct Vertex<const N: usize>{
    pub position: nalgebra_glm::TVec<f32, N>,
}

#[inline]
fn project(p: f32, z: f32, offset: u32) -> f32 {
        (WIDTH as f32) * match (p, z) {
            (a, z) if z == 0. => a,
            (a, z) => a / z
        } + (offset >> 1) as f32
}

impl Vertex<3> {
    pub fn as_2d(&self) -> nalgebra_glm::Vec2{
        nalgebra_glm::vec2(
            project(self.position.x, self.position.z, WIDTH),
            project(self.position.y, self.position.z, HEIGHT)
            )
    }
}

impl Vertex<4>{
    pub fn translate(&self, translation: &Mat4) -> Vertex<3> {
        let a: Mat4=nalgebra_glm::translate(translation ,&nalgebra_glm::vec3(1.,1.,1.) );
        let b: nalgebra_glm::Vec4= a * self.position;
        return Vertex{position:b.xyz()};
    }

    pub fn new(x: f32, y: f32, z:f32) -> Vertex<4>{
        Vertex{
            position: nalgebra_glm::vec4(x,y,z, 1.)
        }
    }
}
