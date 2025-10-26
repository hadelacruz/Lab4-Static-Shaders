#![allow(dead_code)]

use raylib::prelude::*;
use crate::matrix::create_view_matrix;
use crate::vector::Vector3;
use std::f32::consts::PI;

pub struct Camera {
    // Posición y orientación de la cámara
    pub eye: Vector3,        // Posición de la cámara
    pub target: Vector3,     // Punto al que mira la cámara
    pub up: Vector3,         // Vector arriba

    // Parámetros de cámara orbital
    pub yaw: f32,            // Rotación alrededor del eje Y (izquierda/derecha)
    pub pitch: f32,          // Rotación alrededor del eje X (arriba/abajo)
    pub distance: f32,       // Distancia desde el objetivo

    // Velocidad de movimiento
    pub rotation_speed: f32,
    pub zoom_speed: f32,
    pub pan_speed: f32,
}

impl Camera {
    pub fn new() -> Self {
        let mut camera = Camera {
            eye: Vector3::new(0.0, 0.0, 5.0),
            target: Vector3::new(0.0, 0.0, 0.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            distance: 5.0,
            rotation_speed: 2.0,
            zoom_speed: 1.0,
            pan_speed: 0.5,
        };
        camera.update_position();
        camera
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        let mouse_delta = rl.get_mouse_delta();
        let wheel_move = rl.get_mouse_wheel_move();

        // Rotar cámara con el mouse
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            self.yaw -= mouse_delta.x * self.rotation_speed * rl.get_frame_time();
            self.pitch -= mouse_delta.y * self.rotation_speed * rl.get_frame_time();
            
            // Limitar pitch para evitar voltear
            self.pitch = self.pitch.clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);
        }

        // Zoom con rueda del mouse
        self.distance -= wheel_move * self.zoom_speed;
        self.distance = self.distance.clamp(1.0, 20.0);

        // Pan con botón derecho del mouse
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
            let right = Vector3::new(self.yaw.cos(), 0.0, self.yaw.sin());
            let forward = Vector3::new(-self.yaw.sin(), 0.0, self.yaw.cos());
            
            self.target = self.target + right * (-mouse_delta.x * self.pan_speed * rl.get_frame_time());
            self.target = self.target + forward * (mouse_delta.y * self.pan_speed * rl.get_frame_time());
        }

        self.update_position();
    }

    fn update_position(&mut self) {
        // Calcular posición de la cámara basada en coordenadas esféricas
        let x = self.target.x + self.distance * self.pitch.cos() * self.yaw.cos();
        let y = self.target.y + self.distance * self.pitch.sin();
        let z = self.target.z + self.distance * self.pitch.cos() * self.yaw.sin();
        
        self.eye = Vector3::new(x, y, z);
    }

    pub fn get_view_matrix(&self) -> crate::matrix::Matrix {
        create_view_matrix(self.eye, self.target, self.up)
    }

    pub fn get_raylib_camera(&self) -> Camera3D {
        Camera3D::perspective(
            raylib::prelude::Vector3 { x: self.eye.x, y: self.eye.y, z: self.eye.z },
            raylib::prelude::Vector3 { x: self.target.x, y: self.target.y, z: self.target.z },
            raylib::prelude::Vector3 { x: self.up.x, y: self.up.y, z: self.up.z },
            45.0,
        )
    }
}