// ============================================================================
// SISTEMA DE CÁMARA ORBITAL INTERACTIVA
// ============================================================================

#![allow(dead_code)]

use raylib::prelude::*;
use crate::transform::build_view_transform;
use crate::vector::Vector3;
use std::f32::consts::PI;

/// Controlador de cámara orbital con interacción por mouse
pub struct OrbitalCamera {
    // Configuración espacial
    pub position: Vector3,      
    pub focal_point: Vector3,   
    pub up_vector: Vector3,     

    // Parámetros orbitales (coordenadas esféricas)
    pub azimuth: f32,           
    pub elevation: f32,         
    pub radius: f32,            

    // Configuración de sensibilidad
    pub orbit_sensitivity: f32,  
    pub zoom_sensitivity: f32,   
    pub pan_sensitivity: f32,    
}

impl OrbitalCamera {
    /// Crea una nueva cámara orbital con configuración por defecto
    pub fn new() -> Self {
        let mut camera = OrbitalCamera {
            position: Vector3::new(0.0, 0.0, 5.0),
            focal_point: Vector3::new(0.0, 0.0, 0.0),
            up_vector: Vector3::new(0.0, 1.0, 0.0),
            azimuth: 0.0,
            elevation: 0.0,
            radius: 5.0,
            orbit_sensitivity: 2.0,
            zoom_sensitivity: 1.0,
            pan_sensitivity: 0.5,
        };
        camera.recalculate_position();
        camera
    }

    /// Actualiza el estado de la cámara basado en entrada del usuario
    pub fn process_input(&mut self, raylib_handle: &RaylibHandle) {
        let mouse_movement = raylib_handle.get_mouse_delta();
        let scroll_delta = raylib_handle.get_mouse_wheel_move();

        // Control de órbita con clic izquierdo
        if raylib_handle.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let delta_time = raylib_handle.get_frame_time();
            self.azimuth -= mouse_movement.x * self.orbit_sensitivity * delta_time;
            self.elevation -= mouse_movement.y * self.orbit_sensitivity * delta_time;
            
            // Limitar elevación para evitar gimbal lock
            const ELEVATION_LIMIT: f32 = PI * 0.5 - 0.1;
            self.elevation = self.elevation.clamp(-ELEVATION_LIMIT, ELEVATION_LIMIT);
        }

        // Control de zoom con rueda del mouse
        if scroll_delta != 0.0 {
            self.radius -= scroll_delta * self.zoom_sensitivity;
            self.radius = self.radius.clamp(1.0, 20.0);
        }

        // Control de paneo (desplazamiento) con clic derecho
        if raylib_handle.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
            let delta_time = raylib_handle.get_frame_time();
            
            // Calcular vectores laterales basados en orientación actual
            let lateral_vector = Vector3::new(
                self.azimuth.cos(), 
                0.0, 
                self.azimuth.sin()
            );
            let frontal_vector = Vector3::new(
                -self.azimuth.sin(), 
                0.0, 
                self.azimuth.cos()
            );
            
            // Aplicar desplazamiento al punto focal
            let horizontal_shift = -mouse_movement.x * self.pan_sensitivity * delta_time;
            let vertical_shift = mouse_movement.y * self.pan_sensitivity * delta_time;
            
            self.focal_point = self.focal_point + lateral_vector * horizontal_shift;
            self.focal_point = self.focal_point + frontal_vector * vertical_shift;
        }

        self.recalculate_position();
    }

    /// Recalcula la posición de la cámara usando coordenadas esféricas
    fn recalculate_position(&mut self) {
        let horizontal_distance = self.radius * self.elevation.cos();
        
        let cam_x = self.focal_point.x + horizontal_distance * self.azimuth.cos();
        let cam_y = self.focal_point.y + self.radius * self.elevation.sin();
        let cam_z = self.focal_point.z + horizontal_distance * self.azimuth.sin();
        
        self.position = Vector3::new(cam_x, cam_y, cam_z);
    }

    /// Obtiene la matriz de vista para usar en transformaciones
    pub fn get_transform_matrix(&self) -> crate::transform::Transform4x4 {
        build_view_transform(self.position, self.focal_point, self.up_vector)
    }

    /// Convierte a formato de cámara de Raylib para renderizado
    pub fn to_raylib_camera(&self) -> Camera3D {
        Camera3D::perspective(
            raylib::prelude::Vector3 { 
                x: self.position.x, 
                y: self.position.y, 
                z: self.position.z 
            },
            raylib::prelude::Vector3 { 
                x: self.focal_point.x, 
                y: self.focal_point.y, 
                z: self.focal_point.z 
            },
            raylib::prelude::Vector3 { 
                x: self.up_vector.x, 
                y: self.up_vector.y, 
                z: self.up_vector.z 
            },
            45.0,
        )
    }
}
