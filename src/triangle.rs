use nalgebra_glm::{Vec3, Vec4, Mat4};
use crate::vertex::Vertex;
use crate::fragment::Fragment;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Mat4) -> Vec4 {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );
    uniforms * position
}

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex, uniforms: &Mat4) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let transformed_v1 = vertex_shader(v1, uniforms);
    let transformed_v2 = vertex_shader(v2, uniforms);
    let transformed_v3 = vertex_shader(v3, uniforms);

    let w1 = transformed_v1.w;
    let w2 = transformed_v2.w;
    let w3 = transformed_v3.w;

    let screen_v1 = Vec3::new(transformed_v1.x / w1, transformed_v1.y / w1, transformed_v1.z / w1);
    let screen_v2 = Vec3::new(transformed_v2.x / w2, transformed_v2.y / w2, transformed_v2.z / w2);
    let screen_v3 = Vec3::new(transformed_v3.x / w3, transformed_v3.y / w3, transformed_v3.z / w3);

    let min_x = screen_v1.x.min(screen_v2.x).min(screen_v3.x).floor() as i32;
    let min_y = screen_v1.y.min(screen_v2.y).min(screen_v3.y).floor() as i32;
    let max_x = screen_v1.x.max(screen_v2.x).max(screen_v3.x).ceil() as i32;
    let max_y = screen_v1.y.max(screen_v2.y).max(screen_v3.y).ceil() as i32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);
            let (w1, w2, w3) = barycentric_coordinates(&point, &screen_v1, &screen_v2, &screen_v3);

            if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {
                let depth = screen_v1.z * w1 + screen_v2.z * w2 + screen_v3.z * w3;
                
                let normal = v1.normal * w1 + v2.normal * w2 + v3.normal * w3;
                let vertex_pos = v1.position * w1 + v2.position * w2 + v3.position * w3;

                fragments.push(Fragment::new(
                    Vec3::new(x as f32, y as f32, depth),
                    normal,
                    depth,
                    vertex_pos,
                ));
            }
        }
    }

    fragments
}

fn barycentric_coordinates(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3) -> (f32, f32, f32) {
    let v0 = Vec3::new(b.x - a.x, b.y - a.y, 0.0);
    let v1 = Vec3::new(c.x - a.x, c.y - a.y, 0.0);
    let v2 = Vec3::new(p.x - a.x, p.y - a.y, 0.0);

    let d00 = v0.dot(&v0);
    let d01 = v0.dot(&v1);
    let d11 = v1.dot(&v1);
    let d20 = v2.dot(&v0);
    let d21 = v2.dot(&v1);

    let denom = d00 * d11 - d01 * d01;
    if denom.abs() < f32::EPSILON {
        return (0.0, 0.0, 0.0);
    }

    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;

    (u, v, w)
}
