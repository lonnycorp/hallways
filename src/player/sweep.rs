use glam::Vec3;
use parry3d::math::{Isometry, Vector};

use crate::level::cache::{LevelCache, LevelCacheResult};

use super::Player;

pub struct PlayerSweepHit {
    pub time: f32,
    pub normal: Vec3,
    pub point: Vec3,
}

pub struct PlayerSweepParams<'a> {
    pub cache: &'a mut LevelCache,
    pub velocity: Vec3,
    pub max_toi: f32,
    pub current_tick: u64,
}

impl Player {
    pub(super) fn sweep(&self, params: PlayerSweepParams<'_>) -> Option<PlayerSweepHit> {
        let level_url = self.level_url.as_ref().unwrap();
        let LevelCacheResult::Ready(level) = params.cache.get(level_url, params.current_tick)
        else {
            return None;
        };
        let collider = self.movement_state.collider();
        let shape_pos = Isometry::translation(self.position.x, self.position.y, self.position.z);
        let shape_vel = Vector::new(params.velocity.x, params.velocity.y, params.velocity.z);

        let mut best_hit: Option<PlayerSweepHit> = level
            .sweep(&shape_pos, &shape_vel, &collider, params.max_toi)
            .map(|hit| {
                return PlayerSweepHit {
                    time: hit.time_of_impact,
                    normal: Vec3::new(hit.normal2.x, hit.normal2.y, hit.normal2.z),
                    point: Vec3::new(hit.witness2.x, hit.witness2.y, hit.witness2.z),
                };
            });

        for src_portal in level.portals() {
            let name = src_portal.name();
            let portal_hit = src_portal
                .sweep(&shape_pos, &shape_vel, &collider, params.max_toi)
                .map(|r| PlayerSweepHit {
                    time: r.time_of_impact,
                    normal: Vec3::new(r.normal2.x, r.normal2.y, r.normal2.z),
                    point: Vec3::new(r.witness2.x, r.witness2.y, r.witness2.z),
                });
            let Some(portal_hit) = portal_hit else {
                continue;
            };

            // Get linked level and destination portal
            let link = src_portal.link(params.cache, params.current_tick);
            let Some(link) = link else {
                // Destination level not loaded or incompatible - treat as solid
                match &best_hit {
                    Some(best) if best.time <= portal_hit.time => {}
                    _ => best_hit = Some(portal_hit),
                }
                continue;
            };
            let target = src_portal.target().unwrap();

            let LevelCacheResult::Ready(dst_level) =
                params.cache.get(target.url(), params.current_tick)
            else {
                panic!("linked level not ready")
            };

            // Validate destination portal links back to this level + source portal
            let Some(dst_portal) = dst_level.portals().get(link.portal_ix) else {
                continue;
            };
            let Some(dst_target) = dst_portal.target() else {
                match &best_hit {
                    Some(best) if best.time <= portal_hit.time => {}
                    _ => best_hit = Some(portal_hit),
                }
                continue;
            };
            if dst_target.url() != level_url || dst_target.name() != name {
                match &best_hit {
                    Some(best) if best.time <= portal_hit.time => {}
                    _ => best_hit = Some(portal_hit),
                }
                continue;
            }
            let transformed_pos = link.position_transform(self.position);
            let transformed_vel = link.velocity_transform(params.velocity);

            let shape_pos =
                Isometry::translation(transformed_pos.x, transformed_pos.y, transformed_pos.z);
            let shape_vel = Vector::new(transformed_vel.x, transformed_vel.y, transformed_vel.z);
            let result_hit = dst_level.sweep(&shape_pos, &shape_vel, &collider, params.max_toi);
            let Some(result_hit) = result_hit else {
                continue;
            };

            let dst_link = dst_portal.link(params.cache, params.current_tick).unwrap();
            let dst_normal = Vec3::new(
                result_hit.normal2.x,
                result_hit.normal2.y,
                result_hit.normal2.z,
            );
            let dst_point = Vec3::new(
                result_hit.witness2.x,
                result_hit.witness2.y,
                result_hit.witness2.z,
            );
            let through_portal_hit = PlayerSweepHit {
                time: result_hit.time_of_impact,
                normal: dst_link.velocity_transform(dst_normal),
                point: dst_link.position_transform(dst_point),
            };

            match &best_hit {
                Some(best) if best.time <= through_portal_hit.time => {}
                _ => best_hit = Some(through_portal_hit),
            }
        }

        return best_hit;
    }
}
