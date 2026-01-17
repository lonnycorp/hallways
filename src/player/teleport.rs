use crate::audio::CrossFader;
use glam::Vec3;
use parry3d::math::{Isometry, Vector};

use crate::level::cache::{LevelCache, LevelCacheResult};
use crate::overlay::{Log, LogData};

use super::movement_state::PlayerMovementState;
use super::Player;

pub struct PlayerTeleportParams<'a> {
    pub cache: &'a mut LevelCache,
    pub cross_fader: &'a mut CrossFader,
    pub log: &'a mut Log,
    pub current_tick: u64,
}

impl Player {
    pub(super) fn teleport(&mut self, params: PlayerTeleportParams<'_>) -> bool {
        let level_url = self.level_url.as_ref().unwrap();
        let LevelCacheResult::Ready(level) = params.cache.get(level_url, params.current_tick)
        else {
            return false;
        };
        let Some(prev_snapshot) = self.prev_snapshot else {
            return false;
        };
        let start_pos = prev_snapshot.position;
        let prev_collider = prev_snapshot.movement_state.collider();
        let start_eye = start_pos + Vec3::Y * prev_collider.half_height;
        let end_eye = self.eye_position();
        let collider = self.movement_state.collider();
        for src_portal in level.portals() {
            let Some(link) = src_portal.link(params.cache, params.current_tick) else {
                continue;
            };

            let src_geometry = src_portal.geometry();
            let src_normal = src_geometry.normal;
            let start_side = (start_eye - src_geometry.center).dot(src_normal);
            let end_side = (end_eye - src_geometry.center).dot(src_normal);
            let crossed = start_side * end_side <= 0.0;

            if !crossed {
                continue;
            }

            let move_delta = self.position - start_pos;
            let shape_pos = Isometry::translation(start_pos.x, start_pos.y, start_pos.z);
            let shape_vel = Vector::new(move_delta.x, move_delta.y, move_delta.z);
            let in_contact_prev = src_portal
                .sweep(&shape_pos, &shape_vel, &prev_collider, 1.0)
                .is_some();
            let in_contact_curr = src_portal
                .sweep(&shape_pos, &shape_vel, &collider, 1.0)
                .is_some();
            let in_contact = in_contact_prev || in_contact_curr;

            if in_contact {
                let target = src_portal.target().unwrap();
                let yaw_delta = link.yaw_delta();
                self.position = link.position_transform(self.position);
                self.velocity = link.velocity_transform(self.velocity);
                self.movement_state = match self.movement_state {
                    PlayerMovementState::Grounded { normal, crouching } => {
                        PlayerMovementState::Grounded {
                            normal: link.velocity_transform(normal).normalize_or_zero(),
                            crouching,
                        }
                    }
                    PlayerMovementState::Airborne { crouching } => {
                        PlayerMovementState::Airborne { crouching }
                    }
                    PlayerMovementState::Floating { normal } => {
                        let transformed_normal = normal
                            .map(|normal| link.velocity_transform(normal).normalize_or_zero());
                        PlayerMovementState::Floating {
                            normal: transformed_normal,
                        }
                    }
                };
                self.rotation.y += yaw_delta;

                let level_changed = self.level_url.as_ref() != Some(target.url());
                self.level_url = Some(target.url().clone());
                if level_changed {
                    let LevelCacheResult::Ready(dst_level) =
                        params.cache.get(target.url(), params.current_tick)
                    else {
                        panic!("linked level not ready")
                    };
                    match dst_level.track() {
                        Some(track) => params.cross_fader.track_fade_in(track.clone()),
                        None => params.cross_fader.track_fade_out(),
                    }
                    params.log.push(LogData::Entered {
                        name: dst_level.meta().name.to_string(),
                        portal_name: Some(src_portal.name().to_string()),
                    });
                }

                return true;
            }
        }
        return false;
    }
}
