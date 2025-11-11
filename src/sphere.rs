use crate::vector::Vector3;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vector3,
    pub normal: Vector3,
    pub uv: (f32, f32),
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn from_obj(path: &str) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| format!("No se pudo abrir el archivo OBJ: {}", e))?;
        let reader = BufReader::new(file);
        
        let mut positions: Vec<Vector3> = Vec::new();
        let mut normals: Vec<Vector3> = Vec::new();
        let mut uvs: Vec<(f32, f32)> = Vec::new();
        
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        
        // Mapa para evitar duplicar vértices
        let mut vertex_map: HashMap<String, u32> = HashMap::new();
        
        for line in reader.lines() {
            let line = line.map_err(|e| format!("Error leyendo línea: {}", e))?;
            let line = line.trim();
            
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            
            match parts[0] {
                "v" if parts.len() >= 4 => {
                    // Vértice posición
                    let x = parts[1].parse::<f32>().map_err(|e| format!("Error parseando x: {}", e))?;
                    let y = parts[2].parse::<f32>().map_err(|e| format!("Error parseando y: {}", e))?;
                    let z = parts[3].parse::<f32>().map_err(|e| format!("Error parseando z: {}", e))?;
                    positions.push(Vector3::new(x, y, z));
                },
                "vn" if parts.len() >= 4 => {
                    // Normal
                    let x = parts[1].parse::<f32>().map_err(|e| format!("Error parseando nx: {}", e))?;
                    let y = parts[2].parse::<f32>().map_err(|e| format!("Error parseando ny: {}", e))?;
                    let z = parts[3].parse::<f32>().map_err(|e| format!("Error parseando nz: {}", e))?;
                    normals.push(Vector3::new(x, y, z));
                },
                "vt" if parts.len() >= 3 => {
                    // Coordenadas de textura
                    let u = parts[1].parse::<f32>().map_err(|e| format!("Error parseando u: {}", e))?;
                    let v = parts[2].parse::<f32>().map_err(|e| format!("Error parseando v: {}", e))?;
                    uvs.push((u, v));
                },
                "f" if parts.len() >= 4 => {
                    // Cara (triángulo)
                    let mut face_indices = Vec::new();
                    
                    for i in 1..parts.len() {
                        let vertex_str = parts[i];
                        
                        // Verificar si ya procesamos este vértice
                        if let Some(&index) = vertex_map.get(vertex_str) {
                            face_indices.push(index);
                        } else {
                            // Parsear índices v/vt/vn
                            let indices_parts: Vec<&str> = vertex_str.split('/').collect();
                            
                            if indices_parts.is_empty() {
                                continue;
                            }
                            
                            let pos_idx = indices_parts[0].parse::<usize>()
                                .map_err(|e| format!("Error parseando índice de posición: {}", e))? - 1;
                            
                            let uv_idx = if indices_parts.len() > 1 && !indices_parts[1].is_empty() {
                                indices_parts[1].parse::<usize>()
                                    .map_err(|e| format!("Error parseando índice de UV: {}", e))? - 1
                            } else {
                                0
                            };
                            
                            let normal_idx = if indices_parts.len() > 2 && !indices_parts[2].is_empty() {
                                indices_parts[2].parse::<usize>()
                                    .map_err(|e| format!("Error parseando índice de normal: {}", e))? - 1
                            } else {
                                0
                            };
                            
                            // Crear vértice
                            let position = if pos_idx < positions.len() {
                                positions[pos_idx]
                            } else {
                                return Err(format!("Índice de posición fuera de rango: {}", pos_idx));
                            };
                            
                            let normal = if normal_idx < normals.len() {
                                normals[normal_idx]
                            } else {
                                // Si no hay normal, usar la posición normalizada como aproximación
                                position.normalize()
                            };
                            
                            let uv = if uv_idx < uvs.len() {
                                uvs[uv_idx]
                            } else {
                                (0.0, 0.0)
                            };
                            
                            let vertex = Vertex {
                                position,
                                normal,
                                uv,
                            };
                            
                            let new_index = vertices.len() as u32;
                            vertices.push(vertex);
                            vertex_map.insert(vertex_str.to_string(), new_index);
                            face_indices.push(new_index);
                        }
                    }
                    
                    // Agregar índices del triángulo (asumiendo caras trianguladas)
                    if face_indices.len() >= 3 {
                        indices.push(face_indices[0]);
                        indices.push(face_indices[1]);
                        indices.push(face_indices[2]);
                        
                        // Si la cara tiene más de 3 vértices, triangular en abanico
                        for i in 3..face_indices.len() {
                            indices.push(face_indices[0]);
                            indices.push(face_indices[i - 1]);
                            indices.push(face_indices[i]);
                        }
                    }
                },
                _ => {}
            }
        }
        
        println!("Modelo OBJ cargado: {} vértices, {} triángulos", vertices.len(), indices.len() / 3);
        
        Ok(Mesh {
            vertices,
            indices,
        })
    }
}