use super::bind_group::texture_bind_group_layout_create;
use super::constant::PUSH_CONSTANT_RANGE;
use super::vertex::sprite_model_vertex_layout;

const SHADER_PATH: &str = "shader/overlay.wgsl";

pub fn pipeline_sprite_create(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Sprite Shader"),
        source: wgpu::ShaderSource::Wgsl(
            std::str::from_utf8(crate::ASSET.get_file(SHADER_PATH).unwrap().contents())
                .unwrap()
                .into(),
        ),
    });

    let texture_layout = texture_bind_group_layout_create(device);
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Sprite Pipeline Layout"),
        bind_group_layouts: &[&texture_layout],
        push_constant_ranges: &[PUSH_CONSTANT_RANGE],
    });

    return device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Sprite Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[sprite_model_vertex_layout()],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
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
}
