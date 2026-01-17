mod fetch;

#[cfg(test)]
mod test;

use glam::Vec3;
use rayon::prelude::*;
use rayon::ThreadPool;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use url::Url;

use crate::color::Color;

use fetch::fetch;
pub use fetch::FetchError;

const MANIFEST_VERSION: &str = "coco";
const MAX_PORTALS: usize = 4;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum LevelManifestMaterialTextureAddressing {
    Linear,
    Nearest,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LevelManifestMaterial {
    pub frames: Option<Vec<String>>,
    pub animation_speed: Option<f32>,
    pub color: Option<Color>,
    pub texture_addressing: Option<LevelManifestMaterialTextureAddressing>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct LevelManifestPortalTarget {
    pub href: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LevelManifestPortal {
    pub collider: String,
    pub target: Option<LevelManifestPortalTarget>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LevelManifestLevel {
    pub model: String,
    pub collider: Option<String>,
    pub track: Option<String>,
    pub spawn: Option<Vec3>,
    materials: HashMap<String, LevelManifestMaterial>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct LevelManifestMeta {
    pub name: String,
    pub author: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LevelManifest {
    #[serde(rename = "_version")]
    pub version: String,
    pub meta: LevelManifestMeta,
    pub level: LevelManifestLevel,
    portals: HashMap<String, LevelManifestPortal>,
}

pub struct AssetManifest {
    assets: HashMap<String, Vec<u8>>,
}

#[derive(Debug)]
pub enum LevelManifestLoadError {
    Fetch(FetchError),
    UTF8(std::str::Utf8Error),
    Decode(serde_json::Error),
    TooManyPortals,
    InvalidVersion,
}

#[derive(Debug)]
pub enum LevelManifestAssetsError {
    Fetch(FetchError),
}

impl LevelManifest {
    pub fn portal_iter(&self) -> impl Iterator<Item = (&String, &LevelManifestPortal)> {
        return self.portals.iter();
    }

    pub fn load(url: &Url) -> Result<Self, LevelManifestLoadError> {
        let data = fetch(url, "").map_err(LevelManifestLoadError::Fetch)?;
        let contents = std::str::from_utf8(&data).map_err(LevelManifestLoadError::UTF8)?;

        let manifest: LevelManifest =
            serde_json::from_str(contents).map_err(LevelManifestLoadError::Decode)?;

        if manifest.version != MANIFEST_VERSION {
            return Err(LevelManifestLoadError::InvalidVersion);
        }

        if manifest.portals.len() > MAX_PORTALS {
            return Err(LevelManifestLoadError::TooManyPortals);
        }

        return Ok(manifest);
    }

    pub fn assets(
        &self,
        base_url: &Url,
        asset_thread_pool: &ThreadPool,
    ) -> Result<AssetManifest, LevelManifestAssetsError> {
        let mut href_set: HashSet<String> = HashSet::new();
        href_set.insert(self.level.model.clone());
        if let Some(collider_href) = self.level.collider.as_deref() {
            href_set.insert(collider_href.to_string());
        }
        if let Some(track_href) = self.level.track.as_deref() {
            href_set.insert(track_href.to_string());
        }
        for (_, portal) in self.portal_iter() {
            href_set.insert(portal.collider.clone());
        }
        for material in self.level.material_iter() {
            if let Some(frame_hrefs) = material.frames.as_ref() {
                for frame_href in frame_hrefs {
                    href_set.insert(frame_href.clone());
                }
            }
        }

        let fetch_results: Vec<Result<(String, Vec<u8>), LevelManifestAssetsError>> =
            asset_thread_pool.install(|| {
                href_set
                    .into_par_iter()
                    .map(|href| {
                        let data =
                            fetch(base_url, &href).map_err(LevelManifestAssetsError::Fetch)?;
                        return Ok((href, data));
                    })
                    .collect()
            });

        let mut assets: HashMap<String, Vec<u8>> = HashMap::with_capacity(fetch_results.len());
        for result in fetch_results {
            let (href, data) = result?;
            assets.insert(href, data);
        }

        return Ok(AssetManifest { assets });
    }
}

impl LevelManifestLevel {
    pub fn material(&self, name: &str) -> Option<&LevelManifestMaterial> {
        return self.materials.get(name);
    }

    pub fn material_iter(&self) -> impl Iterator<Item = &LevelManifestMaterial> {
        return self.materials.values();
    }
}

impl AssetManifest {
    pub fn asset_get(&self, href: &str) -> Option<&[u8]> {
        return self.assets.get(href).map(Vec::as_slice);
    }
}
