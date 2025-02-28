use bytemuck::{Pod, Zeroable};
use std::mem::size_of;
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    pub fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            // https://sotrh.github.io/learn-wgpu/beginner/tutorial4-buffer/#so-what-do-i-do-with-it
            // This could be simplified using wgpu::vertex_attr_array!
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[rustfmt::skip]
pub const VERTICES: &[Vertex] = &[
    Vertex {position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614]}, // A
    Vertex {position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354]}, // B
    Vertex {position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397]}, // C
    Vertex {position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732914]}, // D
    Vertex {position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641]}, // E
];

#[rustfmt::skip]
pub const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];
