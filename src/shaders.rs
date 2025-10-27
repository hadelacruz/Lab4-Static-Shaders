use crate::vector::Vector3;

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
// PLANETA 1: PLANETA ROCOSO CON DEFORMACIÓN PROCEDURAL (VERTEX SHADER)
// Características: Deformación geométrica procedural, terreno con relieve, colores grises
// ============================================================================
pub struct RockyPlanetShader;

impl PlanetShader for RockyPlanetShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, _uv: (f32, f32), uniforms: &ShaderUniforms) -> (Vector3, Vector3) {
        // === DEFORMACIÓN PROCEDURAL DEL TERRENO ===
        
        // Capa 1: Montañas grandes (escala global)
        let mountain_noise = fbm3d(
            position.x * 2.0,
            position.y * 2.0,
            position.z * 2.0,
            4
        ) * 0.15;
        
        // Capa 2: Colinas medianas
        let hill_noise = fbm3d(
            position.x * 5.0,
            position.y * 5.0,
            position.z * 5.0,
            3
        ) * 0.08;
        
        // Capa 3: Detalles finos (rocas pequeñas)
        let detail_noise = fbm3d(
            position.x * 15.0,
            position.y * 15.0,
            position.z * 15.0,
            2
        ) * 0.03;
        
        // Capa 4: Cráteres procedurales
        let crater_x = position.x * 8.0;
        let crater_y = position.y * 8.0;
        let crater_pattern = voronoi_noise(crater_x, crater_y);
        let crater_depth = if crater_pattern < 0.2 {
            -0.05 * (1.0 - crater_pattern / 0.2)
        } else {
            0.0
        };
        
        // Capa 5: Animación sutil (pulso tectónico)
        let tectonic_pulse = (uniforms.time * 0.5).sin() * 0.01;
        let pulse_noise = fbm3d(
            position.x * 3.0 + uniforms.time * 0.1,
            position.y * 3.0,
            position.z * 3.0,
            2
        ) * tectonic_pulse;
        
        // Combinar todas las deformaciones
        let total_displacement = mountain_noise + hill_noise + detail_noise + crater_depth + pulse_noise;
        
        // Aplicar deformación a lo largo de la normal
        let deformed_position = Vector3::new(
            position.x + normal.x * total_displacement,
            position.y + normal.y * total_displacement,
            position.z + normal.z * total_displacement,
        );
        
        // Recalcular normal aproximada basada en deformación
        let epsilon = 0.01;
        let neighbor_noise = fbm3d(
            (position.x + epsilon) * 2.0,
            (position.y + epsilon) * 2.0,
            (position.z + epsilon) * 2.0,
            4
        ) * 0.15;
        
        let tangent_displacement = neighbor_noise - mountain_noise;
        let normal_perturbation = Vector3::new(
            -tangent_displacement,
            -tangent_displacement,
            -tangent_displacement,
        ) * 0.3;
        
        let perturbed_normal = Vector3::new(
            normal.x + normal_perturbation.x,
            normal.y + normal_perturbation.y,
            normal.z + normal_perturbation.z,
        ).normalize();
        
        (deformed_position, perturbed_normal)
    }

    fn fragment_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor {
        // === PALETA DE COLORES GRISES (NO CAMBIA CON EL TIEMPO) ===
        let darkest_gray = ShaderColor::from_rgb(20, 20, 25);      // Gris casi negro
        let dark_gray = ShaderColor::from_rgb(50, 50, 55);         // Gris oscuro
        let medium_dark = ShaderColor::from_rgb(80, 80, 85);       // Gris medio-oscuro
        let medium_gray = ShaderColor::from_rgb(110, 110, 115);    // Gris medio
        let light_gray = ShaderColor::from_rgb(140, 140, 145);     // Gris claro
        let lighter_gray = ShaderColor::from_rgb(170, 170, 175);   // Gris más claro
        let _lightest_gray = ShaderColor::from_rgb(200, 200, 205);  // Gris casi blanco (reservado)
        let lightest_gray = ShaderColor::from_rgb(200, 200, 205);  // Gris casi blanco
        
        // === CAPA 1: TEXTURA BASE (Variación de rocas) ===
        let rock_variation = fbm3d(
            position.x * 8.0,
            position.y * 8.0,
            position.z * 8.0,
            5
        );
        
        // === CAPA 2: DETALLES GEOLÓGICOS (Erosión y fracturas) ===
        let erosion = fbm(uv.0 * 20.0, uv.1 * 20.0, 4);
        let fractures = ridge_noise(uv.0 * 15.0, uv.1 * 15.0, 3);
        
        // === CAPA 3: CRÁTERES (Oscurecimiento) ===
        let crater_noise = voronoi_noise(uv.0 * 8.0, uv.1 * 8.0);
        let is_crater = crater_noise < 0.2;
        
        // === CAPA 4: VETAS MINERALES (Líneas más claras) ===
        let mineral_veins = ridge_noise(uv.0 * 25.0, uv.1 * 25.0, 2);
        let has_veins = mineral_veins > 0.75;
        
        // === SELECCIÓN DE COLOR BASE (Solo grises, NO cambia) ===
        let base_color = if rock_variation < 0.2 {
            mix_color(darkest_gray, dark_gray, smoothstep(0.0, 0.2, rock_variation))
        } else if rock_variation < 0.4 {
            mix_color(dark_gray, medium_dark, smoothstep(0.2, 0.4, rock_variation))
        } else if rock_variation < 0.6 {
            mix_color(medium_dark, medium_gray, smoothstep(0.4, 0.6, rock_variation))
        } else if rock_variation < 0.75 {
            mix_color(medium_gray, light_gray, smoothstep(0.6, 0.75, rock_variation))
        } else if rock_variation < 0.85 {
            mix_color(light_gray, lighter_gray, smoothstep(0.75, 0.85, rock_variation))
        } else {
            mix_color(lighter_gray, lightest_gray, smoothstep(0.85, 1.0, rock_variation))
        };
        
        let mut final_base = base_color;
        
        // === APLICAR CRÁTERES (Oscurecer) ===
        if is_crater {
            final_base = mix_color(final_base, darkest_gray, 0.6);
        }
        
        // === APLICAR EROSIÓN (Variación sutil) ===
        if erosion > 0.6 {
            let erosion_factor = smoothstep(0.6, 0.8, erosion);
            final_base = mix_color(final_base, dark_gray, erosion_factor * 0.3);
        }
        
        // === APLICAR FRACTURAS (Oscurecer líneas) ===
        if fractures > 0.7 {
            let fracture_intensity = smoothstep(0.7, 0.85, fractures);
            final_base = mix_color(final_base, darkest_gray, fracture_intensity * 0.5);
        }
        
        // === APLICAR VETAS MINERALES (Aclarar líneas) ===
        if has_veins {
            let vein_intensity = smoothstep(0.75, 0.9, mineral_veins);
            final_base = mix_color(final_base, lightest_gray, vein_intensity * 0.4);
        }
        // === APLICAR VETAS MINERALES (Aclarar líneas) ===
        if has_veins {
            let vein_intensity = smoothstep(0.75, 0.9, mineral_veins);
            final_base = mix_color(final_base, lightest_gray, vein_intensity * 0.4);
        }
        
        // === ILUMINACIÓN (Sin colores, solo intensidad) ===
        let light_dir = uniforms.light_direction.normalize();
        let view_dir = (uniforms.camera_position - position).normalize();
        
        // Difusa básica
        let diffuse = normal.dot(&light_dir).max(0.0);
        
        // Especular suave para rocas
        let reflect_dir = normal * (2.0 * normal.dot(&light_dir)) - light_dir;
        let specular = view_dir.dot(&reflect_dir).max(0.0).powf(8.0) * 0.2;
        
        // Oclusión ambiental basada en curvatura
        let ambient_occlusion = (1.0 - erosion * 0.3).max(0.3);
        
        let ambient = 0.2;
        let lighting_intensity = (ambient * ambient_occlusion + diffuse * 0.7 + specular).min(1.2);
        
        // Color final (SOLO GRISES, sin cambios de color)
        let final_color = ShaderColor::new(
            final_base.r * lighting_intensity,
            final_base.g * lighting_intensity,
            final_base.b * lighting_intensity,
            1.0,
        );
        
        ShaderColor::new(
            final_color.r.clamp(0.0, 1.0),
            final_color.g.clamp(0.0, 1.0),
            final_color.b.clamp(0.0, 1.0),
            1.0,
        )
    }
}

// ============================================================================
// PLANETA 2: GIGANTE GASEOSO ESTILO JÚPITER
// Características: Bandas horizontales con MUCHA TEXTURA y colores variados
// ============================================================================
pub struct GasGiantShader;

impl PlanetShader for GasGiantShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, _uv: (f32, f32), _uniforms: &ShaderUniforms) -> (Vector3, Vector3) {
        (position, normal)
    }

    fn fragment_shader(&self, _position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor {
        // === PALETA DE JÚPITER - MUY CONTRASTADA ===
        let very_dark = ShaderColor::from_rgb(60, 35, 15);         // Casi negro
        let dark_brown = ShaderColor::from_rgb(110, 65, 25);       // Marrón oscuro
        let rust_brown = ShaderColor::from_rgb(150, 85, 35);       // Óxido
        let orange = ShaderColor::from_rgb(200, 120, 50);          // Naranja
        let tan = ShaderColor::from_rgb(210, 150, 90);             // Bronceado
        let beige = ShaderColor::from_rgb(230, 190, 130);          // Beige
        let cream = ShaderColor::from_rgb(245, 220, 170);          // Crema
        let white = ShaderColor::from_rgb(255, 250, 230);          // Blanco
        let red_spot = ShaderColor::from_rgb(200, 60, 30);         // Rojo intenso
        
        let latitude = uv.1;
        let animated_longitude = uv.0 + uniforms.time * 0.015;
        
        // === BANDAS BASE CON TEXTURA ===
        // Sistema de bandas alternadas (14 bandas como Júpiter real)
        let band_pos = latitude * 14.0;
        let band_index = band_pos.floor() as i32 % 8;
        let band_fract = band_pos.fract();
        
        // TURBULENCIA HORIZONTAL FUERTE (corrientes de chorro)
        let jet_stream = fbm(
            animated_longitude * 12.0,
            latitude * 5.0,
            5
        ) * 0.25; // Más turbulencia
        
        // Remolinos y vórtices a lo largo de las bandas
        let vortices = fbm(
            animated_longitude * 8.0 + latitude * 20.0,
            latitude * 8.0,
            4
        ) * 0.15;
        
        // Textura fina (nubes pequeñas)
        let fine_texture = fbm(
            animated_longitude * 25.0,
            latitude * 20.0,
            3
        ) * 0.08;
        
        // Combinar todas las texturas
        let texture_value = jet_stream + vortices + fine_texture;
        let final_band = (band_fract + texture_value).clamp(0.0, 1.0);
        
        // === COLORES POR BANDA (Alternando oscuro/claro) ===
        let base_color = match band_index {
            0 => mix_color(very_dark, dark_brown, final_band),
            1 => mix_color(tan, beige, final_band),
            2 => mix_color(dark_brown, rust_brown, final_band),
            3 => mix_color(cream, white, final_band),
            4 => mix_color(rust_brown, orange, final_band),
            5 => mix_color(beige, cream, final_band),
            6 => mix_color(orange, tan, final_band),
            _ => mix_color(white, cream, final_band),
        };
        
        let mut final_color = base_color;
        
        // === GRAN MANCHA ROJA (más grande y visible) ===
        let storm_x = 0.3;
        let storm_y = 0.4;
        let dx = (animated_longitude - storm_x) * 2.5; // Elipse horizontal
        let dy = latitude - storm_y;
        let dist_storm = (dx * dx + dy * dy).sqrt();
        
        if dist_storm < 0.15 {
            let strength = smoothstep(0.15, 0.06, dist_storm);
            // Rotación interna del remolino
            let angle = dy.atan2(dx);
            let spiral = (angle * 3.0 - dist_storm * 10.0 + uniforms.time * 0.5).sin() * 0.5 + 0.5;
            let storm_color = mix_color(red_spot, orange, spiral);
            final_color = mix_color(final_color, storm_color, strength * 0.9);
        }
        
        // === ÓVALOS BLANCOS (tormentas menores) ===
        let oval1_dist = ((animated_longitude - 0.6).powi(2) * 4.0 + (latitude - 0.55).powi(2)).sqrt();
        if oval1_dist < 0.05 {
            let oval_str = smoothstep(0.05, 0.02, oval1_dist);
            final_color = mix_color(final_color, white, oval_str * 0.8);
        }
        
        let oval2_dist = ((animated_longitude - 0.75).powi(2) * 5.0 + (latitude - 0.32).powi(2)).sqrt();
        if oval2_dist < 0.04 {
            let oval_str = smoothstep(0.04, 0.015, oval2_dist);
            final_color = mix_color(final_color, cream, oval_str * 0.7);
        }
        
        // === ILUMINACIÓN ===
        let light_dir = uniforms.light_direction.normalize();
        let diffuse = normal.dot(&light_dir).max(0.0);
        let ambient = 0.4;
        let lighting = (ambient + diffuse * 0.6).min(1.0);
        
        ShaderColor::new(
            (final_color.r * lighting).clamp(0.0, 1.0),
            (final_color.g * lighting).clamp(0.0, 1.0),
            (final_color.b * lighting).clamp(0.0, 1.0),
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

// ============================================================================
// PLANETA 5: PLANETA METÁLICO CON PICOS/PÚAS
// Características: Superficie con prickles/picos generados en VERTEX SHADER
// ============================================================================
pub struct SaturnShader;

impl PlanetShader for SaturnShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> (Vector3, Vector3) {
        // === GENERAR PICOS/PÚAS PROCEDURALMENTE ===
        
        // CAPA 1: Picos grandes principales (usando Voronoi para distribución)
        let voronoi_scale = 15.0;
        let voronoi_pattern = voronoi_noise(
            position.x * voronoi_scale + position.z * voronoi_scale,
            position.y * voronoi_scale
        );
        
        // Los picos se generan donde el Voronoi es pequeño (centros de células)
        let spike_large = if voronoi_pattern < 0.15 {
            smoothstep(0.15, 0.05, voronoi_pattern) * 0.35 // Picos grandes
        } else {
            0.0
        };
        
        // CAPA 2: Picos medianos (más densidad)
        let voronoi_medium = voronoi_noise(
            position.x * 25.0 + position.z * 25.0,
            position.y * 25.0
        );
        
        let spike_medium = if voronoi_medium < 0.12 {
            smoothstep(0.12, 0.04, voronoi_medium) * 0.25
        } else {
            0.0
        };
        
        // CAPA 3: Picos pequeños (muy densos, como púas)
        let voronoi_small = voronoi_noise(
            position.x * 40.0 + uniforms.time * 0.1 + position.z * 40.0,
            position.y * 40.0
        );
        
        let spike_small = if voronoi_small < 0.1 {
            smoothstep(0.1, 0.03, voronoi_small) * 0.15
        } else {
            0.0
        };
        
        // CAPA 4: Rugosidad base (textura áspera metálica)
        let roughness = fbm3d(
            position.x * 50.0,
            position.y * 50.0,
            position.z * 50.0,
            4
        ) * 0.05;
        
        // CAPA 5: Deformación animada (pulsación metálica)
        let pulse = (uniforms.time * 2.0 + position.length() * 5.0).sin() * 0.02;
        
        // Combinar todas las capas de picos
        let total_displacement = spike_large + spike_medium + spike_small + roughness + pulse;
        
        // Desplazar el vértice a lo largo de la normal (hacia afuera)
        let displaced_position = position + normal * total_displacement;
        
        // Recalcular la normal basada en los picos
        // (aproximación: la normal apunta más hacia afuera donde hay picos)
        let spike_factor = spike_large * 2.0 + spike_medium * 1.5 + spike_small;
        let adjusted_normal = (normal + normal * spike_factor).normalize();
        
        (displaced_position, adjusted_normal)
    }

    fn fragment_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor {
        // === PALETA METÁLICA ===
        let dark_metal = ShaderColor::from_rgb(40, 45, 50);        // Metal oscuro
        let medium_metal = ShaderColor::from_rgb(80, 90, 100);     // Acero
        let light_metal = ShaderColor::from_rgb(140, 150, 160);    // Plata oscura
        let bright_metal = ShaderColor::from_rgb(200, 210, 220);   // Plata brillante
        let chrome = ShaderColor::from_rgb(240, 245, 250);         // Cromado
        let rust_accent = ShaderColor::from_rgb(120, 80, 60);      // Acento oxidado
        
        // === TEXTURA METÁLICA PROCEDURAL ===
        
        // Patrón Voronoi para variación metálica
        let metal_pattern = voronoi_noise(
            position.x * 20.0 + position.z * 20.0,
            position.y * 20.0
        );
        
        // Ruido para imperfecciones metálicas
        let imperfections = fbm3d(
            position.x * 30.0,
            position.y * 30.0,
            position.z * 30.0,
            4
        );
        
        // Rayones y arañazos (scratch pattern)
        let scratches = fbm(
            uv.0 * 100.0,
            uv.1 * 100.0,
            3
        );
        
        // Combinar para obtener color base metálico
        let metal_value = (metal_pattern + imperfections * 0.5 + scratches * 0.3 + 1.5) / 3.0;
        
        let base_color = if metal_value > 0.8 {
            mix_color(chrome, bright_metal, (1.0 - metal_value) * 5.0)
        } else if metal_value > 0.6 {
            mix_color(bright_metal, light_metal, (0.8 - metal_value) * 5.0)
        } else if metal_value > 0.4 {
            mix_color(light_metal, medium_metal, (0.6 - metal_value) * 5.0)
        } else if metal_value > 0.2 {
            mix_color(medium_metal, dark_metal, (0.4 - metal_value) * 5.0)
        } else {
            mix_color(dark_metal, rust_accent, metal_value * 5.0)
        };
        
        // === ILUMINACIÓN METÁLICA ===
        let light_dir = uniforms.light_direction.normalize();
        let view_dir = (uniforms.camera_position - position).normalize();
        
        // Difusa (metales reflejan poco difusamente)
        let diffuse = normal.dot(&light_dir).max(0.0) * 0.3;
        
        // Especular FUERTE (metales son muy especulares)
        let reflect_dir = normal * (2.0 * normal.dot(&light_dir)) - light_dir;
        let specular = view_dir.dot(&reflect_dir).max(0.0).powf(32.0) * 1.2; // Exponente alto para brillo concentrado
        
        // Especular secundario (reflexión más amplia)
        let specular_broad = view_dir.dot(&reflect_dir).max(0.0).powf(8.0) * 0.5;
        
        // Fresnel effect (bordes más brillantes)
        let fresnel = (1.0 - view_dir.dot(&normal).abs()).powf(3.0) * 0.4;
        
        // Ambiente metálico
        let ambient = 0.2;
        
        // Oclusión ambiental en los valles entre picos
        let ao = (1.0 - imperfections.abs() * 0.3).max(0.5);
        
        // Combinar iluminación
        let lighting_intensity = (ambient * ao + diffuse + specular + specular_broad + fresnel).min(2.0);
        
        let lit_color = ShaderColor::new(
            base_color.r * lighting_intensity,
            base_color.g * lighting_intensity,
            base_color.b * lighting_intensity,
            1.0,
        );
        
        // Añadir highlight especular adicional (brillo puro)
        let final_color = if specular > 0.8 {
            let highlight_amount = (specular - 0.8) * 5.0;
            mix_color(lit_color, chrome, highlight_amount.min(0.5))
        } else {
            lit_color
        };
        
        ShaderColor::new(
            final_color.r.clamp(0.0, 1.0),
            final_color.g.clamp(0.0, 1.0),
            final_color.b.clamp(0.0, 1.0),
            1.0,
        )
    }
}
