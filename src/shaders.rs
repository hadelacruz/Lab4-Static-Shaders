use crate::vector::Vector3;
use crate::sphere::Vertex;

#[derive(Debug, Clone, Copy)]
pub struct ShaderColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl ShaderColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        ShaderColor { r, g, b, a }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        ShaderColor {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }

    pub fn to_raylib_color(&self) -> raylib::prelude::Color {
        raylib::prelude::Color {
            r: (self.r * 255.0) as u8,
            g: (self.g * 255.0) as u8,
            b: (self.b * 255.0) as u8,
            a: (self.a * 255.0) as u8,
        }
    }

    pub const WHITE: ShaderColor = ShaderColor { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: ShaderColor = ShaderColor { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const YELLOW: ShaderColor = ShaderColor { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
}

pub struct ShaderUniforms {
    pub time: f32,
    pub light_direction: Vector3,
    pub camera_position: Vector3,
}

pub trait PlanetShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> (Vector3, Vector3);
    fn fragment_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor;
}

// Función de ruido Perlin simplificado mejorado
fn perlin_noise(x: f32, y: f32, z: f32) -> f32 {
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let zi = z.floor() as i32;
    
    let xf = x - x.floor();
    let yf = y - y.floor();
    let zf = z - z.floor();
    
    let u = xf * xf * (3.0 - 2.0 * xf);
    let v = yf * yf * (3.0 - 2.0 * yf);
    let w = zf * zf * (3.0 - 2.0 * zf);
    
    let hash = |x: i32, y: i32, z: i32| -> f32 {
        let n = x.wrapping_add(y.wrapping_mul(57)).wrapping_add(z.wrapping_mul(113));
        let n = (n << 13) ^ n;
        let nn = n.wrapping_mul(n.wrapping_mul(n.wrapping_mul(15731).wrapping_add(789221)).wrapping_add(1376312589));
        1.0 - ((nn & 0x7fffffff) as f32 / 1073741824.0)
    };
    
    let aaa = hash(xi, yi, zi);
    let aba = hash(xi, yi + 1, zi);
    let aab = hash(xi, yi, zi + 1);
    let abb = hash(xi, yi + 1, zi + 1);
    let baa = hash(xi + 1, yi, zi);
    let bba = hash(xi + 1, yi + 1, zi);
    let bab = hash(xi + 1, yi, zi + 1);
    let bbb = hash(xi + 1, yi + 1, zi + 1);
    
    let x1 = mix(aaa, baa, u);
    let x2 = mix(aba, bba, u);
    let x3 = mix(aab, bab, u);
    let x4 = mix(abb, bbb, u);
    
    let y1 = mix(x1, x2, v);
    let y2 = mix(x3, x4, v);
    
    mix(y1, y2, w)
}

// Funciones de ruido mejoradas para efectos procedurales
fn simple_noise(x: f32, y: f32) -> f32 {
    let seed = ((x * 12.9898 + y * 78.233) * 43758.5453).sin().abs();
    (seed * 1000.0).fract()
}

fn fbm(mut x: f32, mut y: f32, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;
    
    for _ in 0..octaves {
        value += amplitude * simple_noise(x, y);
        x *= 2.0;
        y *= 2.0;
        amplitude *= 0.5;
    }
    
    value
}

// Función de ruido 3D mejorado
fn fbm3d(x: f32, y: f32, z: f32, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;
    let mut frequency = 1.0;
    
    for _ in 0..octaves {
        value += amplitude * perlin_noise(x * frequency, y * frequency, z * frequency);
        frequency *= 2.0;
        amplitude *= 0.5;
    }
    
    value
}

// Función de ruido Voronoi para efectos de celdas
fn voronoi_noise(x: f32, y: f32) -> f32 {
    let cell_x = x.floor();
    let cell_y = y.floor();
    let mut min_dist = f32::INFINITY;
    
    for i in -1..=1 {
        for j in -1..=1 {
            let neighbor_x = cell_x + i as f32;
            let neighbor_y = cell_y + j as f32;
            let point_x = neighbor_x + simple_noise(neighbor_x, neighbor_y);
            let point_y = neighbor_y + simple_noise(neighbor_y, neighbor_x);
            
            let dist = ((x - point_x).powi(2) + (y - point_y).powi(2)).sqrt();
            min_dist = min_dist.min(dist);
        }
    }
    
    min_dist
}

// Función de ruido ridge para efectos de montañas
fn ridge_noise(x: f32, y: f32, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;
    let mut frequency = 1.0;
    
    for _ in 0..octaves {
        let n = simple_noise(x * frequency, y * frequency);
        let ridge = 1.0 - (2.0 * n - 1.0).abs();
        value += ridge * amplitude;
        frequency *= 2.0;
        amplitude *= 0.5;
    }
    
    value
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn mix(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

fn mix_color(a: ShaderColor, b: ShaderColor, t: f32) -> ShaderColor {
    ShaderColor::new(
        mix(a.r, b.r, t),
        mix(a.g, b.g, t),
        mix(a.b, b.b, t),
        mix(a.a, b.a, t),
    )
}

// ============================================================================
// PLANETA 1: PLANETA ROCOSO GEOLÓGICO AVANZADO
// Características: Múltiples capas geológicas, cráteres, atmósfera, océanos
// ============================================================================
pub struct RockyPlanetShader;

impl PlanetShader for RockyPlanetShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, _uv: (f32, f32), _uniforms: &ShaderUniforms) -> (Vector3, Vector3) {
        (position, normal)
    }

    fn fragment_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor {
        // === PALETA DE COLORES EXTENDIDA ===
        let ocean_deep = ShaderColor::from_rgb(10, 30, 80);        // Océano profundo
        let ocean_shallow = ShaderColor::from_rgb(30, 80, 150);    // Océano poco profundo
        let beach_sand = ShaderColor::from_rgb(194, 178, 128);     // Arena de playa
        let grassland = ShaderColor::from_rgb(85, 107, 47);        // Pastizal
        let forest = ShaderColor::from_rgb(34, 139, 34);           // Bosque
        let mountain_base = ShaderColor::from_rgb(101, 67, 33);    // Base montañosa
        let mountain_rock = ShaderColor::from_rgb(120, 120, 120);  // Roca montañosa
        let mountain_peak = ShaderColor::from_rgb(240, 240, 255);  // Pico nevado
        let desert_sand = ShaderColor::from_rgb(210, 180, 140);    // Arena desértica
        let volcanic_rock = ShaderColor::from_rgb(60, 40, 30);     // Roca volcánica
        
        // === CAPA 1: ALTURA BASE (Continentes y océanos) ===
        let elevation_noise = fbm3d(
            position.x * 2.0,
            position.y * 2.0,
            position.z * 2.0,
            6
        );
        
        // === CAPA 2: DETALLES GEOLÓGICOS ===
        let geological_detail = fbm(uv.0 * 15.0, uv.1 * 15.0, 5);
        let micro_detail = fbm(uv.0 * 40.0, uv.1 * 40.0, 3);
        
        // === CAPA 3: CRÁTERES ===
        let crater_noise = voronoi_noise(uv.0 * 8.0, uv.1 * 8.0);
        let crater_intensity = smoothstep(0.15, 0.25, crater_noise);
        
        // === CAPA 4: PLACAS TECTÓNICAS ===
        let tectonic_lines = ridge_noise(uv.0 * 6.0, uv.1 * 6.0, 4);
        let tectonic_effect = smoothstep(0.65, 0.75, tectonic_lines);
        
        // === CAPA 5: SISTEMAS CLIMÁTICOS (Nubes) ===
        let cloud_layer1 = fbm(uv.0 * 8.0 + uniforms.time * 0.02, uv.1 * 8.0, 4);
        let cloud_layer2 = fbm(uv.0 * 12.0 - uniforms.time * 0.015, uv.1 * 12.0, 3);
        let cloud_coverage = (cloud_layer1 * 0.6 + cloud_layer2 * 0.4);
        
        // === COMBINACIÓN DE ALTURA ===
        let final_elevation = elevation_noise * 0.6 + geological_detail * 0.3 + micro_detail * 0.1;
        
        // === SELECCIÓN DE BIOMA POR ALTURA ===
        let mut base_color = if final_elevation < 0.25 {
            // Océano profundo
            mix_color(ocean_deep, ocean_shallow, smoothstep(0.15, 0.25, final_elevation))
        } else if final_elevation < 0.35 {
            // Playa
            mix_color(ocean_shallow, beach_sand, smoothstep(0.25, 0.35, final_elevation))
        } else if final_elevation < 0.45 {
            // Pastizales
            mix_color(beach_sand, grassland, smoothstep(0.35, 0.45, final_elevation))
        } else if final_elevation < 0.55 {
            // Bosques
            mix_color(grassland, forest, smoothstep(0.45, 0.55, final_elevation))
        } else if final_elevation < 0.65 {
            // Base montañosa / Desierto
            let is_desert = simple_noise(uv.0 * 5.0, uv.1 * 5.0) > 0.5;
            if is_desert {
                mix_color(forest, desert_sand, smoothstep(0.55, 0.65, final_elevation))
            } else {
                mix_color(forest, mountain_base, smoothstep(0.55, 0.65, final_elevation))
            }
        } else if final_elevation < 0.75 {
            // Montañas rocosas
            mix_color(mountain_base, mountain_rock, smoothstep(0.65, 0.75, final_elevation))
        } else {
            // Picos nevados
            mix_color(mountain_rock, mountain_peak, smoothstep(0.75, 0.85, final_elevation))
        };
        
        // === APLICAR CRÁTERES ===
        if crater_intensity > 0.5 && final_elevation > 0.35 {
            base_color = mix_color(base_color, volcanic_rock, (1.0 - crater_intensity) * 0.6);
        }
        
        // === APLICAR LÍNEAS TECTÓNICAS ===
        if tectonic_effect > 0.7 {
            base_color = mix_color(base_color, volcanic_rock, tectonic_effect * 0.4);
        }
        
        // === APLICAR NUBES (solo sobre tierra) ===
        let cloud_white = ShaderColor::from_rgb(255, 255, 255);
        if cloud_coverage > 0.55 && final_elevation > 0.35 {
            let cloud_alpha = smoothstep(0.55, 0.7, cloud_coverage) * 0.7;
            base_color = mix_color(base_color, cloud_white, cloud_alpha);
        }
        
        // === ILUMINACIÓN AVANZADA ===
        let light_dir = uniforms.light_direction.normalize();
        let view_dir = (uniforms.camera_position - position).normalize();
        
        // Difusa básica
        let diffuse = normal.dot(&light_dir).max(0.0);
        
        // Especular para océanos
        let is_ocean = final_elevation < 0.35;
        let specular = if is_ocean {
            let reflect_dir = normal * (2.0 * normal.dot(&light_dir)) - light_dir;
            view_dir.dot(&reflect_dir).max(0.0).powf(20.0) * 0.8
        } else {
            0.0
        };
        
        // Rim lighting (atmósfera)
        let rim = (1.0 - view_dir.dot(&normal).abs()).powf(3.0) * 0.4;
        let atmosphere_color = ShaderColor::from_rgb(135, 206, 250); // Azul cielo
        
        // Sombras suaves
        let shadow = smoothstep(-0.1, 0.1, diffuse);
        
        let ambient = 0.25;
        let lighting_intensity = (ambient + diffuse * 0.6 * shadow + specular).min(1.3);
        
        // Color final con iluminación
        let mut final_color = ShaderColor::new(
            base_color.r * lighting_intensity,
            base_color.g * lighting_intensity,
            base_color.b * lighting_intensity,
            1.0,
        );
        
        // Añadir atmósfera en los bordes
        if rim > 0.3 {
            final_color = mix_color(final_color, atmosphere_color, rim * 0.5);
        }
        
        ShaderColor::new(
            final_color.r.clamp(0.0, 1.0),
            final_color.g.clamp(0.0, 1.0),
            final_color.b.clamp(0.0, 1.0),
            1.0,
        )
    }
}

// ============================================================================
// PLANETA 2: GIGANTE GASEOSO CON TORMENTAS DINÁMICAS
// Características: Bandas atmosféricas, vórtices, tormentas, turbulencia
// ============================================================================
pub struct GasGiantShader;

impl PlanetShader for GasGiantShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, _uv: (f32, f32), _uniforms: &ShaderUniforms) -> (Vector3, Vector3) {
        (position, normal)
    }

    fn fragment_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor {
        // === PALETA DE COLORES JUPITERIANA ===
        let deep_red = ShaderColor::from_rgb(139, 0, 0);           // Rojo profundo
        let brick_red = ShaderColor::from_rgb(178, 34, 34);        // Rojo ladrillo
        let orange_red = ShaderColor::from_rgb(255, 69, 0);        // Naranja rojizo
        let orange = ShaderColor::from_rgb(255, 140, 0);           // Naranja
        let gold = ShaderColor::from_rgb(255, 215, 0);             // Dorado
        let cream = ShaderColor::from_rgb(255, 248, 220);          // Crema
        let pale_yellow = ShaderColor::from_rgb(255, 255, 200);    // Amarillo pálido
        let white_cream = ShaderColor::from_rgb(255, 250, 240);    // Blanco cremoso
        let storm_white = ShaderColor::from_rgb(245, 245, 255);    // Blanco tormenta
        
        // === CAPA 1: BANDAS ATMOSFÉRICAS PRINCIPALES ===
        let band_frequency = 10.0;
        let band_offset = uniforms.time * 0.03; // Rotación diferencial
        let latitude = uv.1;
        
        // Diferentes velocidades por banda (rotación diferencial)
        let band_speed = (latitude * band_frequency).sin() * 0.02 + 0.01;
        let band_position = (latitude * band_frequency + band_offset * band_speed).sin();
        
        // === CAPA 2: TURBULENCIA ATMOSFÉRICA ===
        let turbulence1 = fbm(
            uv.0 * 8.0 + uniforms.time * 0.04,
            uv.1 * 6.0,
            6
        );
        let turbulence2 = fbm(
            uv.0 * 12.0 - uniforms.time * 0.03,
            uv.1 * 8.0,
            5
        );
        let turbulence3 = fbm(
            uv.0 * 20.0 + uniforms.time * 0.02,
            uv.1 * 15.0,
            4
        );
        
        let combined_turbulence = turbulence1 * 0.5 + turbulence2 * 0.3 + turbulence3 * 0.2;
        
        // === CAPA 3: VÓRTICES Y TORMENTAS ===
        let storm_centers = voronoi_noise(uv.0 * 6.0, uv.1 * 6.0);
        let vortex_pattern = fbm(
            uv.0 * 15.0 + uniforms.time * 0.1,
            uv.1 * 15.0 - uniforms.time * 0.08,
            4
        );
        
        // Gran Mancha Roja simulada
        let red_spot_x = 0.3 + (uniforms.time * 0.01).sin() * 0.05;
        let red_spot_y = 0.6;
        let dist_to_spot = ((uv.0 - red_spot_x).powi(2) + (uv.1 - red_spot_y).powi(2)).sqrt();
        let red_spot_influence = smoothstep(0.15, 0.05, dist_to_spot);
        
        // Vórtices secundarios
        let vortex_strength = smoothstep(0.25, 0.15, storm_centers) * vortex_pattern;
        
        // === CAPA 4: CORRIENTES DE CHORRO ===
        let jet_stream = ridge_noise(uv.0 * 25.0 + uniforms.time * 0.15, uv.1 * 3.0, 3);
        let jet_effect = smoothstep(0.7, 0.85, jet_stream);
        
        // === CONSTRUCCIÓN DEL COLOR BASE ===
        let distorted_band = band_position + combined_turbulence * 0.8;
        
        // Selección de banda por latitud
        let mut base_color = if distorted_band > 0.8 {
            mix_color(white_cream, storm_white, smoothstep(0.8, 1.0, distorted_band))
        } else if distorted_band > 0.5 {
            mix_color(cream, white_cream, smoothstep(0.5, 0.8, distorted_band))
        } else if distorted_band > 0.2 {
            mix_color(gold, cream, smoothstep(0.2, 0.5, distorted_band))
        } else if distorted_band > -0.1 {
            mix_color(orange, gold, smoothstep(-0.1, 0.2, distorted_band))
        } else if distorted_band > -0.4 {
            mix_color(orange_red, orange, smoothstep(-0.4, -0.1, distorted_band))
        } else if distorted_band > -0.6 {
            mix_color(brick_red, orange_red, smoothstep(-0.6, -0.4, distorted_band))
        } else {
            mix_color(deep_red, brick_red, smoothstep(-0.8, -0.6, distorted_band))
        };
        
        // === APLICAR GRAN MANCHA ROJA ===
        if red_spot_influence > 0.1 {
            let spot_rotation = (uniforms.time * 0.5 + dist_to_spot * 10.0).sin() * 0.5 + 0.5;
            let spot_color = mix_color(deep_red, brick_red, spot_rotation);
            base_color = mix_color(base_color, spot_color, red_spot_influence * 0.9);
        }
        
        // === APLICAR VÓRTICES ===
        if vortex_strength > 0.6 {
            let vortex_color = mix_color(pale_yellow, storm_white, vortex_strength);
            base_color = mix_color(base_color, vortex_color, vortex_strength * 0.6);
        }
        
        // === APLICAR CORRIENTES DE CHORRO ===
        if jet_effect > 0.5 {
            base_color = mix_color(base_color, pale_yellow, jet_effect * 0.4);
        }
        
        // === CAPA 5: NUBES DE ALTA ALTITUD ===
        let high_clouds = fbm(
            uv.0 * 18.0 + uniforms.time * 0.08,
            uv.1 * 12.0,
            4
        );
        if high_clouds > 0.65 {
            let cloud_intensity = smoothstep(0.65, 0.8, high_clouds);
            base_color = mix_color(base_color, storm_white, cloud_intensity * 0.5);
        }
        
        // === ILUMINACIÓN AVANZADA ===
        let light_dir = uniforms.light_direction.normalize();
        let view_dir = (uniforms.camera_position - position).normalize();
        
        // Difusa con envolvimiento suave (atmósfera densa)
        let wrap_diffuse = (normal.dot(&light_dir) * 0.5 + 0.5).max(0.0);
        
        // Scattering atmosférico
        let scatter = (1.0 - normal.dot(&light_dir).abs()).powf(2.0) * 0.3;
        
        // Rim lighting (atmósfera brillante)
        let rim = (1.0 - view_dir.dot(&normal).abs()).powf(2.5);
        let rim_intensity = rim * 0.6;
        
        // Auto-iluminación de tormentas
        let storm_glow = (vortex_strength * 0.3 + red_spot_influence * 0.2);
        
        let ambient = 0.3;
        let lighting_intensity = (ambient + wrap_diffuse * 0.5 + scatter + storm_glow).min(1.5);
        
        let mut final_color = ShaderColor::new(
            base_color.r * lighting_intensity,
            base_color.g * lighting_intensity,
            base_color.b * lighting_intensity,
            1.0,
        );
        
        // Añadir brillo atmosférico en los bordes
        if rim_intensity > 0.3 {
            let atmosphere_glow = ShaderColor::from_rgb(255, 220, 180);
            final_color = mix_color(final_color, atmosphere_glow, rim_intensity * 0.5);
        }
        
        ShaderColor::new(
            final_color.r.clamp(0.0, 1.0),
            final_color.g.clamp(0.0, 1.0),
            final_color.b.clamp(0.0, 1.0),
            1.0,
        )
    }
}

// ============================================================================
// PLANETA 3: PLANETA SCI-FI TECNOLÓGICO
// Características: Líneas de energía, circuitos, pulsos tecnológicos, hologramas
// ============================================================================
pub struct CrystalPlanetShader;

impl PlanetShader for CrystalPlanetShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, _uv: (f32, f32), uniforms: &ShaderUniforms) -> (Vector3, Vector3) {
        // Deformación de pulso de energía
        let pulse = (uniforms.time * 3.0 + position.length() * 5.0).sin() * 0.01;
        let pulsed_position = position + normal * pulse;
        (pulsed_position, normal)
    }

    fn fragment_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor {
        // === PALETA TECNOLÓGICA ===
        let base_dark = ShaderColor::from_rgb(10, 15, 30);         // Base oscura
        let tech_blue = ShaderColor::from_rgb(0, 150, 255);        // Azul tecnológico
        let cyber_cyan = ShaderColor::from_rgb(0, 255, 255);       // Cian cibernético
        let neon_green = ShaderColor::from_rgb(0, 255, 150);       // Verde neón
        let electric_purple = ShaderColor::from_rgb(150, 0, 255);  // Púrpura eléctrico
        let hot_pink = ShaderColor::from_rgb(255, 0, 150);         // Rosa caliente
        let energy_white = ShaderColor::from_rgb(200, 255, 255);   // Blanco energético
        let warning_orange = ShaderColor::from_rgb(255, 150, 0);   // Naranja advertencia
        
        // === CAPA 1: GRILLA TECNOLÓGICA BASE ===
        let grid_size = 20.0;
        let grid_x = (uv.0 * grid_size).fract();
        let grid_y = (uv.1 * grid_size).fract();
        let grid_lines = smoothstep(0.02, 0.0, grid_x.min(1.0 - grid_x)) +
                        smoothstep(0.02, 0.0, grid_y.min(1.0 - grid_y));
        
        // === CAPA 2: CIRCUITOS HEXAGONALES ===
        let hex_pattern = voronoi_noise(uv.0 * 12.0, uv.1 * 12.0);
        let hex_cells = smoothstep(0.15, 0.2, hex_pattern);
        let hex_borders = smoothstep(0.18, 0.22, hex_pattern) - smoothstep(0.22, 0.25, hex_pattern);
        
        // === CAPA 3: FLUJO DE DATOS ===
        let data_flow1 = fbm(
            uv.0 * 15.0 + uniforms.time * 0.5,
            uv.1 * 15.0,
            4
        );
        let data_flow2 = fbm(
            uv.0 * 20.0 - uniforms.time * 0.7,
            uv.1 * 10.0 + uniforms.time * 0.3,
            3
        );
        let data_streams = smoothstep(0.6, 0.8, data_flow1) + smoothstep(0.65, 0.85, data_flow2);
        
        // === CAPA 4: PULSOS DE ENERGÍA ===
        let pulse_frequency = 5.0;
        let pulse_wave = (uniforms.time * pulse_frequency + position.length() * 3.0).sin() * 0.5 + 0.5;
        let pulse_wave2 = (uniforms.time * pulse_frequency * 1.5 - position.length() * 2.0).sin() * 0.5 + 0.5;
        let energy_pulse = pulse_wave * 0.6 + pulse_wave2 * 0.4;
        
        // === CAPA 5: NODOS DE PODER ===
        let power_nodes = voronoi_noise(uv.0 * 8.0, uv.1 * 8.0);
        let node_centers = smoothstep(0.08, 0.05, power_nodes);
        let node_glow = smoothstep(0.15, 0.05, power_nodes);
        
        // === CAPA 6: ESCANEO HOLOGRÁFICO ===
        let scan_line = ((uv.1 * 10.0 - uniforms.time * 2.0) % 1.0);
        let scan_intensity = smoothstep(0.05, 0.0, (scan_line - 0.5).abs());
        
        // === CAPA 7: INTERFERENCIA DIGITAL ===
        let glitch = simple_noise(
            (uniforms.time * 10.0).floor() * 0.1,
            (uv.1 * 20.0).floor()
        );
        let glitch_effect = if glitch > 0.95 {
            simple_noise(uv.0 * 100.0 + uniforms.time * 50.0, uv.1) * 0.3
        } else {
            0.0
        };
        
        // === CONSTRUCCIÓN DEL COLOR ===
        let mut base_color = base_dark;
        
        // Agregar grilla base
        if grid_lines > 0.1 {
            base_color = mix_color(base_color, tech_blue, grid_lines * energy_pulse * 0.5);
        }
        
        // Agregar circuitos hexagonales
        if hex_borders > 0.1 {
            let circuit_color = mix_color(cyber_cyan, tech_blue, energy_pulse);
            base_color = mix_color(base_color, circuit_color, hex_borders * 0.8);
        }
        
        // Celdas hexagonales con variación de color
        if hex_cells > 0.5 {
            let cell_variety = simple_noise(uv.0 * 12.0, uv.1 * 12.0);
            let cell_color = if cell_variety > 0.7 {
                electric_purple
            } else if cell_variety > 0.4 {
                neon_green
            } else {
                tech_blue
            };
            base_color = mix_color(base_color, cell_color, hex_cells * 0.3 * energy_pulse);
        }
        
        // Agregar flujo de datos
        if data_streams > 0.5 {
            let stream_color = mix_color(neon_green, cyber_cyan, (uniforms.time * 2.0).sin() * 0.5 + 0.5);
            base_color = mix_color(base_color, stream_color, data_streams * 0.7);
        }
        
        // Agregar nodos de poder
        if node_centers > 0.5 {
            let node_pulse = ((uniforms.time * 4.0 + uv.0 * 20.0).sin() * 0.5 + 0.5);
            let node_color = mix_color(hot_pink, energy_white, node_pulse);
            base_color = mix_color(base_color, node_color, node_centers);
        }
        
        if node_glow > 0.3 {
            base_color = mix_color(base_color, warning_orange, node_glow * 0.4 * energy_pulse);
        }
        
        // Agregar línea de escaneo
        if scan_intensity > 0.1 {
            base_color = mix_color(base_color, energy_white, scan_intensity * 0.8);
        }
        
        // Efecto de glitch
        if glitch_effect > 0.1 {
            let glitch_color = mix_color(hot_pink, cyber_cyan, glitch);
            base_color = mix_color(base_color, glitch_color, glitch_effect);
        }
        
        // === CAPA 8: PATRONES FRACTALES ===
        let fractal = fbm3d(
            position.x * 10.0 + uniforms.time * 0.1,
            position.y * 10.0,
            position.z * 10.0 - uniforms.time * 0.15,
            5
        );
        if fractal > 0.6 {
            let fractal_color = mix_color(electric_purple, tech_blue, fractal);
            base_color = mix_color(base_color, fractal_color, smoothstep(0.6, 0.75, fractal) * 0.4);
        }
        
        // === ILUMINACIÓN TECNOLÓGICA ===
        let light_dir = uniforms.light_direction.normalize();
        let view_dir = (uniforms.camera_position - position).normalize();
        
        // Iluminación mínima (el planeta se auto-ilumina)
        let diffuse = normal.dot(&light_dir).max(0.0) * 0.2;
        
        // Rim lighting holográfico
        let rim = (1.0 - view_dir.dot(&normal).abs()).powf(3.0);
        let rim_color = mix_color(cyber_cyan, hot_pink, (uniforms.time * 2.0).sin() * 0.5 + 0.5);
        
        // Auto-iluminación
        let self_illumination = 0.8 + energy_pulse * 0.3;
        
        let ambient = 0.2;
        let lighting_intensity = (ambient + diffuse + self_illumination).min(2.0);
        
        let mut final_color = ShaderColor::new(
            base_color.r * lighting_intensity,
            base_color.g * lighting_intensity,
            base_color.b * lighting_intensity,
            1.0,
        );
        
        // Añadir rim lighting
        if rim > 0.4 {
            final_color = mix_color(final_color, rim_color, rim * 0.8);
        }
        
        ShaderColor::new(
            final_color.r.clamp(0.0, 1.0),
            final_color.g.clamp(0.0, 1.0),
            final_color.b.clamp(0.0, 1.0),
            1.0,
        )
    }
}

// ============================================================================
// PLANETA 4: NEBULOSA ETÉREA CÓSMICA
// Características: Gas etéreo, partículas estelares, efectos volumétricos
// ============================================================================
pub struct LavaPlanetShader;

impl PlanetShader for LavaPlanetShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, _uv: (f32, f32), uniforms: &ShaderUniforms) -> (Vector3, Vector3) {
        // Ondulación suave como gas etéreo
        let wave1 = (uniforms.time * 1.5 + position.x * 3.0 + position.y * 2.0).sin() * 0.03;
        let wave2 = (uniforms.time * 2.0 - position.z * 4.0 + position.y).cos() * 0.02;
        let wavy_position = position + normal * (wave1 + wave2);
        (wavy_position, normal)
    }

    fn fragment_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor {
        // === PALETA NEBULOSA CÓSMICA ===
        let void_black = ShaderColor::from_rgb(5, 0, 10);          // Vacío espacial
        let deep_purple = ShaderColor::from_rgb(30, 0, 60);        // Púrpura profundo
        let royal_purple = ShaderColor::from_rgb(75, 0, 130);      // Púrpura real
        let magenta = ShaderColor::from_rgb(200, 0, 150);          // Magenta brillante
        let hot_pink = ShaderColor::from_rgb(255, 20, 147);        // Rosa intenso
        let electric_blue = ShaderColor::from_rgb(0, 100, 255);    // Azul eléctrico
        let cyan_bright = ShaderColor::from_rgb(0, 255, 255);      // Cian brillante
        let orange_flame = ShaderColor::from_rgb(255, 100, 0);     // Naranja fuego
        let yellow_star = ShaderColor::from_rgb(255, 255, 100);    // Amarillo estelar
        let white_hot = ShaderColor::from_rgb(255, 255, 255);      // Blanco caliente
        
        // === CAPA 1: GAS NEBULAR BASE (Movimiento lento y fluido) ===
        let nebula_gas1 = fbm3d(
            position.x * 2.0 + uniforms.time * 0.03,
            position.y * 2.0 + uniforms.time * 0.02,
            position.z * 2.0 - uniforms.time * 0.025,
            7
        );
        let nebula_gas2 = fbm3d(
            position.x * 3.0 - uniforms.time * 0.02,
            position.y * 3.0 + uniforms.time * 0.035,
            position.z * 3.0 + uniforms.time * 0.015,
            6
        );
        let nebula_density = nebula_gas1 * 0.6 + nebula_gas2 * 0.4;
        
        // === CAPA 2: REMOLINOS DE POLVO CÓSMICO ===
        let cosmic_dust1 = fbm(
            uv.0 * 8.0 + uniforms.time * 0.05,
            uv.1 * 8.0 - uniforms.time * 0.04,
            5
        );
        let cosmic_dust2 = fbm(
            uv.0 * 12.0 - uniforms.time * 0.03,
            uv.1 * 12.0 + uniforms.time * 0.06,
            4
        );
        let dust_swirls = cosmic_dust1 * 0.5 + cosmic_dust2 * 0.5;
        
        // === CAPA 3: CAMPOS DE IONIZACIÓN ===
        let ionization1 = fbm3d(
            position.x * 5.0 + uniforms.time * 0.1,
            position.y * 5.0,
            position.z * 5.0 - uniforms.time * 0.08,
            4
        );
        let ionization2 = fbm3d(
            position.x * 7.0 - uniforms.time * 0.12,
            position.y * 7.0 + uniforms.time * 0.09,
            position.z * 7.0,
            3
        );
        let ion_fields = ionization1 * 0.6 + ionization2 * 0.4;
        
        // === CAPA 4: VÓRTICES MAGNÉTICOS ===
        let vortex_pattern = voronoi_noise(
            uv.0 * 6.0 + uniforms.time * 0.04,
            uv.1 * 6.0 - uniforms.time * 0.035
        );
        let vortex_intensity = smoothstep(0.2, 0.1, vortex_pattern);
        
        // === CAPA 5: RAYOS CÓSMICOS Y RADIACIÓN ===
        let cosmic_rays = ridge_noise(
            uv.0 * 20.0 + uniforms.time * 0.3,
            uv.1 * 20.0 - uniforms.time * 0.25,
            3
        );
        let ray_intensity = smoothstep(0.75, 0.9, cosmic_rays);
        
        // === CAPA 6: ESTRELLAS EN FORMACIÓN ===
        let star_formation = voronoi_noise(uv.0 * 15.0, uv.1 * 15.0);
        let proto_stars = smoothstep(0.05, 0.02, star_formation);
        let star_glow = smoothstep(0.12, 0.02, star_formation);
        
        // === CAPA 7: PULSOS DE ENERGÍA CÓSMICA ===
        let energy_pulse1 = ((uniforms.time * 2.0 + position.length() * 3.0).sin() * 0.5 + 0.5);
        let energy_pulse2 = ((uniforms.time * 3.0 - position.length() * 2.0).cos() * 0.5 + 0.5);
        let cosmic_pulse = energy_pulse1 * 0.6 + energy_pulse2 * 0.4;
        
        // === CAPA 8: ONDAS DE CHOQUE ===
        let shockwave_distance = ((position.x + uniforms.time * 0.5).powi(2) + 
                                  position.y.powi(2) + 
                                  position.z.powi(2)).sqrt();
        let shockwave = ((shockwave_distance * 5.0 - uniforms.time * 3.0).sin() * 0.5 + 0.5);
        let shockwave_intensity = smoothstep(0.7, 0.9, shockwave) * smoothstep(0.3, 0.4, nebula_density);
        
        // === CONSTRUCCIÓN DEL COLOR BASE ===
        let mut base_color = void_black;
        
        // Gradiente de gas nebular
        if nebula_density > 0.2 {
            let gas_color = if nebula_density > 0.7 {
                mix_color(royal_purple, magenta, smoothstep(0.7, 0.85, nebula_density))
            } else if nebula_density > 0.5 {
                mix_color(deep_purple, royal_purple, smoothstep(0.5, 0.7, nebula_density))
            } else {
                mix_color(void_black, deep_purple, smoothstep(0.2, 0.5, nebula_density))
            };
            base_color = mix_color(base_color, gas_color, 0.9);
        }
        
        // Aplicar remolinos de polvo con colores naranjas
        if dust_swirls > 0.55 {
            let dust_color = mix_color(orange_flame, yellow_star, smoothstep(0.55, 0.75, dust_swirls));
            let dust_alpha = smoothstep(0.55, 0.7, dust_swirls) * 0.6;
            base_color = mix_color(base_color, dust_color, dust_alpha);
        }
        
        // Campos de ionización (azul eléctrico)
        if ion_fields > 0.6 {
            let ion_color = mix_color(electric_blue, cyan_bright, cosmic_pulse);
            let ion_alpha = smoothstep(0.6, 0.75, ion_fields) * 0.7;
            base_color = mix_color(base_color, ion_color, ion_alpha);
        }
        
        // Vórtices magnéticos (rosa intenso)
        if vortex_intensity > 0.4 {
            let vortex_color = mix_color(hot_pink, magenta, cosmic_pulse);
            base_color = mix_color(base_color, vortex_color, vortex_intensity * 0.8);
        }
        
        // Rayos cósmicos brillantes
        if ray_intensity > 0.5 {
            let ray_color = mix_color(cyan_bright, white_hot, energy_pulse1);
            base_color = mix_color(base_color, ray_color, ray_intensity * 0.9);
        }
        
        // Estrellas en formación (muy brillantes)
        if proto_stars > 0.6 {
            let star_pulse = ((uniforms.time * 5.0 + uv.0 * 50.0).sin() * 0.5 + 0.5);
            let star_color = mix_color(yellow_star, white_hot, star_pulse);
            base_color = mix_color(base_color, star_color, proto_stars);
        }
        
        // Resplandor de estrellas
        if star_glow > 0.3 && star_glow < 0.7 {
            let glow_color = mix_color(orange_flame, yellow_star, cosmic_pulse);
            base_color = mix_color(base_color, glow_color, star_glow * 0.5);
        }
        
        // Ondas de choque
        if shockwave_intensity > 0.5 {
            let shock_color = mix_color(cyan_bright, electric_blue, shockwave);
            base_color = mix_color(base_color, shock_color, shockwave_intensity * 0.7);
        }
        
        // === CAPA 9: BRILLO VOLUMÉTRICO ===
        let volumetric_glow = nebula_density * 0.5 + dust_swirls * 0.3 + ion_fields * 0.2;
        
        // === CAPA 10: PARTÍCULAS ESTELARES ===
        let particles = simple_noise(uv.0 * 100.0, uv.1 * 100.0);
        if particles > 0.98 {
            let particle_brightness = simple_noise(uv.0 * 200.0 + uniforms.time, uv.1 * 200.0);
            let particle_color = mix_color(yellow_star, white_hot, particle_brightness);
            base_color = mix_color(base_color, particle_color, (particles - 0.98) * 50.0);
        }
        
        // === ILUMINACIÓN VOLUMÉTRICA ===
        let light_dir = uniforms.light_direction.normalize();
        let view_dir = (uniforms.camera_position - position).normalize();
        
        // Muy poca iluminación direccional (luz propia)
        let diffuse = normal.dot(&light_dir).max(0.0) * 0.1;
        
        // Auto-iluminación fuerte (la nebulosa brilla por sí misma)
        let self_illumination = 1.2 + cosmic_pulse * 0.5 + volumetric_glow * 0.8;
        
        // Rim lighting etéreo
        let rim = (1.0 - view_dir.dot(&normal).abs()).powf(2.0);
        let rim_color = mix_color(
            mix_color(magenta, cyan_bright, energy_pulse1),
            hot_pink,
            energy_pulse2
        );
        
        // Scattering interno
        let internal_scatter = (1.0 - nebula_density.abs()) * 0.3;
        
        let ambient = 0.1;
        let lighting_intensity = (ambient + diffuse + self_illumination + internal_scatter).min(2.5);
        
        let mut final_color = ShaderColor::new(
            base_color.r * lighting_intensity,
            base_color.g * lighting_intensity,
            base_color.b * lighting_intensity,
            1.0,
        );
        
        // Añadir rim lighting etéreo
        if rim > 0.3 {
            final_color = mix_color(final_color, rim_color, rim * 0.7);
        }
        
        // Bloom effect simulado
        if volumetric_glow > 0.6 {
            let bloom_intensity = smoothstep(0.6, 0.8, volumetric_glow) * 0.3;
            let bloom_color = ShaderColor::from_rgb(255, 200, 255);
            final_color = mix_color(final_color, bloom_color, bloom_intensity);
        }
        
        ShaderColor::new(
            final_color.r.clamp(0.0, 1.0),
            final_color.g.clamp(0.0, 1.0),
            final_color.b.clamp(0.0, 1.0),
            1.0,
        )
    }
}

