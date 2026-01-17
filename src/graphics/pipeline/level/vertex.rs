use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3};

const LEVEL_MODEL_VERTEX_SHADER_LOCATION_POSITION: u32 = 0;
const LEVEL_MODEL_VERTEX_SHADER_LOCATION_DIFFUSE_UV: u32 = 1;
const LEVEL_MODEL_VERTEX_SHADER_LOCATION_MATERIAL_IX: u32 = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct LevelModelVertex {
    pub position: Vec3,
    pub diffuse_uv: Vec2,
    pub material_ix: u32,
}

pub fn level_model_vertex_layout() -> wgpu::VertexBufferLayout<'static> {
    return wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<LevelModelVertex>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: std::mem::offset_of!(LevelModelVertex, position) as u64,
                shader_location: LEVEL_MODEL_VERTEX_SHADER_LOCATION_POSITION,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: std::mem::offset_of!(LevelModelVertex, diffuse_uv) as u64,
                shader_location: LEVEL_MODEL_VERTEX_SHADER_LOCATION_DIFFUSE_UV,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Uint32,
                offset: std::mem::offset_of!(LevelModelVertex, material_ix) as u64,
                shader_location: LEVEL_MODEL_VERTEX_SHADER_LOCATION_MATERIAL_IX,
            },
        ],
    };
}
