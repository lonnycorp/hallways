use glam::{Mat3, Vec3};

use crate::app::SIM_STEP;
use crate::level::cache::LevelCache;
use crate::parry3d::CylinderExt;

use super::movement_state::PlayerMovementState;
use super::sweep::PlayerSweepParams;
use super::Player;

const SIM_STEP_SECS: f32 = SIM_STEP.as_secs_f32();
const STAND_DELTA: f32 = 0.7;
const GRAVITY: f32 = 80.0;
const JUMP_SPEED: f32 = 20.0;

fn velocity_plane_project(vector: Vec3, normal: Vec3) -> Vec3 {
    return vector - normal * vector.dot(normal);
}

impl Player {
    fn velocity_floating_plane_normal_get(&self) -> Vec3 {
        let pitch = Mat3::from_rotation_x(self.rotation.x);
        let yaw = Mat3::from_rotation_y(self.rotation.y);
        return (yaw * pitch * Vec3::Y).normalize_or_zero();
    }

    fn velocity_friction_apply(
        &mut self,
        plane_normal: Vec3,
        friction: f32,
        planar_velocity: bool,
    ) {
        let normal_component = plane_normal * self.velocity.dot(plane_normal);
        let mut planar = velocity_plane_project(self.velocity, plane_normal);
        let speed = planar.length();
        if speed <= 0.0 {
            if planar_velocity {
                self.velocity = planar;
            } else {
                self.velocity = normal_component + planar;
            }
            return;
        }

        let next_speed = (speed - friction * SIM_STEP_SECS).max(0.0);
        if next_speed <= 0.0 {
            planar = Vec3::ZERO;
            if planar_velocity {
                self.velocity = planar;
            } else {
                self.velocity = normal_component + planar;
            }
            return;
        }

        planar *= next_speed / speed;
        if planar_velocity {
            self.velocity = planar;
        } else {
            self.velocity = normal_component + planar;
        }
    }

    fn velocity_acceleration_apply(
        &mut self,
        plane_normal: Vec3,
        wish_speed: f32,
        accel: f32,
        planar_velocity: bool,
    ) {
        let normal_component = plane_normal * self.velocity.dot(plane_normal);
        let mut planar = velocity_plane_project(self.velocity, plane_normal);
        let wish_direction =
            velocity_plane_project(self.wish_direction, plane_normal).normalize_or_zero();
        if wish_direction == Vec3::ZERO || wish_speed <= 0.0 {
            if planar_velocity {
                self.velocity = planar;
            } else {
                self.velocity = normal_component + planar;
            }
            return;
        }

        let wish_velocity = wish_direction * wish_speed;
        let delta = wish_velocity - planar;
        let delta_len = delta.length();
        if delta_len <= 0.0 {
            if planar_velocity {
                self.velocity = planar;
            } else {
                self.velocity = normal_component + planar;
            }
            return;
        }

        let max_step = accel * SIM_STEP_SECS;
        if delta_len <= max_step {
            planar = wish_velocity;
        } else {
            planar += delta / delta_len * max_step;
        }

        if planar_velocity {
            self.velocity = planar;
        } else {
            self.velocity = normal_component + planar;
        }
    }

    pub(super) fn posture_update(&mut self, cache: &mut LevelCache, current_tick: u64) {
        let old_state = self.movement_state;
        let old_height = old_state.collider().height();

        self.movement_state = match old_state {
            PlayerMovementState::Grounded {
                normal,
                crouching: old_crouching,
            } => {
                let clear_sweep = self
                    .sweep(PlayerSweepParams {
                        cache,
                        velocity: Vec3::Y,
                        max_toi: STAND_DELTA,
                        current_tick,
                    })
                    .is_none();
                let crouching = self.wish_crouching || (old_crouching && !clear_sweep);
                if self.wish_jumping {
                    PlayerMovementState::Airborne { crouching }
                } else {
                    PlayerMovementState::Grounded { normal, crouching }
                }
            }
            PlayerMovementState::Airborne {
                crouching: old_crouching,
            } => {
                let clear_sweep = self
                    .sweep(PlayerSweepParams {
                        cache,
                        velocity: Vec3::NEG_Y,
                        max_toi: STAND_DELTA,
                        current_tick,
                    })
                    .is_none();
                let crouching = self.wish_crouching || (old_crouching && !clear_sweep);
                if self.wish_float {
                    PlayerMovementState::Floating { normal: None }
                } else {
                    PlayerMovementState::Airborne { crouching }
                }
            }
            PlayerMovementState::Floating { normal } => {
                if self.wish_jumping {
                    PlayerMovementState::Airborne { crouching: true }
                } else {
                    PlayerMovementState::Floating { normal }
                }
            }
        };

        let new_height = self.movement_state.collider().height();

        self.wish_jumping = false;
        self.wish_float = false;

        match self.movement_state {
            PlayerMovementState::Grounded {
                normal,
                crouching: _,
            } => {
                let properties = self.movement_state.properties();
                self.position.y += (new_height - old_height) / 2.0;
                self.velocity_friction_apply(normal, properties.friction, true);
                self.velocity_acceleration_apply(normal, properties.speed, properties.accel, true);
            }
            PlayerMovementState::Airborne { .. } => {
                let properties = self.movement_state.properties();
                self.position.y -= (new_height - old_height) / 2.0;
                self.velocity_friction_apply(Vec3::Y, properties.friction, false);
                self.velocity_acceleration_apply(
                    Vec3::Y,
                    properties.speed,
                    properties.accel,
                    false,
                );
                if matches!(old_state, PlayerMovementState::Grounded { .. }) {
                    self.velocity.y = JUMP_SPEED;
                } else {
                    self.velocity.y -= GRAVITY * SIM_STEP_SECS;
                }
            }
            PlayerMovementState::Floating { normal } => {
                let properties = self.movement_state.properties();
                let plane_normal = match normal {
                    Some(normal) => normal,
                    None => self.velocity_floating_plane_normal_get(),
                };
                self.position.y -= (new_height - old_height) / 2.0;
                self.velocity_friction_apply(plane_normal, properties.friction, true);
                self.velocity_acceleration_apply(
                    plane_normal,
                    properties.speed,
                    properties.accel,
                    true,
                );
            }
        }

        return;
    }
}
