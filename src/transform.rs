// ============================================================================
// MÓDULO DE TRANSFORMACIONES MATEMÁTICAS 4x4
// ============================================================================

use crate::vector::Vector3;

/// Estructura que representa una matriz de transformación 4x4
#[derive(Debug, Clone, Copy)]
pub struct Transform4x4 {
    pub elements: [[f32; 4]; 4],
}

impl Transform4x4 {
    /// Crea una matriz identidad (sin transformación)
    pub fn identity() -> Self {
        Transform4x4 {
            elements: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Constructor completo con todos los elementos de la matriz
    pub fn from_elements(
        e00: f32, e01: f32, e02: f32, e03: f32,
        e10: f32, e11: f32, e12: f32, e13: f32,
        e20: f32, e21: f32, e22: f32, e23: f32,
        e30: f32, e31: f32, e32: f32, e33: f32,
    ) -> Self {
        Transform4x4 {
            elements: [
                [e00, e01, e02, e03],
                [e10, e11, e12, e13],
                [e20, e21, e22, e23],
                [e30, e31, e32, e33],
            ],
        }
    }

    /// Multiplica dos matrices (composición de transformaciones)
    pub fn compose(&self, other: &Transform4x4) -> Transform4x4 {
        let mut result = Transform4x4::identity();
        
        for row in 0..4 {
            for col in 0..4 {
                result.elements[row][col] = 0.0;
                for k in 0..4 {
                    result.elements[row][col] += self.elements[row][k] * other.elements[k][col];
                }
            }
        }
        
        result
    }

    /// Aplica la transformación a un vector 3D
    pub fn apply_to_vector(&self, vec: &Vector3) -> Vector3 {
        let transformed_x = self.elements[0][0] * vec.x + self.elements[0][1] * vec.y 
                          + self.elements[0][2] * vec.z + self.elements[0][3];
        let transformed_y = self.elements[1][0] * vec.x + self.elements[1][1] * vec.y 
                          + self.elements[1][2] * vec.z + self.elements[1][3];
        let transformed_z = self.elements[2][0] * vec.x + self.elements[2][1] * vec.y 
                          + self.elements[2][2] * vec.z + self.elements[2][3];
        let w_component = self.elements[3][0] * vec.x + self.elements[3][1] * vec.y 
                        + self.elements[3][2] * vec.z + self.elements[3][3];
        
        if w_component != 0.0 && w_component != 1.0 {
            Vector3::new(
                transformed_x / w_component,
                transformed_y / w_component,
                transformed_z / w_component
            )
        } else {
            Vector3::new(transformed_x, transformed_y, transformed_z)
        }
    }
}

/// Crea una matriz de vista (lookAt) para la cámara
pub fn build_view_transform(observer: Vector3, focus: Vector3, up_direction: Vector3) -> Transform4x4 {
    // Vector dirección (de la cámara al objetivo)
    let mut direction = Vector3::new(
        focus.x - observer.x,
        focus.y - observer.y,
        focus.z - observer.z,
    );
    
    // Normalizar dirección
    let dir_magnitude = (direction.x * direction.x + direction.y * direction.y + direction.z * direction.z).sqrt();
    direction.x /= dir_magnitude;
    direction.y /= dir_magnitude;
    direction.z /= dir_magnitude;

    // Vector derecha (producto cruz de dirección y arriba)
    let mut right_vec = Vector3::new(
        direction.y * up_direction.z - direction.z * up_direction.y,
        direction.z * up_direction.x - direction.x * up_direction.z,
        direction.x * up_direction.y - direction.y * up_direction.x,
    );
    
    // Normalizar derecha
    let right_magnitude = (right_vec.x * right_vec.x + right_vec.y * right_vec.y + right_vec.z * right_vec.z).sqrt();
    right_vec.x /= right_magnitude;
    right_vec.y /= right_magnitude;
    right_vec.z /= right_magnitude;

    // Recalcular vector arriba (producto cruz de derecha y dirección)
    let corrected_up = Vector3::new(
        right_vec.y * direction.z - right_vec.z * direction.y,
        right_vec.z * direction.x - right_vec.x * direction.z,
        right_vec.x * direction.y - right_vec.y * direction.x,
    );

    // Construir matriz de vista (inversa de la transformación de cámara)
    Transform4x4::from_elements(
        right_vec.x, right_vec.y, right_vec.z, 
        -(right_vec.x * observer.x + right_vec.y * observer.y + right_vec.z * observer.z),
        
        corrected_up.x, corrected_up.y, corrected_up.z, 
        -(corrected_up.x * observer.x + corrected_up.y * observer.y + corrected_up.z * observer.z),
        
        -direction.x, -direction.y, -direction.z, 
        direction.x * observer.x + direction.y * observer.y + direction.z * observer.z,
        
        0.0, 0.0, 0.0, 1.0,
    )
}

/// Crea una matriz de proyección perspectiva
pub fn build_perspective_projection(
    vertical_fov: f32, 
    aspect_ratio: f32, 
    near_plane: f32, 
    far_plane: f32
) -> Transform4x4 {
    let focal_length = 1.0 / (vertical_fov * 0.5).tan();

    Transform4x4::from_elements(
        focal_length / aspect_ratio, 0.0, 0.0, 0.0,
        0.0, focal_length, 0.0, 0.0,
        0.0, 0.0, -(far_plane + near_plane) / (far_plane - near_plane), 
        -(2.0 * far_plane * near_plane) / (far_plane - near_plane),
        0.0, 0.0, -1.0, 0.0,
    )
}

/// Crea una transformación de viewport (NDC a coordenadas de pantalla)
pub fn build_viewport_transform(origin_x: f32, origin_y: f32, width: f32, height: f32) -> Transform4x4 {
    let half_w = width * 0.5;
    let half_h = height * 0.5;

    Transform4x4::from_elements(
        half_w, 0.0, 0.0, origin_x + half_w,
        0.0, -half_h, 0.0, origin_y + half_h,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}

/// Crea una rotación alrededor del eje Y
pub fn build_y_axis_rotation(radians: f32) -> Transform4x4 {
    let cosine = radians.cos();
    let sine = radians.sin();
    
    Transform4x4::from_elements(
        cosine, 0.0, sine, 0.0,
        0.0, 1.0, 0.0, 0.0,
        -sine, 0.0, cosine, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}

/// Crea una transformación de traslación
pub fn build_translation(offset_x: f32, offset_y: f32, offset_z: f32) -> Transform4x4 {
    Transform4x4::from_elements(
        1.0, 0.0, 0.0, offset_x,
        0.0, 1.0, 0.0, offset_y,
        0.0, 0.0, 1.0, offset_z,
        0.0, 0.0, 0.0, 1.0,
    )
}
