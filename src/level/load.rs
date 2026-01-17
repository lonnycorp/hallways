use std::collections::HashMap;
use url::Url;

use glam::Vec2;
use parry3d::shape::TriMesh;
use rayon::ThreadPool;

use crate::audio::TrackData;
use crate::gltf::GLTFMesh;
use crate::graphics::model::Model;
use crate::graphics::pipeline::level::LevelModelVertex;
use crate::parry3d::trimesh::{TriMeshFromPositionsError, TriMeshFromPositionsExt};

use super::error::{LevelLoadError, LevelMeshLoadError, LevelModelBuildError, LevelTrackLoadError};
use super::manifest::{AssetManifest, LevelManifest, LevelManifestMaterial};
use super::material::level_material_load;
use super::portal::{LevelPortal, LevelPortalLoadParams};
use super::Level;

impl Level {
    fn material_index_build<'a>(
        manifest: &'a LevelManifest,
        mesh: &GLTFMesh,
    ) -> Vec<Option<&'a LevelManifestMaterial>> {
        let mut mapped: Vec<Option<&LevelManifestMaterial>> =
            Vec::with_capacity(mesh.materials().len());

        for material_name in mesh.materials() {
            let material = match material_name {
                Some(name) => manifest.level.material(name),
                None => None,
            };
            mapped.push(material);
        }
        return mapped;
    }

    fn mesh_load(assets: &AssetManifest, mesh_href: &str) -> Result<GLTFMesh, LevelMeshLoadError> {
        let mesh_data = assets.asset_get(mesh_href).unwrap();
        return GLTFMesh::from_bytes(mesh_data).map_err(LevelMeshLoadError::GLTF);
    }

    fn model_build(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        mesh: &GLTFMesh,
        materials: &[Option<&LevelManifestMaterial>],
    ) -> Result<Model<LevelModelVertex>, LevelModelBuildError> {
        let mut vertices: Vec<LevelModelVertex> = Vec::new();
        for vertex in mesh.vertices() {
            let material_ix = match vertex.material_ix {
                Some(material_ix) => material_ix,
                None => return Err(LevelModelBuildError::MaterialIXMissing),
            };
            if !matches!(materials.get(material_ix as usize), Some(Some(_))) {
                return Err(LevelModelBuildError::MaterialConfigMissing);
            }
            vertices.push(LevelModelVertex {
                position: vertex.position,
                diffuse_uv: vertex.diffuse_uv.unwrap_or(Vec2::ZERO),
                material_ix,
            });
        }

        let mut model = Model::new(device, mesh.vertex_count());
        model
            .upload(queue, &vertices)
            .map_err(LevelModelBuildError::ModelUpload)?;
        return Ok(model);
    }

    fn track_load(
        assets: &AssetManifest,
        track_href: &str,
    ) -> Result<TrackData, LevelTrackLoadError> {
        let track_data = assets.asset_get(track_href).unwrap();
        return TrackData::new(track_data, true).map_err(LevelTrackLoadError::Decode);
    }

    fn collider_build(mesh: &GLTFMesh) -> Result<TriMesh, TriMeshFromPositionsError> {
        return TriMesh::from_positions(mesh.vertices().map(|vertex| vertex.position));
    }

    pub fn load(
        url: Url,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        asset_thread_pool: &ThreadPool,
    ) -> Result<Self, LevelLoadError> {
        let manifest = LevelManifest::load(&url).map_err(LevelLoadError::ManifestLoad)?;
        let assets = manifest
            .assets(&url, asset_thread_pool)
            .map_err(LevelLoadError::ManifestAssets)?;

        let level_model_mesh =
            Self::mesh_load(&assets, &manifest.level.model).map_err(LevelLoadError::MeshLoad)?;
        let material_index = Self::material_index_build(&manifest, &level_model_mesh);

        let material_result = level_material_load(device, queue, &assets, &material_index)
            .map_err(LevelLoadError::MaterialLoad)?;
        let model = Self::model_build(device, queue, &level_model_mesh, &material_index)
            .map_err(LevelLoadError::ModelBuild)?;

        let collider_href = match manifest.level.collider.as_deref() {
            Some(collider) => collider,
            None => manifest.level.model.as_str(),
        };
        let level_collider_mesh =
            Self::mesh_load(&assets, collider_href).map_err(LevelLoadError::MeshLoad)?;
        let collider =
            Self::collider_build(&level_collider_mesh).map_err(LevelLoadError::ColliderBuild)?;

        let mut portals = Vec::new();
        let mut portal_lookup = HashMap::new();
        for (name, manifest_portal) in manifest.portal_iter() {
            let portal_ix = portals.len();
            let portal = LevelPortal::load(LevelPortalLoadParams {
                name: name.clone(),
                portal_ix,
                base_url: &url,
                assets: &assets,
                collider_href: &manifest_portal.collider,
                target: manifest_portal.target.as_ref(),
                device,
                queue,
            })
            .map_err(LevelLoadError::PortalLoad)?;
            portal_lookup.insert(name.clone(), portal_ix);
            portals.push(portal);
        }

        let track = match manifest.level.track.as_deref() {
            Some(track_href) => {
                Some(Self::track_load(&assets, track_href).map_err(LevelLoadError::TrackLoad)?)
            }
            None => None,
        };

        return Ok(Self {
            manifest,
            collider,
            model,
            texture_bind_group: material_result.texture_bind_group,
            material_index_bind_group: material_result.material_index_bind_group,
            portals,
            portal_lookup,
            track,
        });
    }
}
