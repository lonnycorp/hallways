use rodio::decoder::DecoderError;

use crate::gltf::GLTFMeshError;
use crate::graphics::model::ModelUploadError;
use crate::parry3d::trimesh::TriMeshFromPositionsError;

use super::manifest::{FetchError, LevelManifestAssetsError, LevelManifestLoadError};
use super::material::LevelMaterialLoadError;
use super::portal::LevelPortalLoadError;

#[derive(Debug)]
pub enum LevelMeshLoadError {
    GLTF(GLTFMeshError),
}

#[derive(Debug)]
pub enum LevelTrackLoadError {
    Decode(DecoderError),
}

#[derive(Debug)]
pub enum LevelModelBuildError {
    MaterialIXMissing,
    MaterialConfigMissing,
    ModelUpload(ModelUploadError),
}

#[derive(Debug)]
pub enum LevelLoadError {
    ManifestLoad(LevelManifestLoadError),
    ManifestAssets(LevelManifestAssetsError),
    MeshLoad(LevelMeshLoadError),
    ColliderBuild(TriMeshFromPositionsError),
    MaterialLoad(LevelMaterialLoadError),
    ModelBuild(LevelModelBuildError),
    PortalLoad(LevelPortalLoadError),
    TrackLoad(LevelTrackLoadError),
}

fn fetch_error_fmt(
    f: &mut std::fmt::Formatter<'_>,
    context: &str,
    err: &FetchError,
) -> std::fmt::Result {
    return match err {
        FetchError::HTTP(err) => {
            write!(f, "failed to load {}: http fetch error ({})", context, err)
        }
        FetchError::IO(err) => {
            write!(f, "failed to load {}: io fetch error ({})", context, err)
        }
        FetchError::URLJoin(err) => {
            write!(f, "failed to load {}: URL join error ({})", context, err)
        }
        FetchError::InvalidScheme => {
            write!(f, "failed to load {}: invalid URL scheme", context)
        }
    };
}

impl std::fmt::Display for LevelLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            LevelLoadError::ManifestLoad(err) => match err {
                LevelManifestLoadError::Fetch(fetch_err) => {
                    fetch_error_fmt(f, "level manifest", fetch_err)
                }
                LevelManifestLoadError::UTF8(err) => {
                    write!(f, "failed to parse level manifest UTF-8: {}", err)
                }
                LevelManifestLoadError::Decode(err) => {
                    write!(f, "failed to decode level manifest JSON: {}", err)
                }
                LevelManifestLoadError::TooManyPortals => {
                    write!(f, "failed to parse level manifest: too many portals")
                }
                LevelManifestLoadError::InvalidVersion => {
                    write!(
                        f,
                        "failed to parse level manifest: invalid manifest version"
                    )
                }
            },
            LevelLoadError::ManifestAssets(err) => match err {
                LevelManifestAssetsError::Fetch(fetch_err) => {
                    fetch_error_fmt(f, "level asset manifest", fetch_err)
                }
            },
            LevelLoadError::MeshLoad(err) => match err {
                LevelMeshLoadError::GLTF(gltf_err) => match gltf_err {
                    GLTFMeshError::GLTF => {
                        write!(f, "failed to load level mesh: GLTF decode error")
                    }
                    GLTFMeshError::NoScene => {
                        write!(f, "failed to load level mesh: no scene in GLTF")
                    }
                    GLTFMeshError::MultipleScenes => {
                        write!(f, "failed to load level mesh: multiple scenes in GLTF")
                    }
                    GLTFMeshError::InconsistentDiffuseUVs => {
                        write!(
                            f,
                            "failed to load level mesh: inconsistent diffuse UV data in GLTF"
                        )
                    }
                    GLTFMeshError::InconsistentColors => {
                        write!(
                            f,
                            "failed to load level mesh: inconsistent vertex color data in GLTF"
                        )
                    }
                },
            },
            LevelLoadError::ColliderBuild(err) => match err {
                TriMeshFromPositionsError::EmptyVertexSet => {
                    write!(f, "failed to build level collider: empty vertex set")
                }
                TriMeshFromPositionsError::VertexCountNotMultipleOf3 => {
                    write!(
                        f,
                        "failed to build level collider: vertex count is not multiple of 3"
                    )
                }
            },
            LevelLoadError::MaterialLoad(err) => match err {
                LevelMaterialLoadError::ImageDecode(err) => {
                    write!(
                        f,
                        "failed to load level materials: image decode error ({})",
                        err
                    )
                }
                LevelMaterialLoadError::TextureBucketMissing { width, height } => {
                    write!(
                        f,
                        "failed to load level materials: texture bucket missing for {}x{}",
                        width, height
                    )
                }
                LevelMaterialLoadError::TextureArrayWrite(err) => {
                    write!(
                        f,
                        "failed to load level materials: texture write error ({:?})",
                        err
                    )
                }
                LevelMaterialLoadError::MaterialIndex(err) => {
                    write!(
                        f,
                        "failed to load level materials: material index write error ({:?})",
                        err
                    )
                }
            },
            LevelLoadError::ModelBuild(err) => match err {
                LevelModelBuildError::MaterialIXMissing => {
                    write!(
                        f,
                        "failed to build level model: material index missing on vertex"
                    )
                }
                LevelModelBuildError::MaterialConfigMissing => {
                    write!(
                        f,
                        "failed to build level model: material config missing for index"
                    )
                }
                LevelModelBuildError::ModelUpload(upload_err) => match upload_err {
                    ModelUploadError::VerticesExceedCapacity => {
                        write!(
                            f,
                            "failed to build level model: model upload vertices exceed capacity"
                        )
                    }
                },
            },
            LevelLoadError::PortalLoad(err) => match err {
                LevelPortalLoadError::URLJoin(parse_err) => {
                    write!(
                        f,
                        "failed to load level portals: URL join error ({})",
                        parse_err
                    )
                }
                LevelPortalLoadError::GLTF(gltf_err) => {
                    write!(
                        f,
                        "failed to load level portals: GLTF decode error ({:?})",
                        gltf_err
                    )
                }
                LevelPortalLoadError::GeometryFromGLTF(geometry_err) => {
                    write!(
                        f,
                        "failed to load level portals: portal geometry decode error ({:?})",
                        geometry_err
                    )
                }
                LevelPortalLoadError::ModelUpload(upload_err) => {
                    write!(
                        f,
                        "failed to load level portals: model upload error ({:?})",
                        upload_err
                    )
                }
                LevelPortalLoadError::Collider(collider_err) => {
                    write!(
                        f,
                        "failed to load level portals: collider build error ({:?})",
                        collider_err
                    )
                }
            },
            LevelLoadError::TrackLoad(err) => match err {
                LevelTrackLoadError::Decode(err) => {
                    write!(f, "failed to load level track: decode error ({})", err)
                }
            },
        };
    }
}
