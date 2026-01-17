mod handler;
mod state;

pub use handler::{WindowHandler, WindowHandlerEvent};
pub use state::WindowState;

pub struct Window<H: handler::WindowHandler> {
    title: String,
    handler: H,
}

use glam::Vec2;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowId;

impl<H: WindowHandler> Window<H> {
    pub fn new(title: &str, handler: H) -> Self {
        return Self {
            title: title.to_string(),
            handler,
        };
    }

    pub fn run(&mut self) {
        env_logger::init();
        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(self).unwrap();
    }
}

impl<H: WindowHandler> ApplicationHandler for Window<H> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let event = WindowHandlerEvent::Resume(WindowState::build(&self.title, event_loop));
        self.handler.on_event(event, event_loop);
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        self.handler
            .on_event(WindowHandlerEvent::Suspend, event_loop);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let handler = &mut self.handler;

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                handler.on_event(WindowHandlerEvent::KeyChange(event), event_loop);
            }
            WindowEvent::Resized(size) => {
                handler.on_event(
                    WindowHandlerEvent::Resize {
                        width: size.width,
                        height: size.height,
                    },
                    event_loop,
                );
            }
            WindowEvent::RedrawRequested => {
                handler.on_event(WindowHandlerEvent::Redraw, event_loop);
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        if let DeviceEvent::MouseMotion { delta } = event {
            let delta = Vec2::new(delta.0 as f32, delta.1 as f32);
            self.handler
                .on_event(WindowHandlerEvent::MouseMotion(delta), event_loop);
        }
    }
}
