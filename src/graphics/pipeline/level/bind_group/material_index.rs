use crate::graphics::storage::MaterialIndexStorageBuffer;

const BIND_GROUP_INDEX: u32 = 2;

pub fn material_index_bind_group_layout_create(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Level Material Index Bind Group Layout"),
        entries: &[MaterialIndexStorageBuffer::bind_group_layout_entry(0)],
    });
}

pub struct PipelineLevelBindGroupMaterialIndex {
    bind_group: wgpu::BindGroup,
}

impl PipelineLevelBindGroupMaterialIndex {
    pub fn new(device: &wgpu::Device, material_index: &MaterialIndexStorageBuffer) -> Self {
        let layout = material_index_bind_group_layout_create(device);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Level Material Index Bind Group"),
            layout: &layout,
            entries: &[material_index.bind_group_entry(0)],
        });

        return Self { bind_group };
    }

    pub fn bind<'a>(&'a self, rp: &mut wgpu::RenderPass<'a>) {
        rp.set_bind_group(BIND_GROUP_INDEX, &self.bind_group, &[]);
    }
}
