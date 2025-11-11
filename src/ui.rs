// ============================================================================
// MÓDULO DE INTERFAZ DE USUARIO
// ============================================================================

use raylib::prelude::*;

pub struct UIConfig {
    pub margin: i32,
    pub panel_padding: i32,
    pub font_size_title: i32,
    pub font_size_normal: i32,
    pub font_size_small: i32,
}

impl Default for UIConfig {
    fn default() -> Self {
        UIConfig {
            margin: 20,
            panel_padding: 15,
            font_size_title: 20,
            font_size_normal: 16,
            font_size_small: 14,
        }
    }
}

pub struct PlanetInfo {
    pub name: &'static str,
    pub key: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    pub color: Color,
}

pub const PLANET_INFO: [PlanetInfo; 5] = [
    PlanetInfo {
        name: "Planeta Rocoso",
        key: "1",
        description: "Terreno deformado con cráteres",
        icon: "🪨",
        color: Color::new(160, 160, 160, 255),
    },
    PlanetInfo {
        name: "Gigante Gaseoso",
        key: "2",
        description: "Bandas atmosféricas como Júpiter",
        icon: "🌍",
        color: Color::new(200, 120, 50, 255),
    },
    PlanetInfo {
        name: "Planeta Sci-Fi",
        key: "3",
        description: "Circuitos y energía tecnológica",
        icon: "⚡",
        color: Color::new(0, 255, 255, 255),
    },
    PlanetInfo {
        name: "Planeta Nebulosa",
        key: "4",
        description: "Gas cósmico etéreo",
        icon: "🌌",
        color: Color::new(200, 0, 150, 255),
    },
    PlanetInfo {
        name: "Planeta Metálico",
        key: "5",
        description: "Superficie con picos metálicos",
        icon: "⚙️",
        color: Color::new(200, 210, 220, 255),
    },
];

pub fn render_ui(d: &mut RaylibDrawHandle, current_planet: usize, fps: i32) {
    let config = UIConfig::default();
    let width = d.get_screen_width();
    let height = d.get_screen_height();
    
    // Panel semi-transparente de fondo
    let panel_width = 380;
    let panel_height = 280;
    let panel_x = config.margin;
    let panel_y = config.margin;
    
    d.draw_rectangle(
        panel_x,
        panel_y,
        panel_width,
        panel_height,
        Color::new(0, 0, 0, 180),
    );
    
    // Borde del panel
    d.draw_rectangle_lines(
        panel_x,
        panel_y,
        panel_width,
        panel_height,
        Color::new(100, 200, 255, 200),
    );
    
    let mut y_offset = panel_y + config.panel_padding;
    
    // Título
    d.draw_text(
        "🌌 CONTROL DE PLANETAS",
        panel_x + config.panel_padding,
        y_offset,
        config.font_size_title,
        Color::new(100, 200, 255, 255),
    );
    y_offset += config.font_size_title + 10;
    
    // Línea separadora
    d.draw_line(
        panel_x + config.panel_padding,
        y_offset,
        panel_x + panel_width - config.panel_padding,
        y_offset,
        Color::new(100, 200, 255, 100),
    );
    y_offset += 10;
    
    // Lista de planetas
    for (i, info) in PLANET_INFO.iter().enumerate() {
        let is_current = i == current_planet;
        let text_color = if is_current {
            Color::new(255, 255, 100, 255)  
        } else {
            Color::new(200, 200, 200, 255)  
        };
        
        let bg_color = if is_current {
            Color::new(50, 50, 100, 150)  
        } else {
            Color::new(0, 0, 0, 0)  
        };
        
        // Fondo de la fila si está activa
        if is_current {
            d.draw_rectangle(
                panel_x + config.panel_padding - 5,
                y_offset - 2,
                panel_width - config.panel_padding * 2 + 10,
                config.font_size_normal + 4,
                bg_color,
            );
        }
        
        d.draw_text(
            &format!("[{}] {} {}", info.key, info.icon, info.name),
            panel_x + config.panel_padding,
            y_offset,
            config.font_size_normal,
            text_color,
        );
        y_offset += config.font_size_normal + 5;
        
        if is_current {
            d.draw_text(
                &format!("    {}", info.description),
                panel_x + config.panel_padding + 10,
                y_offset,
                config.font_size_small,
                Color::new(150, 150, 200, 255),
            );
            y_offset += config.font_size_small + 8;
        }
    }
    
    // Línea separadora inferior
    y_offset += 5;
    d.draw_line(
        panel_x + config.panel_padding,
        y_offset,
        panel_x + panel_width - config.panel_padding,
        y_offset,
        Color::new(100, 200, 255, 100),
    );
    y_offset += 10;
    
    // Controles adicionales
    d.draw_text(
        "🖱️  Mouse: Rotar cámara y zoom",
        panel_x + config.panel_padding,
        y_offset,
        config.font_size_small,
        Color::new(180, 180, 180, 255),
    );
    
    // FPS en la esquina superior derecha
    let fps_color = if fps >= 55 {
        Color::new(0, 255, 100, 255)  
    } else if fps >= 30 {
        Color::new(255, 200, 0, 255)  
    } else {
        Color::new(255, 50, 50, 255)  
    };
    
    let fps_text = format!("FPS: {}", fps);
    d.draw_text(
        &fps_text,
        width - 100,
        20,
        20,
        fps_color,
    );
    
    let current_name = PLANET_INFO[current_planet].name;
    d.draw_text(
        current_name,
        (width - 250) / 2,
        height - 50,
        24,
        PLANET_INFO[current_planet].color,
    );
}
