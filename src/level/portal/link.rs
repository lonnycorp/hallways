use glam::{Mat3, Vec3};

use super::LevelPortalGeometry;

pub struct LevelPortalLink {
    pub portal_ix: usize,
    pub src: LevelPortalGeometry,
    pub dst: LevelPortalGeometry,
}

impl LevelPortalLink {
    pub fn yaw_delta(&self) -> f32 {
        return self.dst.yaw - self.src.yaw;
    }

    pub fn position_transform(&self, pos: Vec3) -> Vec3 {
        let local = pos - self.src.center;
        let rot = Mat3::from_rotation_y(self.yaw_delta());
        return self.dst.center + rot * local;
    }

    pub fn velocity_transform(&self, vel: Vec3) -> Vec3 {
        let rot = Mat3::from_rotation_y(self.yaw_delta());
        return rot * vel;
    }
}
