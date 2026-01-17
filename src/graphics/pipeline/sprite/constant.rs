use std::mem::size_of;

use bytemuck::{Pod, Zeroable};
use glam::Vec2;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct SpritePushConstants {
    pub resolution: Vec2,
}

pub const PUSH_CONSTANT_RANGE: wgpu::PushConstantRange = wgpu::PushConstantRange {
    stages: wgpu::ShaderStages::VERTEX,
    range: 0..size_of::<SpritePushConstants>() as u32,
};

pub fn bind(rp: &mut wgpu::RenderPass, resolution: Vec2) {
    let pc = SpritePushConstants { resolution };
    rp.set_push_constants(wgpu::ShaderStages::VERTEX, 0, bytemuck::bytes_of(&pc));
}
