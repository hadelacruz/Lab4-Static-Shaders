mod vector;
mod transform;        // Anteriormente "matrix" - refactorizado
mod orbital_camera;   // Anteriormente "camera" - refactorizado
mod sphere;
mod shaders;
mod planets;  // Módulo de planetas separados
mod ui;  // Nuevo módulo de interfaz

use raylib::prelude::*;
use vector::Vector3;
use orbital_camera::OrbitalCamera;
use sphere::Mesh;
use shaders::{PlanetShader, ShaderUniforms, ShaderColor};
use planets::{RockyPlanetShader, GasGiantShader, CrystalPlanetShader, LavaPlanetShader, SaturnShader};
use ui::render_ui;  // Importar función de UI

enum PlanetType {
    Rocky,
    GasGiant,
    Crystal,
    Nebula,
    Saturn,
}

struct Planet {
    mesh: Mesh,
    shader: Box<dyn PlanetShader>,
    rotation: f32,
    rotation_speed: f32,
}

impl Planet {
    fn new(planet_type: PlanetType) -> Self {
        // Cargar el modelo OBJ de Blender (obligatorio)
        let mesh = Mesh::from_obj("src/sphere.obj")
            .expect("❌ ERROR CRÍTICO: No se pudo cargar el archivo 'src/sphere.obj'. Asegúrate de que el archivo exista.");
        
        let (shader, rotation_speed): (Box<dyn PlanetShader>, f32) = match planet_type {
            PlanetType::Rocky => (Box::new(RockyPlanetShader), 0.5),
            PlanetType::GasGiant => (Box::new(GasGiantShader), 0.8),
            PlanetType::Crystal => (Box::new(CrystalPlanetShader), 1.2),
            PlanetType::Nebula => (Box::new(LavaPlanetShader), 1.5),
            PlanetType::Saturn => (Box::new(SaturnShader), 0.6),
        };
        
        Planet {
            mesh,
            shader,
            rotation: 0.0,
            rotation_speed,
        }
    }
    
    fn update(&mut self, dt: f32) {
        // Rotación sobre su propio eje
        self.rotation += self.rotation_speed * dt;
    }
}

fn render_planet_software(
    planet: &Planet,
    camera: &OrbitalCamera,
    uniforms: &ShaderUniforms,
    rl: &mut RaylibDrawHandle,
    width: i32,
    height: i32,
) {
    let view_matrix = camera.get_transform_matrix();
    let proj_matrix = transform::build_perspective_projection(
        45.0_f32.to_radians(),
        width as f32 / height as f32,
        0.1,
        100.0,
    );
    let viewport_matrix = transform::build_viewport_transform(0.0, 0.0, width as f32, height as f32);
    
    // Matriz de transformación del planeta (solo rotación)
    let rotation_matrix = transform::build_y_axis_rotation(planet.rotation);
    
    // Renderizar triángulos
    for i in (0..planet.mesh.indices.len()).step_by(3) {
        let i1 = planet.mesh.indices[i] as usize;
        let i2 = planet.mesh.indices[i + 1] as usize;
        let i3 = planet.mesh.indices[i + 2] as usize;
        
        if i1 >= planet.mesh.vertices.len() || i2 >= planet.mesh.vertices.len() || i3 >= planet.mesh.vertices.len() {
            continue;
        }
        
        let v1 = &planet.mesh.vertices[i1];
        let v2 = &planet.mesh.vertices[i2];
        let v3 = &planet.mesh.vertices[i3];
        
        // Aplicar rotación
        let pos1 = rotation_matrix.apply_to_vector(&v1.position);
        let pos2 = rotation_matrix.apply_to_vector(&v2.position);
        let pos3 = rotation_matrix.apply_to_vector(&v3.position);
        
        let norm1 = rotation_matrix.apply_to_vector(&v1.normal);
        let norm2 = rotation_matrix.apply_to_vector(&v2.normal);
        let norm3 = rotation_matrix.apply_to_vector(&v3.normal);
        
        // Aplicar VERTEX SHADER para deformación procedural
        let (pos1, norm1) = planet.shader.vertex_shader(pos1, norm1, v1.uv, uniforms);
        let (pos2, norm2) = planet.shader.vertex_shader(pos2, norm2, v2.uv, uniforms);
        let (pos3, norm3) = planet.shader.vertex_shader(pos3, norm3, v3.uv, uniforms);
        
        // Transformar a espacio de pantalla
        let screen1 = viewport_matrix.apply_to_vector(&proj_matrix.apply_to_vector(&view_matrix.apply_to_vector(&pos1)));
        let screen2 = viewport_matrix.apply_to_vector(&proj_matrix.apply_to_vector(&view_matrix.apply_to_vector(&pos2)));
        let screen3 = viewport_matrix.apply_to_vector(&proj_matrix.apply_to_vector(&view_matrix.apply_to_vector(&pos3)));
        
        // Calcular colores usando fragment shader
        let color1 = planet.shader.fragment_shader(pos1, norm1, v1.uv, uniforms);
        let color2 = planet.shader.fragment_shader(pos2, norm2, v2.uv, uniforms);
        let color3 = planet.shader.fragment_shader(pos3, norm3, v3.uv, uniforms);
        
        // Dibujar triángulo (simplificado - usar color promedio)
        let avg_color = ShaderColor::new(
            (color1.r + color2.r + color3.r) / 3.0,
            (color1.g + color2.g + color3.g) / 3.0,
            (color1.b + color2.b + color3.b) / 3.0,
            (color1.a + color2.a + color3.a) / 3.0,
        );
        
        // Verificar que los puntos estén en pantalla
        if screen1.x >= 0.0 && screen1.x < width as f32 && screen1.y >= 0.0 && screen1.y < height as f32 &&
           screen2.x >= 0.0 && screen2.x < width as f32 && screen2.y >= 0.0 && screen2.y < height as f32 &&
           screen3.x >= 0.0 && screen3.x < width as f32 && screen3.y >= 0.0 && screen3.y < height as f32 {
            
            rl.draw_triangle(
                Vector2::new(screen1.x, screen1.y),
                Vector2::new(screen2.x, screen2.y),
                Vector2::new(screen3.x, screen3.y),
                avg_color.to_raylib_color(),
            );
        }
    }
}

fn render_galaxy_background(d: &mut RaylibDrawHandle, width: i32, height: i32, time: f32) {
    // Fondo base con gradiente de galaxia (azul-púrpura oscuro)
    let top_color = raylib::prelude::Color::new(5, 5, 20, 255);      // Azul muy oscuro
    let bottom_color = raylib::prelude::Color::new(15, 5, 25, 255);  // Púrpura muy oscuro
    
    // Dibujar gradiente vertical
    for y in 0..height {
        let t = y as f32 / height as f32;
        let r = ((1.0 - t) * top_color.r as f32 + t * bottom_color.r as f32) as u8;
        let g = ((1.0 - t) * top_color.g as f32 + t * bottom_color.g as f32) as u8;
        let b = ((1.0 - t) * top_color.b as f32 + t * bottom_color.b as f32) as u8;
        
        d.draw_line(0, y, width, y, raylib::prelude::Color::new(r, g, b, 255));
    }
    
    // Función de ruido simple para estrellas
    let hash = |x: i32, y: i32| -> f32 {
        let n = x.wrapping_add(y.wrapping_mul(374761393));
        let n = (n << 13) ^ n;
        let nn = n.wrapping_mul(n.wrapping_mul(n.wrapping_mul(15731).wrapping_add(789221)).wrapping_add(1376312589));
        ((nn & 0x7fffffff) as f32 / 1073741824.0).abs()
    };
    
    // Dibujar estrellas como puntos blancos
    let star_density = 0.0008; // Densidad de estrellas
    let total_stars = (width * height) as f32 * star_density;
    
    for i in 0..(total_stars as i32) {
        let seed_x = i * 73856093;
        let seed_y = i * 19349663;
        
        let star_x = (hash(seed_x, 0) * width as f32) as i32;
        let star_y = (hash(seed_y, 1) * height as f32) as i32;
        
        // Variación en brillo de estrellas
        let brightness = hash(seed_x, seed_y);
        
        if brightness > 0.3 {
            let intensity = ((brightness - 0.3) / 0.7 * 255.0) as u8;
            
            // Algunas estrellas parpadean
            let twinkle = ((time * 2.0 + i as f32 * 0.1).sin() * 0.3 + 0.7).max(0.0).min(1.0);
            let final_intensity = (intensity as f32 * twinkle) as u8;
            
            // Tamaño de estrella basado en brillo
            if brightness > 0.95 {
                // Estrellas grandes y brillantes
                d.draw_circle(star_x, star_y, 2.0, raylib::prelude::Color::new(255, 255, 255, final_intensity));
                d.draw_circle(star_x, star_y, 1.0, raylib::prelude::Color::new(255, 255, 255, 255));
            } else if brightness > 0.85 {
                // Estrellas medianas
                d.draw_pixel(star_x, star_y, raylib::prelude::Color::new(255, 255, 255, 255));
                d.draw_pixel(star_x + 1, star_y, raylib::prelude::Color::new(255, 255, 255, final_intensity / 2));
                d.draw_pixel(star_x, star_y + 1, raylib::prelude::Color::new(255, 255, 255, final_intensity / 2));
            } else {
                // Estrellas pequeñas
                d.draw_pixel(star_x, star_y, raylib::prelude::Color::new(255, 255, 255, final_intensity));
            }
        }
    }
    
    // Nebulosa sutil de fondo
    for i in 0..30 {
        let nebula_x = (hash(i * 123, 456) * width as f32) as i32;
        let nebula_y = (hash(i * 789, 321) * height as f32) as i32;
        let nebula_size = (hash(i * 555, 999) * 150.0 + 50.0) as f32;
        
        let nebula_color = if hash(i * 111, 222) > 0.5 {
            raylib::prelude::Color::new(20, 10, 40, 15)  // Púrpura
        } else {
            raylib::prelude::Color::new(10, 15, 35, 15)  // Azul
        };
        
        d.draw_circle(nebula_x, nebula_y, nebula_size, nebula_color);
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1024, 768)
        .title("Laboratorio No. 4 - Shaders")
        .build();

    let mut camera = OrbitalCamera::new();
    let mut planets = vec![
        Planet::new(PlanetType::Rocky),
        Planet::new(PlanetType::GasGiant),
        Planet::new(PlanetType::Crystal),
        Planet::new(PlanetType::Nebula),
        Planet::new(PlanetType::Saturn),
    ];
    
    let mut current_planet = 0;
    let mut time = 0.0f32;

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        time += dt;
        
        // Actualizar cámara
        camera.process_input(&rl);
        
        // Cambiar planeta con teclas
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            current_planet = 0;
        } else if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            current_planet = 1;
        } else if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
            current_planet = 2;
        } else if rl.is_key_pressed(KeyboardKey::KEY_FOUR) {
            current_planet = 3;
        } else if rl.is_key_pressed(KeyboardKey::KEY_FIVE) {
            current_planet = 4;
        }
        
        // Actualizar planeta actual
        planets[current_planet].update(dt);
        
        // Configurar uniforms para shaders
        let uniforms = ShaderUniforms {
            time,
            light_direction: Vector3::new(1.0, 1.0, 1.0).normalize(),
            camera_position: camera.position,
        };
        
        let mut d = rl.begin_drawing(&thread);
        
        // Renderizar fondo de galaxia
        render_galaxy_background(&mut d, 1024, 768, time);
        
        // Renderizar usando nuestro software renderer
        render_planet_software(
            &planets[current_planet],
            &camera,
            &uniforms,
            &mut d,
            1024,
            768,
        );
        
        // Renderizar UI mejorada
        let current_fps = d.get_fps() as i32;
        render_ui(&mut d, current_planet, current_fps);
    }
}
