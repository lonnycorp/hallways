pub struct TextureDepth {
    view: wgpu::TextureView,
}

pub struct TextureDepthNewParams<'a> {
    pub device: &'a wgpu::Device,
    pub width: u32,
    pub height: u32,
}

impl TextureDepth {
    pub fn new(params: TextureDepthNewParams<'_>) -> Self {
        let texture = params.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: params.width,
                height: params.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        return Self {
            view: texture.create_view(&Default::default()),
        };
    }

    pub fn view(&self) -> &wgpu::TextureView {
        return &self.view;
    }
}
