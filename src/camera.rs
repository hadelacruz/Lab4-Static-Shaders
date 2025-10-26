use nalgebra_glm::Vec3;

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
    pub distance: f32,
    pub orbit_speed: f32,
    pub angle: f32,
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        let distance = (eye - center).magnitude();
        Camera {
            eye,
            center,
            up,
            distance,
            orbit_speed: 0.3,
            angle: 0.0,
        }
    }

    pub fn orbit(&mut self, delta_time: f32) {
        self.angle += self.orbit_speed * delta_time;
        
        self.eye.x = self.center.x + self.distance * self.angle.cos();
        self.eye.z = self.center.z + self.distance * self.angle.sin();
    }

    pub fn zoom(&mut self, delta: f32) {
        self.distance = (self.distance + delta).max(2.0).min(15.0);
        
        let direction = (self.eye - self.center).normalize();
        self.eye = self.center + direction * self.distance;
    }
}
