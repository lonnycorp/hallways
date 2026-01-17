use glam::Vec3;

use crate::color::Color;
use crate::gltf::GLTFVertex;

#[derive(Debug)]
pub enum LevelPortalGeometryFromGLTFError {
    InsufficientVertices,
    DegenerateGeometry,
    NotCoplanar,
    TiltedPortal,
    InconsistentColors,
    MissingAnchorColor,
    AmbiguousAnchorColor,
    UnstableAnchor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LevelPortalKind {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone)]
pub struct LevelPortalGeometry {
    pub center: Vec3,
    pub normal: Vec3,
    pub yaw: f32,
    pub kind: LevelPortalKind,
}

struct MergedVertex {
    position: Vec3,
    color: Option<Color>,
}

const EPSILON: f32 = 0.001;
const NORMAL_MATCH_EPSILON: f32 = 0.001;
const ANCHOR_COLOR: Color = Color::MAGENTA;

impl LevelPortalGeometry {
    pub fn from_gltf(
        vertices: impl Iterator<Item = GLTFVertex>,
    ) -> Result<LevelPortalGeometry, LevelPortalGeometryFromGLTFError> {
        let vertices: Vec<GLTFVertex> = vertices.collect();
        let mut merged: Vec<MergedVertex> = Vec::new();
        for vertex in &vertices {
            let existing = merged
                .iter()
                .find(|m| (m.position - vertex.position).length() < EPSILON);

            if let Some(existing) = existing {
                if existing.color != vertex.color {
                    return Err(LevelPortalGeometryFromGLTFError::InconsistentColors);
                }
            } else {
                merged.push(MergedVertex {
                    position: vertex.position,
                    color: vertex.color,
                });
            }
        }

        if merged.len() < 3 {
            return Err(LevelPortalGeometryFromGLTFError::InsufficientVertices);
        }

        let mut normal = None;
        for tri in 0..(vertices.len() / 3) {
            let a = vertices[tri * 3].position;
            let b = vertices[tri * 3 + 1].position;
            let c = vertices[tri * 3 + 2].position;
            let tri_normal = (b - a).cross(c - a);
            if tri_normal.length() > EPSILON {
                normal = Some(tri_normal.normalize());
                break;
            }
        }
        let normal = normal.ok_or(LevelPortalGeometryFromGLTFError::DegenerateGeometry)?;

        if normal.is_nan() {
            return Err(LevelPortalGeometryFromGLTFError::DegenerateGeometry);
        }

        let plane_point = merged[0].position;
        for v in &merged {
            let dist = (v.position - plane_point).dot(normal).abs();
            if dist > EPSILON {
                return Err(LevelPortalGeometryFromGLTFError::NotCoplanar);
            }
        }

        let mut center = Vec3::ZERO;
        for v in &merged {
            center += v.position;
        }
        center /= merged.len() as f32;

        if normal.y.abs() < EPSILON {
            let yaw = normal.x.atan2(normal.z);
            return Ok(LevelPortalGeometry {
                center,
                normal,
                yaw,
                kind: LevelPortalKind::Horizontal,
            });
        }
        if (normal.y - 1.0).abs() >= EPSILON && (normal.y + 1.0).abs() >= EPSILON {
            return Err(LevelPortalGeometryFromGLTFError::TiltedPortal);
        }

        let mut anchor = None;
        for v in &merged {
            if v.color == Some(ANCHOR_COLOR) {
                match anchor {
                    None => {
                        anchor = Some(v.position);
                    }
                    Some(existing) => {
                        if (existing - v.position).length() > EPSILON {
                            return Err(LevelPortalGeometryFromGLTFError::AmbiguousAnchorColor);
                        }
                    }
                }
            }
        }
        let anchor = anchor.ok_or(LevelPortalGeometryFromGLTFError::MissingAnchorColor)?;
        let reference_in_plane = Vec3::X - normal * Vec3::X.dot(normal);
        if reference_in_plane.length() < EPSILON {
            return Err(LevelPortalGeometryFromGLTFError::DegenerateGeometry);
        }
        let reference_in_plane = reference_in_plane.normalize();
        let center_to_anchor = anchor - center;
        if center_to_anchor.length() < EPSILON {
            return Err(LevelPortalGeometryFromGLTFError::UnstableAnchor);
        }
        let center_to_anchor = center_to_anchor.normalize();

        let roll = normal
            .dot(reference_in_plane.cross(center_to_anchor))
            .atan2(reference_in_plane.dot(center_to_anchor));

        return Ok(LevelPortalGeometry {
            center,
            normal,
            yaw: roll,
            kind: LevelPortalKind::Vertical,
        });
    }

    pub fn matches(&self, other: &LevelPortalGeometry) -> bool {
        return match (self.kind, other.kind) {
            (LevelPortalKind::Horizontal, LevelPortalKind::Horizontal) => true,
            (LevelPortalKind::Vertical, LevelPortalKind::Vertical) => {
                (self.normal - other.normal).length() <= NORMAL_MATCH_EPSILON
            }
            _ => false,
        };
    }
}
