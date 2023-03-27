pub struct Vertex{
    pub position: nalgebra_glm::Vec3,
}

impl Vertex{
    pub fn as_2d(&self) -> nalgebra_glm::Vec2{
        nalgebra_glm::vec3_to_vec2(&self.position)
    }

    pub fn new(x: f32, y: f32, z:f32) -> Vertex{
        Vertex{
            position: nalgebra_glm::vec3(x,y,z)
        }
    }
}
