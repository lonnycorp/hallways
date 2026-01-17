use glam::{Mat3, Vec3};
use strum::EnumCount;
use strum::IntoEnumIterator;
use winit::event::{ElementState, KeyEvent};

use super::Player;
use crate::app::SIM_STEP;
use crate::config::Config;
use crate::config::ConfigControl;

const SIM_STEP_SECS: f32 = SIM_STEP.as_secs_f32();
const FLOAT_REPRESS_WINDOW_SECS: f32 = 0.350;
const FLOAT_REPRESS_WINDOW_TICKS: u64 = (FLOAT_REPRESS_WINDOW_SECS / SIM_STEP_SECS).ceil() as u64;

pub(super) struct PlayerWishKeyEventParams<'a> {
    pub event: &'a KeyEvent,
    pub config: &'a Config,
    pub current_tick: u64,
}

impl Player {
    pub(super) fn wish_clear(&mut self) {
        self.wish_direction = Vec3::ZERO;
        self.wish_jumping = false;
        self.wish_float = false;
        self.wish_crouching = false;
        self.control_held = [false; ConfigControl::COUNT];
    }

    pub(super) fn wish_key_event(&mut self, params: PlayerWishKeyEventParams<'_>) {
        let control = ConfigControl::iter().find(|control| {
            return params.config.key_get(*control) == &params.event.physical_key;
        });
        let Some(control) = control else {
            return;
        };

        match control {
            ConfigControl::Forward
            | ConfigControl::Back
            | ConfigControl::StrafeLeft
            | ConfigControl::StrafeRight
            | ConfigControl::Crouch => {
                self.control_held[control as usize] =
                    matches!(params.event.state, ElementState::Pressed);
            }
            ConfigControl::Jump => {
                if !matches!(
                    (params.event.state, params.event.repeat),
                    (ElementState::Pressed, false)
                ) {
                    return;
                }

                self.wish_jumping = true;
                let jump_tick_delta = params.current_tick.wrapping_sub(self.wish_float_jump_tick);
                self.wish_float = jump_tick_delta <= FLOAT_REPRESS_WINDOW_TICKS;
                self.wish_float_jump_tick = params.current_tick;
            }
        }
    }

    pub(super) fn wish_motion_update(&mut self) {
        let mut local_direction = Vec3::ZERO;
        if self.control_held[ConfigControl::Forward as usize] {
            local_direction.z -= 1.0;
        }
        if self.control_held[ConfigControl::Back as usize] {
            local_direction.z += 1.0;
        }
        if self.control_held[ConfigControl::StrafeRight as usize] {
            local_direction.x += 1.0;
        }
        if self.control_held[ConfigControl::StrafeLeft as usize] {
            local_direction.x -= 1.0;
        }

        if local_direction != Vec3::ZERO {
            let pitch = Mat3::from_rotation_x(self.rotation.x);
            let yaw = Mat3::from_rotation_y(self.rotation.y);
            self.wish_direction = yaw * pitch * local_direction.normalize();
        } else {
            self.wish_direction = Vec3::ZERO;
        }

        self.wish_crouching = self.control_held[ConfigControl::Crouch as usize];
    }
}
