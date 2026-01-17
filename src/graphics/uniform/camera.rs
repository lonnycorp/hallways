use std::mem::size_of;
use std::num::NonZeroU64;

use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec2, Vec3, Vec4};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct CameraUniformData {
    projection: Mat4,
    view: Mat4,
    clip_plane: Vec4,
}

#[derive(Clone, Copy)]
pub struct CameraUniformBufferWriteParams {
    pub position: Vec3,
    pub rotation: Vec2,
    pub clip_position: Vec3,
    pub clip_normal: Vec3,
    pub projection_fov_radians: f32,
    pub projection_aspect_ratio: f32,
    pub projection_near: f32,
    pub projection_far: f32,
}

pub fn camera_bind_group_layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    return wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: NonZeroU64::new(size_of::<CameraUniformData>() as u64),
        },
        count: None,
    };
}

pub struct UniformCameraBuffer {
    buffer: wgpu::Buffer,
}

impl UniformCameraBuffer {
    pub fn new(device: &wgpu::Device) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Uniform Buffer"),
            size: size_of::<CameraUniformData>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        return Self { buffer };
    }

    pub fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry<'_> {
        return wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &self.buffer,
                offset: 0,
                size: NonZeroU64::new(size_of::<CameraUniformData>() as u64),
            }),
        };
    }

    pub fn write(&self, queue: &wgpu::Queue, params: &CameraUniformBufferWriteParams) {
        let projection = Mat4::perspective_rh(
            params.projection_fov_radians,
            params.projection_aspect_ratio,
            params.projection_near,
            params.projection_far,
        );

        let view = Mat4::from_rotation_x(-params.rotation.x)
            * Mat4::from_rotation_y(-params.rotation.y)
            * Mat4::from_translation(-params.position);

        let clip_plane = Vec4::new(
            params.clip_normal.x,
            params.clip_normal.y,
            params.clip_normal.z,
            -params.clip_normal.dot(params.clip_position),
        );

        let uniform = CameraUniformData {
            projection,
            view,
            clip_plane,
        };

        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&uniform));
    }
}
