use std::sync::Arc;
use std::time::Duration;

use winit::event_loop::ActiveEventLoop;

use crate::window::{WindowHandler, WindowHandlerEvent, WindowState};

use super::gpu::{AppGPUState, AppGPUStateNewParams};
use super::App;

pub const SIM_STEP: Duration = Duration::from_millis(10);

impl WindowHandler for App {
    fn on_event(&mut self, event: WindowHandlerEvent, event_loop: &ActiveEventLoop) {
        match event {
            WindowHandlerEvent::Resume(WindowState {
                handle,
                device,
                queue,
                surface,
                config,
            }) => {
                let gpu_state = AppGPUState::new(AppGPUStateNewParams {
                    handle,
                    device: Arc::clone(&device),
                    queue: Arc::clone(&queue),
                    asset_thread_pool: Arc::clone(&self.asset_thread_pool),
                    surface,
                    surface_config: config,
                    config: &self.config,
                });
                gpu_state
                    .surface
                    .configure(&gpu_state.device, &gpu_state.surface_config);
                self.gpu_state = Some(gpu_state);
            }
            WindowHandlerEvent::Suspend => {
                self.gpu_state = None;
            }
            WindowHandlerEvent::KeyChange(event) => {
                if self.gpu_state.is_none() {
                    return;
                }

                self.key_event(&event, event_loop);
            }
            WindowHandlerEvent::MouseMotion(delta) => {
                if self.gpu_state.is_none() {
                    return;
                }

                self.mouse_motion(delta);
            }
            WindowHandlerEvent::Resize { width, height } => {
                self.resize(width, height);
            }
            WindowHandlerEvent::Redraw => {
                if self.gpu_state.is_none() {
                    return;
                }

                while self.last_update.elapsed() >= SIM_STEP {
                    self.status_swap();
                    self.update();
                    self.last_update += SIM_STEP;
                }
                self.render();
                self.gpu_state.as_ref().unwrap().handle.request_redraw();
            }
        }
    }
}
