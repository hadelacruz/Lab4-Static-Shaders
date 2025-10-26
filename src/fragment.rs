use nalgebra_glm::Vec3;
use crate::framebuffer::Color;
use crate::vertex::Uniforms;

#[derive(Clone)]
pub struct Fragment {
    pub position: Vec3,
    pub normal: Vec3,
    pub depth: f32,
    pub vertex_position: Vec3,
}

impl Fragment {
    pub fn new(position: Vec3, normal: Vec3, depth: f32, vertex_position: Vec3) -> Self {
        Fragment {
            position,
            normal,
            depth,
            vertex_position,
        }
    }
}

// Función de ruido (noise) para crear variaciones procedurales
pub fn noise(x: f32, y: f32, z: f32, time: f32) -> f32 {
    let x = x + time * 0.1;
    let y = y + time * 0.1;
    let z = z + time * 0.1;
    
    let n = (x * 12.9898 + y * 78.233 + z * 45.164).sin() * 43758.5453;
    n.fract()
}

pub fn fbm(x: f32, y: f32, z: f32, time: f32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;
    let mut frequency = 1.0;
    
    for _ in 0..6 {
        value += amplitude * noise(x * frequency, y * frequency, z * frequency, time);
        frequency *= 2.0;
        amplitude *= 0.5;
    }
    
    value
}

// Shader para planeta rocoso (tipo Marte/Tierra)
pub fn rocky_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let pos = fragment.vertex_position;
    
    // Colores base del planeta rocoso
    let color1 = Vec3::new(0.6, 0.3, 0.1); // Marrón
    let color2 = Vec3::new(0.4, 0.5, 0.2); // Verde oscuro
    let color3 = Vec3::new(0.3, 0.3, 0.3); // Gris oscuro
    let color4 = Vec3::new(0.7, 0.6, 0.4); // Beige
    
    // Generar ruido multi-octava para variación de terreno
    let noise_val = fbm(pos.x * 3.0, pos.y * 3.0, pos.z * 3.0, uniforms.time * 0.1);
    let noise2 = fbm(pos.x * 8.0, pos.y * 8.0, pos.z * 8.0, uniforms.time * 0.05);
    
    // Mezclar colores basado en el ruido
    let base_color = if noise_val > 0.6 {
        color1
    } else if noise_val > 0.4 {
        color2
    } else if noise_val > 0.25 {
        color3
    } else {
        color4
    };
    
    // Añadir detalles finos
    let detail = noise2 * 0.3;
    let final_color = base_color * (0.7 + detail);
    
    // Iluminación simple (Lambert)
    let light_dir = Vec3::new(1.0, 1.0, 1.0).normalize();
    let normal = fragment.normal.normalize();
    let light_intensity = normal.dot(&light_dir).max(0.0);
    let ambient = 0.3;
    let diffuse = light_intensity * 0.7;
    let total_light = ambient + diffuse;
    
    let lit_color = final_color * total_light;
    
    Color::from_float(
        lit_color.x.clamp(0.0, 1.0),
        lit_color.y.clamp(0.0, 1.0),
        lit_color.z.clamp(0.0, 1.0)
    )
}

// Shader para gigante gaseoso (tipo Júpiter)
pub fn gas_giant_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let pos = fragment.vertex_position;
    
    // Crear bandas horizontales características de gigantes gaseosos
    let band_frequency = 8.0;
    let y_coord = pos.y;
    let band_base = (y_coord * band_frequency).sin();
    
    // Añadir turbulencia a las bandas
    let turbulence = fbm(
        pos.x * 5.0 + uniforms.time * 0.3,
        pos.y * 2.0,
        pos.z * 5.0 + uniforms.time * 0.2,
        uniforms.time
    );
    
    let band_val = (band_base + turbulence * 0.5).clamp(-1.0, 1.0);
    
    // Colores del gigante gaseoso
    let color1 = Vec3::new(0.9, 0.7, 0.4); // Amarillo claro
    let color2 = Vec3::new(0.8, 0.5, 0.2); // Naranja
    let color3 = Vec3::new(0.6, 0.3, 0.1); // Marrón oscuro
    let color4 = Vec3::new(0.95, 0.85, 0.6); // Crema
    
    // Interpolar colores basado en las bandas
    let t = (band_val + 1.0) / 2.0;
    let base_color = if t > 0.75 {
        color1
    } else if t > 0.5 {
        color2
    } else if t > 0.25 {
        color3
    } else {
        color4
    };
    
    // Añadir tormentas (la gran mancha roja)
    let storm_center = Vec3::new(0.3, -0.2, 0.5);
    let dist_to_storm = (pos - storm_center).magnitude();
    let storm_effect = (1.0 - (dist_to_storm * 3.0).min(1.0)).max(0.0);
    let storm_color = Vec3::new(0.9, 0.3, 0.2);
    
    let final_color = base_color * (1.0 - storm_effect * 0.6) + storm_color * storm_effect * 0.6;
    
    // Iluminación suave
    let light_dir = Vec3::new(1.0, 0.5, 1.0).normalize();
    let normal = fragment.normal.normalize();
    let light_intensity = normal.dot(&light_dir).max(0.0);
    let ambient = 0.4;
    let diffuse = light_intensity * 0.6;
    let total_light = ambient + diffuse;
    
    let lit_color = final_color * total_light;
    
    Color::from_float(
        lit_color.x.clamp(0.0, 1.0),
        lit_color.y.clamp(0.0, 1.0),
        lit_color.z.clamp(0.0, 1.0)
    )
}

// Shader para planeta de ciencia ficción (cristalino con energía)
pub fn crystal_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let pos = fragment.vertex_position;
    
    // Cristales animados con el tiempo
    let crystal_pattern = fbm(
        pos.x * 6.0 + (uniforms.time * 0.5).sin() * 0.5,
        pos.y * 6.0 + (uniforms.time * 0.7).cos() * 0.5,
        pos.z * 6.0 + uniforms.time * 0.3,
        uniforms.time
    );
    
    // Venas de energía que recorren el planeta
    let energy_veins = (pos.x * 10.0 + uniforms.time).sin() * 
                       (pos.y * 10.0 - uniforms.time * 0.5).sin() * 
                       (pos.z * 10.0 + uniforms.time * 0.7).sin();
    let energy_val = (energy_veins * 0.5 + 0.5).powf(3.0);
    
    // Colores base del cristal
    let crystal_color1 = Vec3::new(0.2, 0.6, 0.9); // Azul cristal
    let crystal_color2 = Vec3::new(0.4, 0.2, 0.8); // Púrpura
    let energy_color = Vec3::new(0.0, 1.0, 0.8);   // Cyan brillante
    
    // Mezclar colores de cristal
    let base_crystal = if crystal_pattern > 0.5 {
        crystal_color1
    } else {
        crystal_color2
    };
    
    // Añadir venas de energía pulsante
    let pulse = (uniforms.time * 2.0).sin() * 0.5 + 0.5;
    let final_color = base_crystal * (1.0 - energy_val) + 
                      energy_color * energy_val * pulse;
    
    // Iluminación con efecto de resplandor
    let light_dir = Vec3::new(0.0, 1.0, 1.0).normalize();
    let normal = fragment.normal.normalize();
    let light_intensity = normal.dot(&light_dir).max(0.0);
    
    // Efecto de fresnel (bordes brillantes)
    let view_dir = Vec3::new(0.0, 0.0, 1.0);
    let fresnel = (1.0 - normal.dot(&view_dir).abs()).powf(3.0);
    
    let ambient = 0.4;
    let diffuse = light_intensity * 0.5;
    let rim_light = fresnel * 0.8;
    let total_light = ambient + diffuse + rim_light;
    
    let lit_color = final_color * total_light;
    
    Color::from_float(
        lit_color.x.clamp(0.0, 1.0),
        lit_color.y.clamp(0.0, 1.0),
        lit_color.z.clamp(0.0, 1.0)
    )
}

pub type FragmentShader = fn(&Fragment, &Uniforms) -> Color;
