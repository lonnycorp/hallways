use std::collections::HashMap;

use crate::color::Color;
use crate::graphics::pipeline::level::{
    PipelineLevelBindGroupMaterialIndex, PipelineLevelBindGroupTexture, TEXTURE_BUCKETS,
};
use crate::graphics::storage::{
    MaterialIndexStorageBuffer, MaterialIndexStorageBufferData,
    MaterialIndexStorageBufferDataWriteError, MaterialTextureRef,
};
use crate::graphics::texture::{TextureArray, TextureArrayNewParams, TextureArrayWriteError};

use super::manifest::{
    AssetManifest, LevelManifestMaterial, LevelManifestMaterialTextureAddressing,
};

const DEFAULT_ANIMATION_SPEED: f32 = 0.5;
const DEFAULT_TEXTURE_ADDRESSING: LevelManifestMaterialTextureAddressing =
    LevelManifestMaterialTextureAddressing::Linear;
const TEXTURE_ADDRESSING_LINEAR: u32 = 0;
const TEXTURE_ADDRESSING_NEAREST: u32 = 1;

pub struct LevelMaterialLoadResult {
    pub texture_bind_group: PipelineLevelBindGroupTexture,
    pub material_index_bind_group: PipelineLevelBindGroupMaterialIndex,
}

#[derive(Debug)]
pub enum LevelMaterialLoadError {
    ImageDecode(image::ImageError),
    TextureBucketMissing { width: u32, height: u32 },
    TextureArrayWrite(TextureArrayWriteError),
    MaterialIndex(MaterialIndexStorageBufferDataWriteError),
}

fn find_texture_bucket(w: u32, h: u32) -> Option<usize> {
    return TEXTURE_BUCKETS
        .iter()
        .position(|b| b.width == w && b.height == h);
}

fn load_image<'a>(assets: &'a AssetManifest, href: &str) -> &'a [u8] {
    return assets.asset_get(href).unwrap();
}

fn texture_addressing_mode_get(addressing: LevelManifestMaterialTextureAddressing) -> u32 {
    return match addressing {
        LevelManifestMaterialTextureAddressing::Linear => TEXTURE_ADDRESSING_LINEAR,
        LevelManifestMaterialTextureAddressing::Nearest => TEXTURE_ADDRESSING_NEAREST,
    };
}

fn material_frame_refs_load(
    queue: &wgpu::Queue,
    assets: &AssetManifest,
    frame_paths: &[String],
    diffuse: &mut [TextureArray],
    next_free: &mut [usize; TEXTURE_BUCKETS.len()],
    texture_ref_cache: &mut HashMap<String, MaterialTextureRef>,
) -> Result<Vec<MaterialTextureRef>, LevelMaterialLoadError> {
    let mut frames: Vec<MaterialTextureRef> = Vec::with_capacity(frame_paths.len());

    for frame_path in frame_paths {
        if let Some(&cached_ref) = texture_ref_cache.get(frame_path) {
            frames.push(cached_ref);
            continue;
        }

        let frame_data = load_image(assets, frame_path);
        let img = image::load_from_memory(frame_data)
            .map_err(LevelMaterialLoadError::ImageDecode)?
            .to_rgba8();
        let (w, h) = img.dimensions();

        let bucket_ix =
            find_texture_bucket(w, h).ok_or(LevelMaterialLoadError::TextureBucketMissing {
                width: w,
                height: h,
            })?;
        let layer = next_free[bucket_ix];
        diffuse[bucket_ix]
            .write(queue, layer, &img)
            .map_err(LevelMaterialLoadError::TextureArrayWrite)?;
        next_free[bucket_ix] += 1;

        let texture_ref = MaterialTextureRef {
            bucket: bucket_ix as u16,
            layer: layer as u16,
        };
        texture_ref_cache.insert(frame_path.clone(), texture_ref);
        frames.push(texture_ref);
    }

    return Ok(frames);
}

pub fn level_material_load(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    assets: &AssetManifest,
    materials: &[Option<&LevelManifestMaterial>],
) -> Result<LevelMaterialLoadResult, LevelMaterialLoadError> {
    let mut diffuse = TEXTURE_BUCKETS.map(|b| {
        TextureArray::new(TextureArrayNewParams {
            device,
            dims: (b.width, b.height),
            size: b.layers,
        })
    });
    let mut material_index_data = MaterialIndexStorageBufferData::new();
    let mut next_free: [usize; TEXTURE_BUCKETS.len()] = [0; TEXTURE_BUCKETS.len()];
    let mut texture_ref_cache: HashMap<String, MaterialTextureRef> = HashMap::new();

    for (ix, material) in materials.iter().enumerate() {
        let material = match material {
            Some(material) => *material,
            None => continue,
        };
        let frame_paths = material.frames.as_deref().unwrap_or(&[]);
        let frames = material_frame_refs_load(
            queue,
            assets,
            frame_paths,
            &mut diffuse,
            &mut next_free,
            &mut texture_ref_cache,
        )?;
        let animation_speed = material.animation_speed.unwrap_or(DEFAULT_ANIMATION_SPEED);
        let color = material.color.unwrap_or(Color::WHITE);
        let texture_addressing = material
            .texture_addressing
            .unwrap_or(DEFAULT_TEXTURE_ADDRESSING);
        let texture_addressing_mode = texture_addressing_mode_get(texture_addressing);

        material_index_data
            .write(
                ix as u32,
                animation_speed,
                &frames,
                color,
                texture_addressing_mode,
            )
            .map_err(LevelMaterialLoadError::MaterialIndex)?;
    }

    let texture_bind_group = PipelineLevelBindGroupTexture::new(device, &diffuse);
    let material_index = MaterialIndexStorageBuffer::new(device);
    material_index.write(queue, &material_index_data);
    let material_index_bind_group =
        PipelineLevelBindGroupMaterialIndex::new(device, &material_index);

    return Ok(LevelMaterialLoadResult {
        texture_bind_group,
        material_index_bind_group,
    });
}
