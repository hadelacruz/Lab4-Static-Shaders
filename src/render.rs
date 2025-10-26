use crate::framebuffer::Framebuffer;
use crate::vertex::{Vertex, Uniforms};
use crate::triangle::triangle;
use crate::fragment::FragmentShader;

pub fn render(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertices: &[Vertex],
    fragment_shader: FragmentShader,
) {
    let transform_matrix = uniforms.viewport_matrix * 
                          uniforms.projection_matrix * 
                          uniforms.view_matrix * 
                          uniforms.model_matrix;

    for triangle_vertices in vertices.chunks(3) {
        if triangle_vertices.len() != 3 {
            continue;
        }

        let fragments = triangle(
            &triangle_vertices[0],
            &triangle_vertices[1],
            &triangle_vertices[2],
            &transform_matrix,
        );

        for fragment in fragments {
            let x = fragment.position.x as usize;
            let y = fragment.position.y as usize;

            if x < framebuffer.width && y < framebuffer.height {
                let color = fragment_shader(&fragment, uniforms);
                framebuffer.point(x, y, color, fragment.depth);
            }
        }
    }
}
