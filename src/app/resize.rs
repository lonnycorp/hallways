use super::gpu::AppGPUState;
use super::App;

impl App {
    pub(super) fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        let Some(context) = self.gpu_state.as_mut() else {
            return;
        };

        context.surface_config.width = width;
        context.surface_config.height = height;
        context
            .surface
            .configure(&context.device, &context.surface_config);

        let rebuild = AppGPUState::rebuild(&context.device, &context.surface_config);
        context.depth_texture = rebuild.depth_texture;
        context.render_frames = rebuild.render_frames;
        context.sprite_resolution = rebuild.sprite_resolution;
    }
}
