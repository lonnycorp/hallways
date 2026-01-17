use crate::graphics::pipeline::sprite::bind_sprite_constants;
use crate::graphics::uniform::CameraUniformBufferWriteParams;
use crate::level::cache::LevelCacheResult;
use crate::level::{LevelRenderParams, LevelRenderSchema};
use crate::overlay::{banner_render, BannerRenderParams};

use super::App;

const CAMERA_FOV_RADIANS: f32 = 75f32.to_radians();
const CAMERA_NEAR: f32 = 0.05;
const CAMERA_FAR: f32 = 1000.0;

impl App {
    pub(super) fn render(&mut self) {
        let context = self.gpu_state.as_mut().unwrap();
        let output = context.surface.get_current_texture().unwrap();
        let color_view = output.texture.create_view(&Default::default());
        let mut encoder = context.device.create_command_encoder(&Default::default());

        {
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &color_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });
        }

        if let Some(level_url) = self.player.level_url() {
            if let LevelCacheResult::Ready(level) = context.cache.get(level_url, self.tick) {
                let camera = CameraUniformBufferWriteParams {
                    position: self.player.eye_position(),
                    rotation: self.player.rotation(),
                    clip_position: glam::Vec3::ZERO,
                    clip_normal: glam::Vec3::ZERO,
                    projection_fov_radians: CAMERA_FOV_RADIANS,
                    projection_aspect_ratio: context.surface_config.width as f32
                        / context.surface_config.height as f32,
                    projection_near: CAMERA_NEAR,
                    projection_far: CAMERA_FAR,
                };

                level.render(LevelRenderParams {
                    queue: &context.queue,
                    encoder: &mut encoder,
                    camera,
                    tick: self.tick,
                    render_frames: &context.render_frames,
                    frame_ix: 0,
                    cache: &mut context.cache,
                    pipeline_level: &context.pipeline_level,
                    pipeline_level_transparent: &context.pipeline_level_transparent,
                    pipeline_composite: &context.pipeline_composite,
                    pipeline_portal: &context.pipeline_portal,
                    color_view: &color_view,
                    depth_view: context.depth_texture.view(),
                    render_schema: LevelRenderSchema::Current,
                });
            }
        }

        context.overlay_buffer.clear();
        let status = self.status;
        self.intro.render(&mut crate::overlay::IntroRenderParams {
            buffer: &mut context.overlay_buffer,
            resolution: context.sprite_resolution,
            status,
        });
        self.menu.render(&mut context.overlay_buffer, status);
        self.menu_settings
            .render(&mut crate::overlay::MenuSettingsStateRenderParams {
                buffer: &mut context.overlay_buffer,
                status,
                tick: self.tick,
            });
        self.menu_visit
            .render(&mut crate::overlay::MenuVisitStateRenderParams {
                buffer: &mut context.overlay_buffer,
                status,
                tick: self.tick,
            });
        banner_render(BannerRenderParams {
            buffer: &mut context.overlay_buffer,
            resolution: context.sprite_resolution,
            status,
            player: &self.player,
            cache: &mut context.cache,
            tick: self.tick,
        });
        self.log.render(crate::overlay::LogRenderParams {
            buffer: &mut context.overlay_buffer,
            resolution: context.sprite_resolution,
            status,
        });

        context
            .overlay_model
            .upload(&context.queue, &context.overlay_buffer)
            .unwrap();

        {
            let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Banner Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &color_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });

            rp.set_pipeline(&context.pipeline_sprite);
            context.sprite_bind_group_texture.bind(&mut rp);
            bind_sprite_constants(&mut rp, context.sprite_resolution);
            context.overlay_model.draw(&mut rp);
        }

        context.queue.submit([encoder.finish()]);
        output.present();
    }
}
