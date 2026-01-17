use super::*;
use rayon::ThreadPoolBuilder;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

fn load_manifest_bytes(data: &[u8]) -> Result<LevelManifest, LevelManifestLoadError> {
    let directory = temp_dir_new("load");
    let manifest_path = directory.join("level.json");
    fs::write(&manifest_path, data).unwrap();
    let url = Url::from_file_path(&manifest_path).unwrap();
    let manifest = LevelManifest::load(&url);
    fs::remove_dir_all(directory).unwrap();
    return manifest;
}

fn load_manifest_json(json: &str) -> Result<LevelManifest, LevelManifestLoadError> {
    return load_manifest_bytes(json.as_bytes());
}

fn temp_dir_new(name: &str) -> PathBuf {
    let mut directory = std::env::temp_dir();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    directory.push(format!(
        "hallways_manifest_assets_{}_{}_{}",
        name,
        std::process::id(),
        nanos
    ));
    fs::create_dir_all(&directory).unwrap();
    return directory;
}

#[test]
fn test_valid_manifest_parsing() {
    let json = r#"{
        "_version": "coco",
        "meta": {
            "name": "Test Level",
            "author": "Test Author"
        },
        "level": {
            "model": "level.glb",
            "spawn": [1.0, 2.0, 3.0],
            "materials": {}
        },
        "portals": {
            "portal_a": {
                "collider": "portal_a.glb",
                "target": { "href": "other.json", "name": "portal_b" }
            }
        }
    }"#;

    let result = load_manifest_json(json);
    assert!(result.is_ok());

    let manifest = result.unwrap();
    assert_eq!(manifest.meta.name, "Test Level");
    assert_eq!(manifest.meta.author.as_deref(), Some("Test Author"));
    assert_eq!(manifest.portal_iter().count(), 1);
    assert!(manifest.portal_iter().any(|(name, _)| name == "portal_a"));
    assert_eq!(manifest.level.model, "level.glb");
    assert_eq!(manifest.level.spawn, Some(glam::Vec3::new(1.0, 2.0, 3.0)));
}

#[test]
fn test_valid_manifest_without_optional_meta_fields() {
    let json = r#"{
        "_version": "coco",
        "meta": {
            "name": "Test Level"
        },
        "level": {
            "model": "level.glb",
            "spawn": [0.0, 0.0, 0.0],
            "materials": {}
        },
        "portals": {
            "p1": { "collider": "p1.glb", "target": { "href": "a.json", "name": "x" } }
        }
    }"#;

    let result = load_manifest_json(json);
    assert!(result.is_ok());

    let manifest = result.unwrap();
    assert_eq!(manifest.meta.name, "Test Level");
    assert!(manifest.meta.author.as_deref().is_none());
}

#[test]
fn test_portal_without_target_is_accepted() {
    let json = r#"{
        "_version": "coco",
        "meta": {
            "name": "Test Level"
        },
        "level": {
            "model": "level.glb",
            "materials": {}
        },
        "portals": {
            "p1": { "collider": "p1.glb" }
        }
    }"#;

    let manifest = load_manifest_json(json).unwrap();
    let portal = manifest.portal_iter().next().unwrap().1;
    assert!(portal.target.is_none());
}

#[test]
fn test_portal_target_missing_name_is_rejected() {
    let json = r#"{
        "_version": "coco",
        "meta": {
            "name": "Test Level"
        },
        "level": {
            "model": "level.glb",
            "materials": {}
        },
        "portals": {
            "p1": { "collider": "p1.glb", "target": { "href": "a.json" } }
        }
    }"#;

    let result = load_manifest_json(json);
    assert!(matches!(result, Err(LevelManifestLoadError::Decode(_))));
}

#[test]
fn test_manifest_without_material_is_rejected() {
    let json = r#"{
        "_version": "coco",
        "meta": {
            "name": "Test Level"
        },
        "level": {
            "model": "level.glb",
            "spawn": [0.0, 0.0, 0.0]
        },
        "portals": {
            "p1": { "collider": "p1.glb", "target": { "href": "a.json", "name": "x" } }
        }
    }"#;

    let result = load_manifest_json(json);
    assert!(matches!(result, Err(LevelManifestLoadError::Decode(_))));
}

#[test]
fn test_manifest_without_portals_is_rejected() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "spawn": [0.0, 0.0, 0.0],
            "materials": {}
        }
    }"#;

    let result = load_manifest_json(json);
    assert!(matches!(result, Err(LevelManifestLoadError::Decode(_))));
}

#[test]
fn test_old_level_mesh_and_surface_fields_rejected() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "mesh": "level.glb",
            "surface": {}
        }
    }"#;

    let result = load_manifest_json(json);
    assert!(matches!(result, Err(LevelManifestLoadError::Decode(_))));
}

#[test]
fn test_old_portal_mesh_field_rejected() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "materials": {}
        },
        "portals": {
            "p1": { "mesh": "p1.glb", "target": { "href": "a.json", "name": "x" } }
        }
    }"#;

    let result = load_manifest_json(json);
    assert!(matches!(result, Err(LevelManifestLoadError::Decode(_))));
}

#[test]
fn test_invalid_json_returns_decode_error() {
    let result = load_manifest_bytes(b"{ invalid json }");
    assert!(matches!(result, Err(LevelManifestLoadError::Decode(_))));
}

#[test]
fn test_invalid_utf8_returns_utf8_error() {
    let result = load_manifest_bytes(&[0xff, 0xfe, 0xfd]);
    assert!(matches!(result, Err(LevelManifestLoadError::UTF8(_))));
}

#[test]
fn test_too_many_portals_returns_error() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "spawn": [0.0, 0.0, 0.0],
            "materials": {}
        },
        "portals": {
            "p1": { "collider": "p1.glb", "target": { "href": "a.json", "name": "x" } },
            "p2": { "collider": "p2.glb", "target": { "href": "a.json", "name": "x" } },
            "p3": { "collider": "p3.glb", "target": { "href": "a.json", "name": "x" } },
            "p4": { "collider": "p4.glb", "target": { "href": "a.json", "name": "x" } },
            "p5": { "collider": "p5.glb", "target": { "href": "a.json", "name": "x" } }
        }
    }"#;

    let result = load_manifest_json(json);
    assert!(matches!(
        result,
        Err(LevelManifestLoadError::TooManyPortals)
    ));
}

#[test]
fn test_invalid_version_returns_error() {
    let json = r#"{
        "_version": "wrong",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "spawn": [0.0, 0.0, 0.0],
            "materials": {}
        },
        "portals": {}
    }"#;

    let result = load_manifest_json(json);
    assert!(matches!(
        result,
        Err(LevelManifestLoadError::InvalidVersion)
    ));
}

#[test]
fn test_spawn_missing_is_none() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "materials": {}
        },
        "portals": {
            "portal_a": { "collider": "p1.glb", "target": { "href": "a.json", "name": "x" } }
        }
    }"#;

    let result = load_manifest_json(json);
    assert!(result.is_ok());
    let manifest = result.unwrap();
    assert_eq!(manifest.level.spawn, None);
}

#[test]
fn test_spawn_accepts_coordinates() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "spawn": [-12.5, 0.0, 3.75],
            "materials": {}
        },
        "portals": {
            "portal_a": { "collider": "p1.glb", "target": { "href": "a.json", "name": "x" } },
            "portal_b": { "collider": "p2.glb", "target": { "href": "b.json", "name": "x" } }
        }
    }"#;

    let result = load_manifest_json(json);
    assert!(result.is_ok());
    let manifest = result.unwrap();
    assert_eq!(
        manifest.level.spawn,
        Some(glam::Vec3::new(-12.5, 0.0, 3.75))
    );
}

#[test]
fn test_level_optional_collider_path_parses() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "render.glb",
            "collider": "collision.glb",
            "materials": {}
        },
        "portals": {
            "portal_a": { "collider": "p1.glb", "target": { "href": "a.json", "name": "x" } }
        }
    }"#;

    let result = load_manifest_json(json);
    assert!(result.is_ok());
    let manifest = result.unwrap();
    assert_eq!(manifest.level.model, "render.glb");
    assert_eq!(manifest.level.collider.as_deref(), Some("collision.glb"));
}

#[test]
fn test_material_parses_optional_fields() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "spawn": [0.0, 0.0, 0.0],
            "materials": {
                "wall": {
                    "frames": ["wall.png"],
                    "color": [255, 255, 255, 255]
                }
            }
        },
        "portals": {}
    }"#;

    let result = load_manifest_json(json);
    assert!(result.is_ok());
    let manifest = result.unwrap();

    let material = manifest.level.material("wall").unwrap();
    let frames = material.frames.as_ref().unwrap();
    assert_eq!(frames.len(), 1);
    assert_eq!(frames[0], "wall.png");
    assert_eq!(material.color, Some(crate::color::Color::WHITE));
}

#[test]
fn test_material_missing_frames_is_accepted() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "spawn": [0.0, 0.0, 0.0],
            "materials": {
                "wall": {}
            }
        },
        "portals": {}
    }"#;

    let result = load_manifest_json(json);
    assert!(result.is_ok());
    let manifest = result.unwrap();

    let material = manifest.level.material("wall").unwrap();
    assert!(material.frames.is_none());
}

#[test]
fn test_material_optional_appearance_fields_are_accepted() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "spawn": [0.0, 0.0, 0.0],
            "materials": {
                "wall": {
                    "frames": []
                }
            }
        },
        "portals": {}
    }"#;

    let manifest = load_manifest_json(json).unwrap();
    let material = manifest.level.material("wall").unwrap();
    let frames = material.frames.as_ref().unwrap();
    assert!(frames.is_empty());
    assert!(material.animation_speed.is_none());
    assert_eq!(material.color, None);
    assert_eq!(material.texture_addressing, None);
}

#[test]
fn test_material_appearance_fields_parse() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "spawn": [0.0, 0.0, 0.0],
            "materials": {
                "wall": {
                    "frames": ["wall.png"],
                    "color": [1, 2, 3, 4]
                },
                "lava": {
                    "frames": ["lava_0.png", "lava_1.png"],
                    "animation_speed": 2.5,
                    "color": [10, 20, 30, 40]
                },
                "paint": {
                    "frames": [],
                    "color": [7, 8, 9, 10]
                }
            }
        },
        "portals": {}
    }"#;

    let result = load_manifest_json(json);
    assert!(result.is_ok());
    let manifest = result.unwrap();

    let wall = manifest.level.material("wall").unwrap();
    let wall_frames = wall.frames.as_ref().unwrap();
    assert_eq!(wall_frames.len(), 1);
    assert_eq!(wall_frames[0], "wall.png");
    assert_eq!(wall.color, Some(crate::color::Color::new(1, 2, 3, 4)));

    let lava = manifest.level.material("lava").unwrap();
    let lava_frames = lava.frames.as_ref().unwrap();
    assert_eq!(lava_frames.len(), 2);
    assert_eq!(lava_frames[0], "lava_0.png");
    assert_eq!(lava_frames[1], "lava_1.png");
    assert_eq!(lava.animation_speed, Some(2.5));
    assert_eq!(lava.color, Some(crate::color::Color::new(10, 20, 30, 40)));

    let paint = manifest.level.material("paint").unwrap();
    let paint_frames = paint.frames.as_ref().unwrap();
    assert!(paint_frames.is_empty());
    assert_eq!(paint.color, Some(crate::color::Color::new(7, 8, 9, 10)));
    assert_eq!(paint.texture_addressing, None);
}

#[test]
fn test_material_texture_addressing_parses() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "materials": {
                "wall": {
                    "frames": ["wall.png"],
                    "texture_addressing": "Linear"
                },
                "pixel": {
                    "frames": ["pixel.png"],
                    "texture_addressing": "Nearest"
                }
            }
        },
        "portals": {}
    }"#;

    let manifest = load_manifest_json(json).unwrap();
    let wall = manifest.level.material("wall").unwrap();
    assert_eq!(
        wall.texture_addressing,
        Some(LevelManifestMaterialTextureAddressing::Linear)
    );

    let pixel = manifest.level.material("pixel").unwrap();
    assert_eq!(
        pixel.texture_addressing,
        Some(LevelManifestMaterialTextureAddressing::Nearest)
    );
}

#[test]
fn test_material_texture_addressing_invalid_value_is_rejected() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "materials": {
                "wall": {
                    "frames": ["wall.png"],
                    "texture_addressing": "nearest"
                }
            }
        },
        "portals": {}
    }"#;

    let result = load_manifest_json(json);
    assert!(matches!(result, Err(LevelManifestLoadError::Decode(_))));
}

#[test]
fn test_material_empty_frames_is_accepted() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "spawn": [0.0, 0.0, 0.0],
            "materials": {
                "lava": {
                    "frames": [],
                    "animation_speed": 1.0,
                    "color": [255, 255, 255, 255]
                }
            }
        },
        "portals": {}
    }"#;

    let manifest = load_manifest_json(json).unwrap();
    let material = manifest.level.material("lava").unwrap();
    let frames = material.frames.as_ref().unwrap();
    assert!(frames.is_empty());
}

#[test]
fn test_old_material_schema_is_rejected() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "spawn": [0.0, 0.0, 0.0],
            "materials": {
                "wall": {
                    "image": "wall.png",
                    "collider_type": "wall"
                }
            }
        },
        "portals": {}
    }"#;

    let result = load_manifest_json(json);
    assert!(matches!(result, Err(LevelManifestLoadError::Decode(_))));
}

#[test]
fn test_invisible_material_type_is_rejected() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "materials": {
                "ghost": {
                    "type": "Invisible"
                }
            }
        },
        "portals": {}
    }"#;

    let result = load_manifest_json(json);
    assert!(matches!(result, Err(LevelManifestLoadError::Decode(_))));
}

#[test]
fn test_manifest_load_accepts_reused_asset_paths() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "track": "music.ogg",
            "spawn": [0.0, 0.0, 0.0],
            "materials": {
                "wall": {
                    "frames": ["shared.png"],
                    "color": [255, 255, 255, 255]
                },
                "lava": {
                    "frames": ["shared.png", "lava_1.png"],
                    "animation_speed": 2.5,
                    "color": [255, 255, 255, 255]
                }
            }
        },
        "portals": {
            "p1": { "collider": "portal_a.glb", "target": { "href": "a.json", "name": "x" } },
            "p2": { "collider": "portal_a.glb", "target": { "href": "b.json", "name": "x" } }
        }
    }"#;

    let manifest = load_manifest_json(json).unwrap();
    assert_eq!(manifest.level.model, "level.glb");
    assert_eq!(manifest.portal_iter().count(), 2);
}

#[test]
fn test_level_color_is_rejected() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "color": [12, 34, 56, 255],
            "materials": {}
        },
        "portals": {}
    }"#;

    let result = load_manifest_json(json);
    assert!(matches!(result, Err(LevelManifestLoadError::Decode(_))));
}

#[test]
fn test_assets_collects_manifest_hrefs_and_dedupes() {
    let json = r#"{
        "_version": "coco",
        "meta": { "name": "Test Level" },
        "level": {
            "model": "level.glb",
            "collider": "level.glb",
            "track": "music.ogg",
            "materials": {
                "wall": {
                    "frames": ["shared.png", "shared.png"],
                    "color": [255, 255, 255, 255]
                }
            }
        },
        "portals": {
            "p1": { "collider": "portal.glb", "target": { "href": "a.json", "name": "x" } },
            "p2": { "collider": "portal.glb", "target": { "href": "b.json", "name": "x" } }
        }
    }"#;

    let directory = temp_dir_new("dedupe");
    fs::write(directory.join("level.glb"), b"mesh").unwrap();
    fs::write(directory.join("music.ogg"), b"track").unwrap();
    fs::write(directory.join("shared.png"), b"frame").unwrap();
    fs::write(directory.join("portal.glb"), b"portal").unwrap();

    let manifest = load_manifest_json(json).unwrap();
    let base_url = Url::from_file_path(directory.join("level.json")).unwrap();
    let asset_thread_pool = ThreadPoolBuilder::new().num_threads(4).build().unwrap();
    let assets = manifest.assets(&base_url, &asset_thread_pool).unwrap();

    assert_eq!(assets.asset_get("level.glb").unwrap(), b"mesh");
    assert_eq!(assets.asset_get("music.ogg").unwrap(), b"track");
    assert_eq!(assets.asset_get("shared.png").unwrap(), b"frame");
    assert_eq!(assets.asset_get("portal.glb").unwrap(), b"portal");

    fs::remove_dir_all(directory).unwrap();
}
