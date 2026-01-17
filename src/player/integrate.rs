use glam::Vec3;
use parry3d::math::Isometry;
use parry3d::query::PointQuery;

use crate::app::SIM_STEP;
use crate::level::cache::LevelCache;

use super::movement_state::PlayerMovementState;
use super::sweep::PlayerSweepParams;
use super::Player;

const EPSILON: f32 = 0.001;
const GROUND_NORMAL_Y_MIN: f32 = 0.7;
const SIM_STEP_SECS: f32 = SIM_STEP.as_secs_f32();
const MAX_ITERATIONS: usize = 3;
const SKIN_THICKNESS: f32 = 0.01;
const GROUND_CHECK_DISTANCE: f32 = 0.04;
const GROUND_SNAP_DISTANCE: f32 = 0.1;
const OVERCLIP: f32 = 1.001;

impl Player {
    pub(super) fn clip_velocity(velocity: Vec3, planes: &[Vec3]) -> Vec3 {
        let n = planes.len();
        if n == 0 {
            return velocity;
        }
        if n == 1 {
            let backoff = velocity.dot(planes[0]) * OVERCLIP;
            return velocity - planes[0] * backoff;
        }
        if n == 2 {
            let dir = planes[0].cross(planes[1]).normalize_or_zero();
            let crease = dir * dir.dot(velocity);
            return crease * OVERCLIP;
        }
        return Vec3::ZERO;
    }

    pub(super) fn slide_move(&mut self, cache: &mut LevelCache, current_tick: u64) {
        let primal_vel = self.velocity;
        let mut resolved_vel = primal_vel;
        let mut remaining_time = SIM_STEP_SECS;
        let mut planes: Vec<Vec3> = Vec::new();

        for _ in 0..MAX_ITERATIONS {
            if let Some(hit) = self.sweep(PlayerSweepParams {
                cache,
                velocity: resolved_vel,
                max_toi: remaining_time,
                current_tick,
            }) {
                let approach_speed = (-resolved_vel.dot(hit.normal)).max(0.0);
                let epsilon_time = SKIN_THICKNESS / approach_speed.max(EPSILON);
                let safe_time = (hit.time - epsilon_time).max(0.0);

                let travel = resolved_vel * safe_time;
                self.position += travel;
                remaining_time -= safe_time;
                if travel.length() > EPSILON {
                    planes.clear();
                }

                let shape_pos =
                    Isometry::translation(self.position.x, self.position.y, self.position.z);
                let hit_point = parry3d::math::Point::new(hit.point.x, hit.point.y, hit.point.z);
                let collider = self.movement_state.collider();
                let dist = collider.distance_to_point(&shape_pos, &hit_point, true);
                if dist < EPSILON {
                    self.position += hit.normal * EPSILON;
                }
                if resolved_vel.dot(hit.normal) >= 0.0 {
                    continue;
                }

                planes.push(hit.normal);
                resolved_vel = Self::clip_velocity(primal_vel, &planes);

                if resolved_vel.dot(primal_vel) <= 0.0 {
                    resolved_vel = Vec3::ZERO;
                    break;
                }
                continue;
            }

            self.position += resolved_vel * remaining_time;
            break;
        }

        self.velocity = resolved_vel;
    }

    pub(super) fn integrate(&mut self, cache: &mut LevelCache, current_tick: u64) {
        let prev_state = self.movement_state;

        self.slide_move(cache, current_tick);

        match prev_state {
            PlayerMovementState::Airborne { crouching } => {
                let hit = self.sweep(PlayerSweepParams {
                    cache,
                    velocity: Vec3::NEG_Y,
                    max_toi: GROUND_CHECK_DISTANCE,
                    current_tick,
                });
                if let Some(hit) = hit {
                    if hit.normal.y >= GROUND_NORMAL_Y_MIN {
                        let snap_dist = (hit.time - SKIN_THICKNESS).max(0.0);
                        self.position.y -= snap_dist;
                        self.movement_state = PlayerMovementState::Grounded {
                            normal: hit.normal.normalize_or_zero(),
                            crouching,
                        };
                    }
                }
                return;
            }
            PlayerMovementState::Grounded { crouching, .. } => {
                let hit = self.sweep(PlayerSweepParams {
                    cache,
                    velocity: Vec3::NEG_Y,
                    max_toi: GROUND_SNAP_DISTANCE,
                    current_tick,
                });
                if let Some(hit) = hit {
                    if hit.normal.y >= GROUND_NORMAL_Y_MIN {
                        let snap_dist = (hit.time - SKIN_THICKNESS).max(0.0);
                        self.position.y -= snap_dist;
                        self.movement_state = PlayerMovementState::Grounded {
                            normal: hit.normal.normalize_or_zero(),
                            crouching,
                        };
                        return;
                    }
                }
                self.movement_state = PlayerMovementState::Airborne { crouching };
                return;
            }
            PlayerMovementState::Floating { .. } => {
                let hit = self.sweep(PlayerSweepParams {
                    cache,
                    velocity: self.wish_direction,
                    max_toi: GROUND_CHECK_DISTANCE,
                    current_tick,
                });
                self.movement_state = match hit {
                    Some(hit)
                        if (hit.normal.y >= GROUND_NORMAL_Y_MIN
                            || hit.normal.y <= -GROUND_NORMAL_Y_MIN) =>
                    {
                        PlayerMovementState::Floating {
                            normal: Some(hit.normal.normalize_or_zero()),
                        }
                    }
                    _ => PlayerMovementState::Floating { normal: None },
                };
                return;
            }
        }
    }
}
