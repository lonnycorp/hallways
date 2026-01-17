use glam::Vec2;
use winit::event::KeyEvent;
use winit::event_loop::ActiveEventLoop;

use super::state::WindowState;

pub enum WindowHandlerEvent {
    Resume(WindowState),
    Suspend,
    KeyChange(KeyEvent),
    MouseMotion(Vec2),
    Resize { width: u32, height: u32 },
    Redraw,
}

pub trait WindowHandler {
    fn on_event(&mut self, event: WindowHandlerEvent, event_loop: &ActiveEventLoop);
}
