use crate::color::Color;
use crate::graphics::pipeline::common::PipelineBindGroupCamera;
use crate::graphics::pipeline::composite::PipelineCompositeBindGroupTexture;
use crate::graphics::pipeline::portal::PipelinePortalBindGroupTexture;
use crate::graphics::uniform::UniformCameraBuffer;

pub struct RenderFrameLevel {
    color_view: wgpu::TextureView,
    depth_view: wgpu::TextureView,
    oit_accum_view: wgpu::TextureView,
    oit_reveal_view: wgpu::TextureView,
    camera_buffer: UniformCameraBuffer,
    camera_bind_group: PipelineBindGroupCamera,
    texture_bind_group_portal: PipelinePortalBindGroupTexture,
    texture_bind_group_composite: PipelineCompositeBindGroupTexture,
}

pub struct RenderFrameLevelNewParams<'a> {
    pub device: &'a wgpu::Device,
    pub size: (u32, u32),
    pub format: wgpu::TextureFormat,
}

impl RenderFrameLevel {
    pub fn new(params: RenderFrameLevelNewParams<'_>) -> Self {
        let extent = wgpu::Extent3d {
            width: params.size.0,
            height: params.size.1,
            depth_or_array_layers: 1,
        };

        let color_texture = params.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("RenderFrameLevel Color"),
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: params.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let oit_accum_texture = params.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("RenderFrameLevel OIT Accum"),
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let oit_reveal_texture = params.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("RenderFrameLevel OIT Reveal"),
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let depth_texture = params.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("RenderFrameLevel Depth"),
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let color_view = color_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let oit_accum_view = oit_accum_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let oit_reveal_view =
            oit_reveal_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let camera_buffer = UniformCameraBuffer::new(params.device);
        let camera_bind_group = PipelineBindGroupCamera::new(params.device, &camera_buffer);

        let texture_bind_group_portal =
            PipelinePortalBindGroupTexture::new(params.device, &color_view);

        let texture_bind_group_composite = PipelineCompositeBindGroupTexture::new(
            params.device,
            &oit_accum_view,
            &oit_reveal_view,
        );

        return Self {
            color_view,
            depth_view,
            oit_accum_view,
            oit_reveal_view,
            camera_buffer,
            camera_bind_group,
            texture_bind_group_portal,
            texture_bind_group_composite,
        };
    }

    pub fn clear(&self, encoder: &mut wgpu::CommandEncoder) {
        let _rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.color_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(Color::BLACK.into()),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
    }

    pub fn color_view(&self) -> &wgpu::TextureView {
        return &self.color_view;
    }

    pub fn depth_view(&self) -> &wgpu::TextureView {
        return &self.depth_view;
    }

    pub fn oit_accum_view(&self) -> &wgpu::TextureView {
        return &self.oit_accum_view;
    }

    pub fn oit_reveal_view(&self) -> &wgpu::TextureView {
        return &self.oit_reveal_view;
    }

    pub fn camera_buffer(&self) -> &UniformCameraBuffer {
        return &self.camera_buffer;
    }

    pub fn camera_bind_group(&self) -> &PipelineBindGroupCamera {
        return &self.camera_bind_group;
    }

    pub fn texture_bind_group_portal(&self) -> &PipelinePortalBindGroupTexture {
        return &self.texture_bind_group_portal;
    }

    pub fn texture_bind_group_composite(&self) -> &PipelineCompositeBindGroupTexture {
        return &self.texture_bind_group_composite;
    }
}
