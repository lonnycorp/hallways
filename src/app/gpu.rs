use std::sync::Arc;

use glam::Vec2;
use rayon::ThreadPool;

use crate::config::Config;
use crate::graphics::frame::level::{RenderFrameLevel, RenderFrameLevelNewParams};
use crate::graphics::model::Model;
use crate::graphics::pipeline::composite::{PipelineComposite, PipelineCompositeNewParams};
use crate::graphics::pipeline::level::{
    PipelineLevelOpaque, PipelineLevelOpaqueNewParams, PipelineLevelTransparent,
    PipelineLevelTransparentNewParams,
};
use crate::graphics::pipeline::portal::{PipelinePortal, PipelinePortalNewParams};
use crate::graphics::pipeline::sprite::bind_group::PipelineSpriteBindGroupTexture;
use crate::graphics::pipeline::sprite::pipeline_sprite_create;
use crate::graphics::pipeline::sprite::SpriteModelVertex;
use crate::graphics::texture::{TextureDepth, TextureDepthNewParams};
use crate::level::cache::{LevelCache, LevelCacheNewParams};

const OVERLAY_MODEL_VERTEX_CAPACITY: usize = 50_000;
const RENDER_FRAME_COUNT: usize = 6;
const TARGET_WIDTH: f32 = 1280.0;
const LEVEL_CACHE_CAPACITY: usize = 8;

pub struct AppGPUState {
    pub handle: Arc<winit::window::Window>,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub surface: Arc<wgpu::Surface<'static>>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub pipeline_level: PipelineLevelOpaque,
    pub pipeline_level_transparent: PipelineLevelTransparent,
    pub pipeline_composite: PipelineComposite,
    pub pipeline_portal: PipelinePortal,
    pub pipeline_sprite: wgpu::RenderPipeline,
    pub depth_texture: TextureDepth,
    pub sprite_bind_group_texture: PipelineSpriteBindGroupTexture,
    pub sprite_resolution: Vec2,
    pub overlay_buffer: Vec<SpriteModelVertex>,
    pub overlay_model: Model<SpriteModelVertex>,
    pub render_frames: Vec<RenderFrameLevel>,
    pub cache: LevelCache,
}

pub struct AppGPUStateRebuildResult {
    pub render_frames: Vec<RenderFrameLevel>,
    pub depth_texture: TextureDepth,
    pub sprite_resolution: Vec2,
}

pub struct AppGPUStateNewParams<'a> {
    pub handle: Arc<winit::window::Window>,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub asset_thread_pool: Arc<ThreadPool>,
    pub surface: Arc<wgpu::Surface<'static>>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub config: &'a Config,
}

impl AppGPUState {
    pub fn rebuild(
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
    ) -> AppGPUStateRebuildResult {
        let depth_texture = TextureDepth::new(TextureDepthNewParams {
            device,
            width: surface_config.width,
            height: surface_config.height,
        });

        let render_frames = (0..RENDER_FRAME_COUNT)
            .map(|_| {
                return RenderFrameLevel::new(RenderFrameLevelNewParams {
                    device,
                    size: (surface_config.width, surface_config.height),
                    format: surface_config.format,
                });
            })
            .collect();

        let width = surface_config.width as f32;
        let height = surface_config.height as f32;
        let scale = (width / TARGET_WIDTH).floor().max(1.0);
        let sprite_resolution = Vec2::new(width / scale, height / scale);

        return AppGPUStateRebuildResult {
            render_frames,
            depth_texture,
            sprite_resolution,
        };
    }

    pub fn new(params: AppGPUStateNewParams<'_>) -> Self {
        let mut surface_config = params.surface_config;
        surface_config.present_mode = params.config.vsync_status.present_mode();

        let rebuild = Self::rebuild(&params.device, &surface_config);

        let pipeline_level = PipelineLevelOpaque::new(PipelineLevelOpaqueNewParams {
            device: &params.device,
            format: surface_config.format,
        });
        let pipeline_level_transparent =
            PipelineLevelTransparent::new(PipelineLevelTransparentNewParams {
                device: &params.device,
            });
        let pipeline_composite = PipelineComposite::new(PipelineCompositeNewParams {
            device: &params.device,
            format: surface_config.format,
        });
        let pipeline_portal = PipelinePortal::new(PipelinePortalNewParams {
            device: &params.device,
            format: surface_config.format,
        });
        let pipeline_sprite = pipeline_sprite_create(&params.device, surface_config.format);
        let cache = LevelCache::new(LevelCacheNewParams {
            device: Arc::clone(&params.device),
            queue: Arc::clone(&params.queue),
            asset_thread_pool: Arc::clone(&params.asset_thread_pool),
            capacity: LEVEL_CACHE_CAPACITY,
        });

        let sprite_bind_group_texture =
            PipelineSpriteBindGroupTexture::new(&params.device, &params.queue);
        let overlay_buffer: Vec<SpriteModelVertex> = Vec::new();
        let overlay_model = Model::new(&params.device, OVERLAY_MODEL_VERTEX_CAPACITY);

        return Self {
            handle: params.handle,
            device: params.device,
            queue: params.queue,
            surface: params.surface,
            surface_config,
            pipeline_level,
            pipeline_level_transparent,
            pipeline_composite,
            pipeline_portal,
            pipeline_sprite,
            depth_texture: rebuild.depth_texture,
            sprite_bind_group_texture,
            sprite_resolution: rebuild.sprite_resolution,
            overlay_buffer,
            overlay_model,
            render_frames: rebuild.render_frames,
            cache,
        };
    }
}
