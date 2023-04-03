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
    pub fn transform(&self, translation: &nalgebra_glm::Vec3, rotation: &nalgebra_glm::Vec3) -> Vertex<3> {
        let tr = nalgebra_glm::mat4(
            1.,0.,0., translation.x,
            0.,1.,0., translation.y,
            0.,0.,1., translation.z,
            0.,0.,0., 0.
        );

        let rot_y = nalgebra_glm::mat4(
            rotation.y.cos(),rotation.y.sin(),0.,0.,
            -rotation.y.sin(),rotation.y.cos(),0.,0.,
            0.,0.,1., 0.,
            0.,0.,0., 1.
        );

        let b: nalgebra_glm::Vec4= tr * rot_y * self.position;
        return Vertex{position:b.xyz()};
    }

    pub fn new(x: f32, y: f32, z:f32) -> Vertex<4>{
        Vertex{
            position: nalgebra_glm::vec4(x,y,z, 1.)
        }
    }
}
