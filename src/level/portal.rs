mod geometry;
mod link;

#[cfg(test)]
mod test;

pub use geometry::LevelPortalGeometry;
pub use link::LevelPortalLink;

use parry3d::math::{Isometry, Vector};
use parry3d::query::{cast_shapes, ShapeCastHit, ShapeCastOptions};
use parry3d::shape::{Shape, TriMesh};
use url::Url;

use crate::gltf::{GLTFMesh, GLTFMeshError};
use crate::graphics::model::{Model, ModelUploadError};
use crate::graphics::pipeline::portal::PortalModelVertex;
use crate::level::cache::{LevelCache, LevelCacheResult};
use crate::parry3d::trimesh::{TriMeshFromPositionsError, TriMeshFromPositionsExt};

use self::geometry::LevelPortalGeometryFromGLTFError;
use super::manifest::{AssetManifest, LevelManifestPortalTarget};

#[derive(Debug)]
pub enum LevelPortalLoadError {
    URLJoin(url::ParseError),
    GLTF(GLTFMeshError),
    GeometryFromGLTF(LevelPortalGeometryFromGLTFError),
    ModelUpload(ModelUploadError),
    Collider(TriMeshFromPositionsError),
}

pub struct LevelPortalLoadParams<'a> {
    pub name: String,
    pub portal_ix: usize,
    pub base_url: &'a Url,
    pub assets: &'a AssetManifest,
    pub collider_href: &'a str,
    pub target: Option<&'a LevelManifestPortalTarget>,
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
}

pub struct LevelPortalTarget {
    url: Url,
    name: String,
}

pub struct LevelPortal {
    name: String,
    portal_ix: usize,
    geometry: LevelPortalGeometry,
    model: Model<PortalModelVertex>,
    collider: TriMesh,
    target: Option<LevelPortalTarget>,
}

impl LevelPortal {
    pub fn new(
        name: String,
        portal_ix: usize,
        geometry: LevelPortalGeometry,
        model: Model<PortalModelVertex>,
        collider: TriMesh,
        target: Option<LevelPortalTarget>,
    ) -> Self {
        return Self {
            name,
            portal_ix,
            geometry,
            model,
            collider,
            target,
        };
    }

    pub fn load(params: LevelPortalLoadParams<'_>) -> Result<Self, LevelPortalLoadError> {
        let collider_data = params.assets.asset_get(params.collider_href).unwrap();
        let portal_mesh =
            GLTFMesh::from_bytes(collider_data).map_err(LevelPortalLoadError::GLTF)?;

        let target = match params.target {
            Some(target) => {
                let target_url = params
                    .base_url
                    .join(&target.href)
                    .map_err(LevelPortalLoadError::URLJoin)?;
                Some(LevelPortalTarget {
                    url: target_url,
                    name: target.name.clone(),
                })
            }
            None => None,
        };
        let geometry = LevelPortalGeometry::from_gltf(portal_mesh.vertices())
            .map_err(LevelPortalLoadError::GeometryFromGLTF)?;

        let portal_vertices: Vec<_> = portal_mesh.vertices().collect();
        let portal_buffer: Vec<_> = portal_vertices
            .iter()
            .map(|vertex| PortalModelVertex {
                position: vertex.position,
            })
            .collect();
        let mut portal_model = Model::new(params.device, portal_mesh.vertex_count());
        portal_model
            .upload(params.queue, &portal_buffer)
            .map_err(LevelPortalLoadError::ModelUpload)?;
        let portal_collider = TriMesh::from_positions(portal_vertices.iter().map(|v| v.position))
            .map_err(LevelPortalLoadError::Collider)?;

        return Ok(Self::new(
            params.name,
            params.portal_ix,
            geometry,
            portal_model,
            portal_collider,
            target,
        ));
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn index(&self) -> usize {
        return self.portal_ix;
    }

    pub fn geometry(&self) -> &LevelPortalGeometry {
        return &self.geometry;
    }

    pub fn target(&self) -> Option<&LevelPortalTarget> {
        return self.target.as_ref();
    }

    pub fn draw<'a>(&'a self, rp: &mut wgpu::RenderPass<'a>) {
        self.model.draw(rp);
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

    pub fn link(&self, cache: &mut LevelCache, tick: u64) -> Option<LevelPortalLink> {
        let target = self.target.as_ref()?;

        let LevelCacheResult::Ready(level) = cache.get(&target.url, tick) else {
            return None;
        };
        let dst_portal = level.portal(&target.name)?;
        if !self.geometry.matches(&dst_portal.geometry) {
            return None;
        }

        return Some(LevelPortalLink {
            portal_ix: dst_portal.index(),
            src: self.geometry.clone(),
            dst: dst_portal.geometry.clone(),
        });
    }
}

impl LevelPortalTarget {
    pub fn url(&self) -> &Url {
        return &self.url;
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }
}
