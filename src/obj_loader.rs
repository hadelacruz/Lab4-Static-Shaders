use nalgebra_glm::Vec3;
use crate::vertex::Vertex;
use std::path::Path;

pub fn load_obj(path: &str) -> Result<Vec<Vertex>, String> {
    let obj_path = Path::new(path);
    
    let load_result = tobj::load_obj(
        obj_path,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    );

    let (models, _) = load_result.map_err(|e| format!("Error cargando OBJ: {}", e))?;

    if models.is_empty() {
        return Err("El archivo OBJ no contiene modelos".to_string());
    }

    let mesh = &models[0].mesh;
    let mut vertices = Vec::new();

    let positions = &mesh.positions;
    let normals = &mesh.normals;
    let texcoords = &mesh.texcoords;

    for i in 0..positions.len() / 3 {
        let position = Vec3::new(
            positions[i * 3],
            positions[i * 3 + 1],
            positions[i * 3 + 2],
        );

        let normal = if !normals.is_empty() {
            Vec3::new(
                normals[i * 3],
                normals[i * 3 + 1],
                normals[i * 3 + 2],
            )
        } else {
            Vec3::new(0.0, 1.0, 0.0)
        };

        let tex_coords = if !texcoords.is_empty() && texcoords.len() > i * 2 + 1 {
            (texcoords[i * 2], texcoords[i * 2 + 1])
        } else {
            (0.0, 0.0)
        };

        vertices.push(Vertex::new(position, normal, tex_coords));
    }

    Ok(vertices)
}
