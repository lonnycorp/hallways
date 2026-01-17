use bytemuck::{Pod, Zeroable};
use glam::Vec3;

const PORTAL_MODEL_VERTEX_SHADER_LOCATION_POSITION: u32 = 0;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct PortalModelVertex {
    pub position: Vec3,
}

pub fn portal_model_vertex_layout() -> wgpu::VertexBufferLayout<'static> {
    return wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<PortalModelVertex>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x3,
            offset: std::mem::offset_of!(PortalModelVertex, position) as u64,
            shader_location: PORTAL_MODEL_VERTEX_SHADER_LOCATION_POSITION,
        }],
    };
}
