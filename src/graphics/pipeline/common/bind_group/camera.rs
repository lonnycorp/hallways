use crate::graphics::uniform::{camera_bind_group_layout_entry, UniformCameraBuffer};

const BIND_GROUP_INDEX: u32 = 1;

pub struct PipelineBindGroupCamera {
    bind_group: wgpu::BindGroup,
}

impl PipelineBindGroupCamera {
    pub fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[camera_bind_group_layout_entry(0)],
        });
    }

    pub fn new(device: &wgpu::Device, camera: &UniformCameraBuffer) -> Self {
        let layout = Self::layout(device);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &layout,
            entries: &[camera.bind_group_entry(0)],
        });

        return Self { bind_group };
    }

    pub fn bind<'a>(&'a self, rp: &mut wgpu::RenderPass<'a>) {
        rp.set_bind_group(BIND_GROUP_INDEX, &self.bind_group, &[]);
    }
}
