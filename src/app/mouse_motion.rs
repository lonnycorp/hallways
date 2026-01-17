use glam::Vec2;

use crate::player::PlayerMouseMotionParams;

use super::App;

impl App {
    pub(super) fn mouse_motion(&mut self, delta: Vec2) {
        self.player.mouse_motion(PlayerMouseMotionParams {
            status: self.status,
            delta,
            config: &self.config,
        });
    }
}
