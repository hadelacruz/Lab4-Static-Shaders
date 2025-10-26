mod framebuffer;
mod vertex;
mod fragment;
mod obj_loader;
mod triangle;
mod render;
mod camera;

use framebuffer::{Framebuffer, Color};
use vertex::{Uniforms, create_model_matrix, create_view_matrix, create_perspective_matrix, create_viewport_matrix};
use fragment::{rocky_planet_shader, gas_giant_shader, crystal_planet_shader, FragmentShader};
use obj_loader::load_obj;
use render::render;
use camera::Camera;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec3;
use std::time::Instant;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const TARGET_FPS: u64 = 60;
const FRAME_DELAY: u64 = 1000 / TARGET_FPS;

#[derive(Clone, Copy, PartialEq)]
enum PlanetType {
    Rocky,
    GasGiant,
    Crystal,
}

impl PlanetType {
    fn get_shader(&self) -> FragmentShader {
        match self {
            PlanetType::Rocky => rocky_planet_shader,
            PlanetType::GasGiant => gas_giant_shader,
            PlanetType::Crystal => crystal_planet_shader,
        }
    }

    fn get_name(&self) -> &str {
        match self {
            PlanetType::Rocky => "Planeta Rocoso",
            PlanetType::GasGiant => "Gigante Gaseoso",
            PlanetType::Crystal => "Planeta Cristalino",
        }
    }
}

fn main() {
    println!("=== Laboratorio 4: Shaders de Planetas ===");
    println!("Cargando modelo de esfera...");
    
    let vertices = match load_obj("esfera.obj") {
        Ok(v) => {
            println!("✓ Modelo cargado: {} vértices", v.len());
            v
        },
        Err(e) => {
            eprintln!("✗ Error cargando esfera.obj: {}", e);
            println!("Asegúrate de que el archivo esfera.obj existe en el directorio raíz del proyecto.");
            return;
        }
    };

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    framebuffer.set_background_color(Color::new(10, 10, 30));

    let mut window = Window::new(
        "Lab 4 - Planetas con Shaders",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Error creando ventana: {}", e);
    });

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut current_planet = PlanetType::Rocky;
    let mut last_key_time = Instant::now();
    let key_cooldown = std::time::Duration::from_millis(200);

    let projection_matrix = create_perspective_matrix(
        45.0_f32.to_radians(),
        WIDTH as f32 / HEIGHT as f32,
        0.1,
        100.0,
    );
    let viewport_matrix = create_viewport_matrix(WIDTH as f32, HEIGHT as f32);

    let start_time = Instant::now();
    let mut last_frame_time = Instant::now();

    println!("\n=== CONTROLES ===");
    println!("1 - Planeta Rocoso");
    println!("2 - Gigante Gaseoso");
    println!("3 - Planeta Cristalino");
    println!("ESC - Salir");
    println!("\nPlaneta actual: {}", current_planet.get_name());

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_start = Instant::now();
        
        // Cambio de shader con teclas
        if last_key_time.elapsed() > key_cooldown {
            if window.is_key_down(Key::Key1) {
                current_planet = PlanetType::Rocky;
                println!("\n→ Cambiado a: {}", current_planet.get_name());
                last_key_time = Instant::now();
            } else if window.is_key_down(Key::Key2) {
                current_planet = PlanetType::GasGiant;
                println!("\n→ Cambiado a: {}", current_planet.get_name());
                last_key_time = Instant::now();
            } else if window.is_key_down(Key::Key3) {
                current_planet = PlanetType::Crystal;
                println!("\n→ Cambiado a: {}", current_planet.get_name());
                last_key_time = Instant::now();
            }
        }

        let delta_time = last_frame_time.elapsed().as_secs_f32();
        last_frame_time = Instant::now();

        camera.orbit(delta_time);

        framebuffer.clear();

        let time = start_time.elapsed().as_secs_f32();
        
        // Rotación suave del planeta
        let model_matrix = create_model_matrix(
            Vec3::new(0.0, 0.0, 0.0),
            1.0,
            Vec3::new(0.0, time * 0.3, 0.0),
        );

        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);

        let uniforms = Uniforms {
            model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
        };

        let shader = current_planet.get_shader();
        render(&mut framebuffer, &uniforms, &vertices, shader);

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();

        // Control de FPS
        let frame_time = frame_start.elapsed().as_millis() as u64;
        if frame_time < FRAME_DELAY {
            std::thread::sleep(std::time::Duration::from_millis(FRAME_DELAY - frame_time));
        }
    }

    println!("\n¡Gracias por usar el renderizador de planetas!");
}
