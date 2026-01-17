mod integrate;
mod movement_state;
mod posture;
mod rotation;
mod sweep;
mod teleport;
mod wish;

#[cfg(test)]
mod test;

use glam::{Vec2, Vec3};
use strum::EnumCount;
use url::Url;
use winit::event::KeyEvent;

use crate::app::AppStatus;
use crate::audio::CrossFader;
use crate::config::{Config, ConfigControl};
use crate::level::cache::{LevelCache, LevelCacheResult};
use crate::overlay::Log;

use self::movement_state::PlayerMovementState;
use self::teleport::PlayerTeleportParams;
use self::wish::PlayerWishKeyEventParams;

pub struct Player {
    prev_snapshot: Option<PlayerSnapshot>,
    position: Vec3,
    rotation: Vec2,
    velocity: Vec3,
    level_url: Option<Url>,
    wish_direction: Vec3,
    wish_jumping: bool,
    wish_float: bool,
    wish_crouching: bool,
    wish_float_jump_tick: u64,
    control_held: [bool; ConfigControl::COUNT],
    movement_state: PlayerMovementState,
}

#[derive(Clone, Copy, Debug)]
struct PlayerSnapshot {
    position: Vec3,
    movement_state: PlayerMovementState,
}

pub struct PlayerUpdateParams<'a> {
    pub status: AppStatus,
    pub cache: &'a mut LevelCache,
    pub cross_fader: &'a mut CrossFader,
    pub log: &'a mut Log,
    pub current_tick: u64,
}

pub struct PlayerKeyEventParams<'a> {
    pub status: AppStatus,
    pub event: &'a KeyEvent,
    pub config: &'a Config,
    pub current_tick: u64,
}

pub struct PlayerMouseMotionParams<'a> {
    pub status: AppStatus,
    pub delta: Vec2,
    pub config: &'a Config,
}

pub struct PlayerSpawnParams<'a> {
    pub level_url: Option<Url>,
    pub cache: &'a mut LevelCache,
    pub cross_fader: &'a mut CrossFader,
    pub current_tick: u64,
}

impl Player {
    pub(super) fn snapshot_update(&mut self) {
        self.prev_snapshot = Some(PlayerSnapshot {
            position: self.position,
            movement_state: self.movement_state,
        });
    }

    pub fn new(position: Vec3) -> Self {
        return Self {
            prev_snapshot: None,
            position,
            rotation: Vec2::ZERO,
            velocity: Vec3::ZERO,
            level_url: None,
            wish_direction: Vec3::ZERO,
            wish_jumping: false,
            wish_float: false,
            wish_crouching: false,
            wish_float_jump_tick: 0,
            control_held: [false; ConfigControl::COUNT],
            movement_state: PlayerMovementState::Airborne { crouching: false },
        };
    }

    pub fn key_event(&mut self, params: PlayerKeyEventParams<'_>) {
        if !matches!(params.status, AppStatus::Simulation) {
            return;
        }

        self.wish_key_event(PlayerWishKeyEventParams {
            event: params.event,
            config: params.config,
            current_tick: params.current_tick,
        });
    }

    pub fn mouse_motion(&mut self, params: PlayerMouseMotionParams<'_>) {
        if !matches!(params.status, AppStatus::Simulation) {
            return;
        }

        self.rotation_mouse_motion(params.delta, params.config);
    }

    pub fn update(&mut self, params: PlayerUpdateParams<'_>) {
        if !matches!(params.status, AppStatus::Simulation) {
            return;
        }

        self.snapshot_update();
        if self.level_url.is_none() {
            return;
        }

        self.wish_motion_update();
        self.posture_update(params.cache, params.current_tick);
        self.integrate(params.cache, params.current_tick);
        self.teleport(PlayerTeleportParams {
            cache: params.cache,
            cross_fader: params.cross_fader,
            log: params.log,
            current_tick: params.current_tick,
        });
    }

    pub fn status_change(&mut self, status: &AppStatus) {
        if matches!(status, AppStatus::Simulation) {
            self.wish_clear();
        }
    }

    pub fn rotation(&self) -> Vec2 {
        return self.rotation;
    }

    pub fn eye_position(&self) -> Vec3 {
        let collider = self.movement_state.collider();
        return self.position + Vec3::Y * collider.half_height;
    }

    pub fn level_url(&self) -> Option<&Url> {
        return self.level_url.as_ref();
    }

    pub fn spawn(&mut self, params: PlayerSpawnParams<'_>) {
        if let Some(level_url) = params.level_url {
            let LevelCacheResult::Ready(level) = params.cache.get(&level_url, params.current_tick)
            else {
                panic!("Level not ready");
            };
            match level.track().cloned() {
                Some(track) => params.cross_fader.track_fade_in(track),
                None => params.cross_fader.track_fade_out(),
            }

            self.position = level.spawn_position();
            self.level_url = Some(level_url);
            self.prev_snapshot = None;
            self.velocity = Vec3::ZERO;
            self.wish_float = false;
            self.wish_float_jump_tick = 0;
            self.movement_state = PlayerMovementState::Airborne { crouching: false };
            return;
        }

        params.cross_fader.track_fade_out();
        self.level_url = None;
    }
}
