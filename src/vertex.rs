use nalgebra_glm::{Vec3, Mat4};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tex_coords: (f32, f32),
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, tex_coords: (f32, f32)) -> Self {
        Vertex {
            position,
            normal,
            tex_coords,
        }
    }
}

pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4,
    pub time: f32,
}

pub fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let translation_matrix = nalgebra_glm::translate(&Mat4::identity(), &translation);
    let scale_matrix = nalgebra_glm::scale(&Mat4::identity(), &Vec3::new(scale, scale, scale));
    
    let rotation_x = nalgebra_glm::rotate(&Mat4::identity(), rotation.x, &Vec3::new(1.0, 0.0, 0.0));
    let rotation_y = nalgebra_glm::rotate(&Mat4::identity(), rotation.y, &Vec3::new(0.0, 1.0, 0.0));
    let rotation_z = nalgebra_glm::rotate(&Mat4::identity(), rotation.z, &Vec3::new(0.0, 0.0, 1.0));
    
    let rotation_matrix = rotation_z * rotation_y * rotation_x;
    
    translation_matrix * rotation_matrix * scale_matrix
}

pub fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    nalgebra_glm::look_at(&eye, &center, &up)
}

pub fn create_perspective_matrix(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    nalgebra_glm::perspective(aspect, fov, near, far)
}

pub fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}
