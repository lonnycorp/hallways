use std::f32::consts::TAU;

use glam::Vec2;

use super::Player;
use crate::config::Config;

const PITCH_LIMIT: f32 = 1.53589;
const BASE_MOUSE_SENSITIVITY: f32 = 0.002;

impl Player {
    pub(super) fn rotation_mouse_motion(&mut self, delta: Vec2, config: &Config) {
        let sensitivity = config.mouse_sensitivity * BASE_MOUSE_SENSITIVITY;
        self.rotation.x =
            (self.rotation.x - delta.y * sensitivity).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        self.rotation.y = (self.rotation.y - delta.x * sensitivity).rem_euclid(TAU);
    }
}
