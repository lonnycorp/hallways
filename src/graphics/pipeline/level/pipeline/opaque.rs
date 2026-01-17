use super::super::bind_group::{
    material_index_bind_group_layout_create, texture_bind_group_layout_create,
    PipelineLevelBindGroupMaterialIndex, PipelineLevelBindGroupTexture,
};
use super::super::constant::{bind as bind_level_constants, PUSH_CONSTANT_RANGE};
use super::super::vertex::level_model_vertex_layout;
use crate::color::Color;
use crate::graphics::pipeline::common::PipelineBindGroupCamera;

const SHADER_PATH: &str = "shader/level.wgsl";

pub struct PipelineLevelOpaque {
    pipeline: wgpu::RenderPipeline,
}

pub struct PipelineLevelOpaqueNewParams<'a> {
    pub device: &'a wgpu::Device,
    pub format: wgpu::TextureFormat,
}

pub struct PipelineLevelOpaqueRenderPassParams<'a> {
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub color_view: &'a wgpu::TextureView,
    pub depth_view: &'a wgpu::TextureView,
    pub texture_bind_group: &'a PipelineLevelBindGroupTexture,
    pub camera_bind_group: &'a PipelineBindGroupCamera,
    pub material_index_bind_group: &'a PipelineLevelBindGroupMaterialIndex,
    pub tick: u64,
}

impl PipelineLevelOpaque {
    pub fn new(params: PipelineLevelOpaqueNewParams<'_>) -> Self {
        let shader = params
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Level Shader"),
                source: wgpu::ShaderSource::Wgsl(
                    std::str::from_utf8(crate::ASSET.get_file(SHADER_PATH).unwrap().contents())
                        .unwrap()
                        .into(),
                ),
            });

        let texture_layout = texture_bind_group_layout_create(params.device);
        let camera_layout = PipelineBindGroupCamera::layout(params.device);
        let material_index_layout = material_index_bind_group_layout_create(params.device);

        let pipeline_layout =
            params
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Level Pipeline Layout"),
                    bind_group_layouts: &[&texture_layout, &camera_layout, &material_index_layout],
                    push_constant_ranges: &[PUSH_CONSTANT_RANGE],
                });

        let pipeline = params
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Level Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[level_model_vertex_layout()],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_opaque"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: params.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: true,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
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
        params: PipelineLevelOpaqueRenderPassParams<'a>,
    ) -> wgpu::RenderPass<'a> {
        let mut rp = params
            .encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Level Opaque Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: params.color_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(Color::BLACK.into()),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: params.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });

        rp.set_pipeline(&self.pipeline);
        params.texture_bind_group.bind(&mut rp);
        params.camera_bind_group.bind(&mut rp);
        params.material_index_bind_group.bind(&mut rp);
        bind_level_constants(&mut rp, params.tick);
        return rp;
    }
}
