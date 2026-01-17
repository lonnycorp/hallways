use std::collections::HashMap;

use crate::audio::TrackData;
use crate::graphics::model::Model;
use crate::graphics::pipeline::level::{
    LevelModelVertex, PipelineLevelBindGroupMaterialIndex, PipelineLevelBindGroupTexture,
};
use parry3d::shape::TriMesh;

use self::manifest::LevelManifest;

pub mod cache;
mod error;
mod load;
mod manifest;
mod material;
mod portal;
mod render;

pub struct Level {
    manifest: LevelManifest,
    collider: TriMesh,
    model: Model<LevelModelVertex>,
    texture_bind_group: PipelineLevelBindGroupTexture,
    material_index_bind_group: PipelineLevelBindGroupMaterialIndex,
    portals: Vec<portal::LevelPortal>,
    portal_lookup: HashMap<String, usize>,
    track: Option<TrackData>,
}

pub use error::LevelLoadError;
pub use manifest::LevelManifestMeta;
pub use portal::LevelPortal;
pub use render::{LevelRenderParams, LevelRenderSchema};

use glam::Vec3;
use parry3d::math::{Isometry, Vector};
use parry3d::query::{cast_shapes, ShapeCastHit, ShapeCastOptions};
use parry3d::shape::Shape;

impl Level {
    pub fn meta(&self) -> &LevelManifestMeta {
        return &self.manifest.meta;
    }

    pub fn sweep(
        &self,
        pos: &Isometry<f32>,
        vel: &Vector<f32>,
        shape: &dyn Shape,
        max_toi: f32,
    ) -> Option<ShapeCastHit> {
        return cast_shapes(
            pos,
            vel,
            shape,
            &Isometry::identity(),
            &Vector::zeros(),
            &self.collider,
            ShapeCastOptions::with_max_time_of_impact(max_toi),
        )
        .unwrap();
    }

    pub fn track(&self) -> Option<&TrackData> {
        return self.track.as_ref();
    }

    pub fn portal(&self, name: &str) -> Option<&LevelPortal> {
        let portal_ix = *self.portal_lookup.get(name)?;
        return self.portals.get(portal_ix);
    }

    pub fn portals(&self) -> &[LevelPortal] {
        return &self.portals;
    }

    pub fn spawn_position(&self) -> Vec3 {
        return self.manifest.level.spawn.unwrap_or(Vec3::ZERO);
    }
}
