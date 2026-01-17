const BIND_GROUP_INDEX: u32 = 0;

pub struct PipelineCompositeBindGroupTexture {
    bind_group: wgpu::BindGroup,
}

impl PipelineCompositeBindGroupTexture {
    pub fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Composite Texture Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
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

    pub fn new(
        device: &wgpu::Device,
        oit_accum_view: &wgpu::TextureView,
        oit_reveal_view: &wgpu::TextureView,
    ) -> Self {
        let layout = Self::layout(device);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Composite Texture Bind Group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(oit_accum_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(oit_reveal_view),
                },
            ],
        });

        return Self { bind_group };
    }

    pub fn bind<'a>(&'a self, rp: &mut wgpu::RenderPass<'a>) {
        rp.set_bind_group(BIND_GROUP_INDEX, &self.bind_group, &[]);
    }
}
