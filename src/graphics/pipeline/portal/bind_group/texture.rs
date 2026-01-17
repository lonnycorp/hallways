use crate::graphics::texture::{sampler_bind_group_layout_entry, Sampler};

const BIND_GROUP_INDEX: u32 = 0;

pub struct PipelinePortalBindGroupTexture {
    bind_group: wgpu::BindGroup,
}

impl PipelinePortalBindGroupTexture {
    pub fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Portal Texture Bind Group Layout"),
            entries: &[
                sampler_bind_group_layout_entry(0),
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
            ],
        });
    }

    pub fn new(device: &wgpu::Device, color_view: &wgpu::TextureView) -> Self {
        let layout = Self::layout(device);
        let sampler = Sampler::new(
            device,
            (
                wgpu::AddressMode::ClampToEdge,
                wgpu::AddressMode::ClampToEdge,
            ),
            wgpu::FilterMode::Linear,
        );

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Portal Texture Bind Group"),
            layout: &layout,
            entries: &[
                sampler.bind_group_entry(0),
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(color_view),
                },
            ],
        });

        return Self { bind_group };
    }

    pub fn bind<'a>(&'a self, rp: &mut wgpu::RenderPass<'a>) {
        rp.set_bind_group(BIND_GROUP_INDEX, &self.bind_group, &[]);
    }
}
