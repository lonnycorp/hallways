use bytemuck::{Pod, Zeroable};
use glam::Vec2;

use crate::color::Color;

const SPRITE_MODEL_VERTEX_SHADER_LOCATION_POSITION: u32 = 0;
const SPRITE_MODEL_VERTEX_SHADER_LOCATION_UV: u32 = 1;
const SPRITE_MODEL_VERTEX_SHADER_LOCATION_TEXTURE_IX: u32 = 2;
const SPRITE_MODEL_VERTEX_SHADER_LOCATION_COLOR: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct SpriteModelVertex {
    pub position: Vec2,
    pub uv: Vec2,
    pub texture_ix: u32,
    pub color: Color,
}

pub fn sprite_model_vertex_layout() -> wgpu::VertexBufferLayout<'static> {
    return wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<SpriteModelVertex>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: std::mem::offset_of!(SpriteModelVertex, position) as u64,
                shader_location: SPRITE_MODEL_VERTEX_SHADER_LOCATION_POSITION,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: std::mem::offset_of!(SpriteModelVertex, uv) as u64,
                shader_location: SPRITE_MODEL_VERTEX_SHADER_LOCATION_UV,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Uint32,
                offset: std::mem::offset_of!(SpriteModelVertex, texture_ix) as u64,
                shader_location: SPRITE_MODEL_VERTEX_SHADER_LOCATION_TEXTURE_IX,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Unorm8x4,
                offset: std::mem::offset_of!(SpriteModelVertex, color) as u64,
                shader_location: SPRITE_MODEL_VERTEX_SHADER_LOCATION_COLOR,
            },
        ],
    };
}
