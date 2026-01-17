use glam::Vec2;

use crate::graphics::frame::level::RenderFrameLevel;
use crate::graphics::pipeline::composite::{PipelineComposite, PipelineCompositeRenderPassParams};
use crate::graphics::pipeline::level::{
    PipelineLevelOpaque, PipelineLevelOpaqueRenderPassParams, PipelineLevelTransparent,
    PipelineLevelTransparentRenderPassParams,
};
use crate::graphics::pipeline::portal::{PipelinePortal, PipelinePortalRenderPassParams};
use crate::graphics::uniform::CameraUniformBufferWriteParams;
use crate::level::cache::{LevelCache, LevelCacheResult};

use super::Level;

pub enum LevelRenderSchema {
    Current,
    Portal { index: usize },
}

pub struct LevelRenderParams<'a> {
    pub queue: &'a wgpu::Queue,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub camera: CameraUniformBufferWriteParams,
    pub tick: u64,
    pub render_frames: &'a [RenderFrameLevel],
    pub frame_ix: usize,
    pub cache: &'a mut LevelCache,
    pub pipeline_level: &'a PipelineLevelOpaque,
    pub pipeline_level_transparent: &'a PipelineLevelTransparent,
    pub pipeline_composite: &'a PipelineComposite,
    pub pipeline_portal: &'a PipelinePortal,
    pub color_view: &'a wgpu::TextureView,
    pub depth_view: &'a wgpu::TextureView,
    pub render_schema: LevelRenderSchema,
}

impl Level {
    pub fn render(&self, params: LevelRenderParams) {
        let frame = &params.render_frames[params.frame_ix];
        let mut next_frame_ix = params.frame_ix + 1;

        frame.camera_buffer().write(params.queue, &params.camera);

        {
            let mut rp = params
                .pipeline_level
                .render_pass(PipelineLevelOpaqueRenderPassParams {
                    encoder: params.encoder,
                    color_view: params.color_view,
                    depth_view: params.depth_view,
                    texture_bind_group: &self.texture_bind_group,
                    camera_bind_group: frame.camera_bind_group(),
                    material_index_bind_group: &self.material_index_bind_group,
                    tick: params.tick,
                });
            self.model.draw(&mut rp);
        }

        {
            let mut rp = params.pipeline_level_transparent.render_pass(
                PipelineLevelTransparentRenderPassParams {
                    encoder: params.encoder,
                    oit_accum_view: frame.oit_accum_view(),
                    oit_reveal_view: frame.oit_reveal_view(),
                    depth_view: params.depth_view,
                    texture_bind_group: &self.texture_bind_group,
                    camera_bind_group: frame.camera_bind_group(),
                    material_index_bind_group: &self.material_index_bind_group,
                    tick: params.tick,
                },
            );
            self.model.draw(&mut rp);
        }

        {
            let mut rp = params
                .pipeline_composite
                .render_pass(PipelineCompositeRenderPassParams {
                    encoder: params.encoder,
                    color_view: params.color_view,
                    texture_bind_group: frame.texture_bind_group_composite(),
                });
            rp.draw(0..3, 0..1);
        }

        for src_portal in self.portals() {
            frame.clear(params.encoder);

            if matches!(&params.render_schema, LevelRenderSchema::Current) {
                let link = src_portal.link(params.cache, params.tick);
                let dst_level = link.as_ref().and_then(|_| {
                    let target = src_portal.target().unwrap();
                    match params.cache.get(target.url(), params.tick) {
                        LevelCacheResult::Ready(level) => Some(level),
                        _ => None,
                    }
                });

                if let (Some(link), Some(dst_level)) = (link, dst_level) {
                    let src_geometry = src_portal.geometry();
                    let yaw_delta = link.yaw_delta();
                    let eye_side = (src_geometry.center - params.camera.position)
                        .dot(src_geometry.normal)
                        .signum();
                    let clip_normal = link.dst.normal * eye_side;
                    let position = link.position_transform(params.camera.position);
                    let rotation = Vec2::new(
                        params.camera.rotation.x,
                        params.camera.rotation.y + yaw_delta,
                    );
                    let mut camera = params.camera;
                    camera.position = position;
                    camera.rotation = rotation;
                    camera.clip_position = link.dst.center;
                    camera.clip_normal = clip_normal;

                    let recurse_frame_ix = next_frame_ix;
                    next_frame_ix += 1;

                    dst_level.render(LevelRenderParams {
                        queue: params.queue,
                        encoder: params.encoder,
                        camera,
                        tick: params.tick,
                        render_frames: params.render_frames,
                        frame_ix: recurse_frame_ix,
                        cache: params.cache,
                        pipeline_level: params.pipeline_level,
                        pipeline_level_transparent: params.pipeline_level_transparent,
                        pipeline_composite: params.pipeline_composite,
                        pipeline_portal: params.pipeline_portal,
                        color_view: frame.color_view(),
                        depth_view: frame.depth_view(),
                        render_schema: LevelRenderSchema::Portal {
                            index: link.portal_ix,
                        },
                    });
                }
            }

            let skip_portal = match &params.render_schema {
                LevelRenderSchema::Current => false,
                LevelRenderSchema::Portal { index } => *index == src_portal.index(),
            };

            if skip_portal {
                continue;
            }

            {
                let mut rp = params
                    .pipeline_portal
                    .render_pass(PipelinePortalRenderPassParams {
                        encoder: params.encoder,
                        color_view: params.color_view,
                        depth_view: params.depth_view,
                        texture_bind_group: frame.texture_bind_group_portal(),
                        camera_bind_group: frame.camera_bind_group(),
                    });
                src_portal.draw(&mut rp);
            }
        }
    }
}
