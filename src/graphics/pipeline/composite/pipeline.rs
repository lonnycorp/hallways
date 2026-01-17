use super::bind_group::PipelineCompositeBindGroupTexture;

const SHADER_PATH: &str = "shader/composite.wgsl";

pub struct PipelineComposite {
    pipeline: wgpu::RenderPipeline,
}

pub struct PipelineCompositeNewParams<'a> {
    pub device: &'a wgpu::Device,
    pub format: wgpu::TextureFormat,
}

pub struct PipelineCompositeRenderPassParams<'a> {
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub color_view: &'a wgpu::TextureView,
    pub texture_bind_group: &'a PipelineCompositeBindGroupTexture,
}

impl PipelineComposite {
    pub fn new(params: PipelineCompositeNewParams<'_>) -> Self {
        let shader = params
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Composite Shader"),
                source: wgpu::ShaderSource::Wgsl(
                    std::str::from_utf8(crate::ASSET.get_file(SHADER_PATH).unwrap().contents())
                        .unwrap()
                        .into(),
                ),
            });

        let texture_bind_group_layout = PipelineCompositeBindGroupTexture::layout(params.device);

        let pipeline_layout =
            params
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Composite Pipeline Layout"),
                    bind_group_layouts: &[&texture_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let pipeline = params
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Composite Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: params.format,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent {
                                src_factor: wgpu::BlendFactor::One,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                operation: wgpu::BlendOperation::Add,
                            },
                            alpha: wgpu::BlendComponent {
                                src_factor: wgpu::BlendFactor::One,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                operation: wgpu::BlendOperation::Add,
                            },
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
            });

        return Self { pipeline };
    }

    pub fn render_pass<'a>(
        &'a self,
        params: PipelineCompositeRenderPassParams<'a>,
    ) -> wgpu::RenderPass<'a> {
        let mut rp = params
            .encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Composite Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: params.color_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });

        rp.set_pipeline(&self.pipeline);
        params.texture_bind_group.bind(&mut rp);
        return rp;
    }
}
