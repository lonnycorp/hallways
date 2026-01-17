use std::mem::size_of;

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct LevelPushConstants {
    pub tick: u32,
}

pub const PUSH_CONSTANT_RANGE: wgpu::PushConstantRange = wgpu::PushConstantRange {
    stages: wgpu::ShaderStages::FRAGMENT,
    range: 0..size_of::<LevelPushConstants>() as u32,
};

pub fn bind(rp: &mut wgpu::RenderPass, tick: u64) {
    let pc = LevelPushConstants { tick: tick as u32 };
    rp.set_push_constants(wgpu::ShaderStages::FRAGMENT, 0, bytemuck::bytes_of(&pc));
}
