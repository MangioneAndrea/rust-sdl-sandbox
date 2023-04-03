use nalgebra_glm::Mat4;

use super::vertex::*;

#[derive(Clone)]
pub struct Triangle<const N:usize>{
    a: Vertex<N>,
    b: Vertex<N>,
    c: Vertex<N>
}

impl Triangle<4> {
    pub fn new(a: Vertex<4>, b:Vertex<4>, c:Vertex<4>) -> Triangle<4>{
        Triangle{a,b,c}
    }

    pub fn new_clone(a: &Vertex<4>, b: &Vertex<4>, c: &Vertex<4>) -> Triangle<4>{
        Triangle::new(a.clone(), b.clone(), c.clone())
    }

    pub fn transform(&self, translate: &Mat4, scale: &Mat4) -> Triangle<3>{
        Triangle{
            a: self.a.translate(translate),
            b: self.b.translate(translate),
            c: self.c.translate(translate),
        }
    }
}

impl crate::geometry::Drawable for Triangle<3>{
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
       let a=&self.a.as_2d(); 
       let b=&self.b.as_2d(); 
       let c=&self.c.as_2d(); 
       
        let ab= b-a;
        let ac= c-a;

        if ab.x * ac.y - ab.y * ac.x > 0. {
            let inv=nalgebra_glm::mat2(ab.x,ab.y, ac.x, ac.y).try_inverse().unwrap();
            for i in nalgebra_glm::min3_scalar(a.y, b.y, c.y) as i32 .. nalgebra_glm::max3_scalar(a.y, b.y, c.y) as i32 {
                for j in nalgebra_glm::min3_scalar(a.x, b.x, c.x) as i32 .. nalgebra_glm::max3_scalar(a.x, b.x, c.x) as i32 {
                    let p = nalgebra_glm::vec2(j as f32, i as f32);
                    let uv = &inv * &(p - a);

                    // Is in screeen
                    if uv.x >= 0. && uv.y >=0. && (uv.x + uv.y) <1. {
                        canvas.draw_point((p.x as i32, p.y as i32));
                    }
                }
            }
        }
    }
}


/*
            for (int i = glm::min(a2d.y, b2d.y, c2d.y); i < glm::max(a2d.y, b2d.y, c2d.y); i++) {
                for (int j = glm::min(a2d.x, b2d.x, c2d.x); j < glm::max(a2d.x, b2d.x, c2d.x); j++) {
                    auto p = glm::dvec2(j, i);
                    glm::dvec2 uv = inv * (p - a2d);

                    if (uv.x >= 0 && uv.y >= 0 && (uv.x + uv.y) < 1) {
                        auto homogeneous = homogeneousA + (homogeneousB - homogeneousA) * uv.x +
                                           (homogeneousC - homogeneousA) * uv.y;


                        auto normalA = a.GetNormal();
                        auto normalB = b.GetNormal();
                        auto normalC = c.GetNormal();


                        auto cosThetaA =
                                glm::dot((a.coordinates - LIGHT_SOURCE), normalA) * LIGHT_INTENSITY;
                        auto cosThetaB =
                                glm::dot((b.coordinates - LIGHT_SOURCE), normalB) * LIGHT_INTENSITY;
                        auto cosThetaC =
                                glm::dot((c.coordinates - LIGHT_SOURCE), normalC) * LIGHT_INTENSITY;
                        if (cosThetaA < 0)
                            cosThetaA = 0;
                        if (cosThetaB < 0)
                            cosThetaB = 0;
                        if (cosThetaC < 0)
                            cosThetaC = 0;

                        auto lambertA = a.color * cosThetaA / a.coordinates.z;
                        auto lambertB = b.color * cosThetaB / b.coordinates.z;
                        auto lambertC = c.color * cosThetaC / c.coordinates.z;

                        auto zValue = a.coordinates.z + (b.coordinates.z - a.coordinates.z) * uv.x +
                                      (c.coordinates.z - a.coordinates.z) * uv.y;
                        if (zbuffer[i * WIDTH + j] > zValue || zbuffer[i * WIDTH + j] == 0) {
                            zbuffer[i * WIDTH + j] = (float) zValue;

                            auto lambert = (lambertA + (lambertB - lambertA) * uv.x +
                                            (lambertC - lambertA) * uv.y)/homogeneous;

                            auto position = a.coordinates + (b.coordinates - a.coordinates) * uv.x +
                                            (c.coordinates - a.coordinates) * uv.y;

                            auto normal = (glm::abs(normalA) + (glm::abs(normalB) - glm::abs(normalA)) * uv.x +
                                           (glm::abs(normalC) - glm::abs(normalA)) * uv.y)/homogeneous;

                            auto np = (normal + position) * 0.5;
                            pixels[i * WIDTH + j] = Color(lambert).ToInt();
                        }
                    }
                }
            }
        }
        */
