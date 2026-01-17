use glam::Vec3;

use super::geometry::{LevelPortalGeometry, LevelPortalGeometryFromGLTFError, LevelPortalKind};
use super::link::LevelPortalLink;
use crate::color::Color;
use crate::gltf::GLTFMesh;

const WHITE_COLOR: Color = Color::WHITE;
const ANCHOR_COLOR: Color = Color::new(255, 0, 255, 255);

fn push_color(bytes: &mut Vec<u8>, color: Color) {
    bytes.push(color.r);
    bytes.push(color.g);
    bytes.push(color.b);
    bytes.push(color.a);
}

fn make_mesh_with_colors(positions: Vec<f32>, colors: Vec<u8>, indices: Vec<u32>) -> GLTFMesh {
    return GLTFMesh::new(positions, indices, None, Some(colors));
}

fn portal_geometry_new(
    center: Vec3,
    normal: Vec3,
    yaw: f32,
    kind: LevelPortalKind,
) -> LevelPortalGeometry {
    return LevelPortalGeometry {
        center,
        normal,
        yaw,
        kind,
    };
}

#[test]
fn rejects_insufficient_vertices() {
    let mesh = make_mesh_with_colors(
        vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        {
            let mut bytes = Vec::new();
            push_color(&mut bytes, ANCHOR_COLOR);
            push_color(&mut bytes, WHITE_COLOR);
            bytes
        },
        vec![0, 1],
    );
    let result = LevelPortalGeometry::from_gltf(mesh.vertices());
    assert!(matches!(
        result,
        Err(LevelPortalGeometryFromGLTFError::InsufficientVertices)
    ));
}

#[test]
fn rejects_non_coplanar_vertices() {
    let positions = vec![
        0.0, 0.0, 0.0, // anchor
        1.0, 0.0, 0.0, 1.0, 1.0, 0.5, // off plane
        0.0, 1.0, 0.0,
    ];
    let mut colors = Vec::new();
    push_color(&mut colors, ANCHOR_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    let mesh = make_mesh_with_colors(positions, colors, vec![0, 1, 2, 0, 2, 3]);

    let result = LevelPortalGeometry::from_gltf(mesh.vertices());
    assert!(matches!(
        result,
        Err(LevelPortalGeometryFromGLTFError::NotCoplanar)
    ));
}

#[test]
fn accepts_arbitrary_polygon_vertical_portal() {
    let positions = vec![
        1.0, 0.0, 0.0, // anchor
        0.5, 0.0, 0.8, -0.5, 0.0, 0.8, -1.0, 0.0, 0.0, -0.5, 0.0, -0.8, 0.5, 0.0, -0.8,
    ];
    let mut colors = Vec::new();
    push_color(&mut colors, ANCHOR_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    let indices = vec![0, 2, 1, 0, 3, 2, 0, 4, 3, 0, 5, 4];
    let mesh = make_mesh_with_colors(positions, colors, indices);

    let spec = LevelPortalGeometry::from_gltf(mesh.vertices()).unwrap();
    assert!(spec.matches(&portal_geometry_new(
        Vec3::ZERO,
        spec.normal,
        0.0,
        LevelPortalKind::Vertical,
    )));
}

#[test]
fn rejects_missing_anchor_color() {
    let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0];
    let mut colors = Vec::new();
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    let mesh = make_mesh_with_colors(positions, colors, vec![0, 1, 2, 0, 2, 3]);

    let result = LevelPortalGeometry::from_gltf(mesh.vertices());
    assert!(matches!(
        result,
        Err(LevelPortalGeometryFromGLTFError::MissingAnchorColor)
    ));
}

#[test]
fn rejects_ambiguous_anchor_color() {
    let positions = vec![
        0.0, 0.0, 0.0, // anchor 1
        1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 2.0, 0.0,
        0.0, // anchor 2 (different position)
    ];
    let mut colors = Vec::new();
    push_color(&mut colors, ANCHOR_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, ANCHOR_COLOR);
    let indices = vec![0, 1, 2, 0, 2, 3, 0, 4, 1];
    let mesh = make_mesh_with_colors(positions, colors, indices);

    let result = LevelPortalGeometry::from_gltf(mesh.vertices());
    assert!(matches!(
        result,
        Err(LevelPortalGeometryFromGLTFError::AmbiguousAnchorColor)
    ));
}

#[test]
fn rejects_unstable_anchor() {
    // Anchor at the centroid of all unique vertices.
    let positions = vec![
        0.0, 0.0, 0.0, // anchor
        1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0,
    ];
    let mut colors = Vec::new();
    push_color(&mut colors, ANCHOR_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    let indices = vec![0, 1, 3, 0, 3, 2, 0, 2, 4, 0, 4, 1];
    let mesh = make_mesh_with_colors(positions, colors, indices);

    let result = LevelPortalGeometry::from_gltf(mesh.vertices());
    assert!(matches!(
        result,
        Err(LevelPortalGeometryFromGLTFError::UnstableAnchor)
    ));
}

#[test]
fn horizontal_portal_computes_yaw() {
    let positions = vec![
        0.0, 0.0, 0.0, // anchor
        1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
    ];
    let mut colors = Vec::new();
    push_color(&mut colors, ANCHOR_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    let mesh = make_mesh_with_colors(positions, colors, vec![0, 1, 2, 0, 2, 3]);

    let spec = LevelPortalGeometry::from_gltf(mesh.vertices()).unwrap();
    assert!(spec.yaw.abs() < 0.001);
    assert!(spec.matches(&portal_geometry_new(
        Vec3::ZERO,
        spec.normal,
        0.0,
        LevelPortalKind::Horizontal,
    )));
}

#[test]
fn vertical_portal_uses_roll_for_yaw() {
    let positions = vec![
        0.0, 0.0, 0.0, // anchor
        1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0,
    ];
    let mut colors = Vec::new();
    push_color(&mut colors, ANCHOR_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    let mesh = make_mesh_with_colors(positions, colors, vec![0, 1, 2, 0, 2, 3]);

    let spec = LevelPortalGeometry::from_gltf(mesh.vertices()).unwrap();
    assert!((spec.yaw - (-2.3561945)).abs() < 0.001);
    assert!(spec.matches(&portal_geometry_new(
        Vec3::ZERO,
        spec.normal,
        0.0,
        LevelPortalKind::Vertical,
    )));
}

#[test]
fn rejects_tilted_portal() {
    let positions = vec![
        0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 3.0, 2.12, 2.12, 0.0, 2.12, 2.12,
    ];
    let mut colors = Vec::new();
    push_color(&mut colors, ANCHOR_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    push_color(&mut colors, WHITE_COLOR);
    let mesh = make_mesh_with_colors(positions, colors, vec![0, 1, 2, 0, 2, 3]);

    let result = LevelPortalGeometry::from_gltf(mesh.vertices());
    assert!(matches!(
        result,
        Err(LevelPortalGeometryFromGLTFError::TiltedPortal)
    ));
}

#[test]
fn wall_link_yaw_delta_uses_yaw() {
    let link = LevelPortalLink {
        portal_ix: 0,
        src: portal_geometry_new(Vec3::ZERO, Vec3::Z, 0.25, LevelPortalKind::Horizontal),
        dst: portal_geometry_new(Vec3::ZERO, Vec3::Z, 1.0, LevelPortalKind::Horizontal),
    };

    assert!((link.yaw_delta() - (1.0 - 0.25)).abs() < 0.001);
}

#[test]
fn vertical_link_yaw_delta_uses_vertical_yaw() {
    let link = LevelPortalLink {
        portal_ix: 0,
        src: portal_geometry_new(Vec3::ZERO, Vec3::Y, 0.25, LevelPortalKind::Vertical),
        dst: portal_geometry_new(Vec3::ZERO, Vec3::NEG_Y, 1.0, LevelPortalKind::Vertical),
    };

    assert!((link.yaw_delta() - (1.0 - 0.25)).abs() < 0.001);
}

#[test]
fn wall_link_position_transform_applies_translation_when_yaw_delta_is_zero() {
    let src_center = Vec3::new(2.0, 3.0, 4.0);
    let dst_center = Vec3::new(10.0, -1.0, 6.0);
    let link = LevelPortalLink {
        portal_ix: 0,
        src: portal_geometry_new(src_center, Vec3::Z, 0.0, LevelPortalKind::Horizontal),
        dst: portal_geometry_new(dst_center, Vec3::Z, 0.0, LevelPortalKind::Horizontal),
    };

    let pos = src_center + Vec3::new(0.6, 0.4, -0.2);
    let transformed = link.position_transform(pos);
    let expected = dst_center + (pos - src_center);
    assert!((transformed - expected).length() < 0.000001);
}

#[test]
fn vertical_link_position_transform_applies_translation_when_yaw_delta_is_zero() {
    let src_center = Vec3::new(-3.0, 0.0, 7.0);
    let dst_center = Vec3::new(8.0, 10.0, -2.0);
    let link = LevelPortalLink {
        portal_ix: 0,
        src: portal_geometry_new(src_center, Vec3::Y, 0.25, LevelPortalKind::Vertical),
        dst: portal_geometry_new(dst_center, Vec3::NEG_Y, 0.25, LevelPortalKind::Vertical),
    };

    let pos = src_center + Vec3::new(0.6, 0.0, -0.2);
    let transformed = link.position_transform(pos);
    let expected = dst_center + (pos - src_center);
    assert!((transformed - expected).length() < 0.000001);
}

#[test]
fn position_transform_maps_src_center_to_dst_center() {
    let link = LevelPortalLink {
        portal_ix: 0,
        src: portal_geometry_new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::Z,
            0.0,
            LevelPortalKind::Horizontal,
        ),
        dst: portal_geometry_new(
            Vec3::new(8.0, 9.0, 10.0),
            Vec3::Z,
            0.0,
            LevelPortalKind::Horizontal,
        ),
    };

    let transformed = link.position_transform(Vec3::new(1.0, 2.0, 3.0));

    assert_eq!(transformed, link.dst.center);
}

#[test]
fn position_transform_is_pure_rotation_around_y_with_translation() {
    let src_center = Vec3::new(0.0, 0.0, 0.0);
    let link = LevelPortalLink {
        portal_ix: 0,
        src: portal_geometry_new(src_center, Vec3::Z, 0.0, LevelPortalKind::Horizontal),
        dst: portal_geometry_new(src_center, Vec3::X, 1.0, LevelPortalKind::Horizontal),
    };

    let pos = src_center + Vec3::new(1.0, 2.0, -0.75);
    let transformed = link.position_transform(pos);
    let local = pos - src_center;
    let transformed_local = transformed - src_center;

    assert!((transformed_local.length() - local.length()).abs() < 0.000001);
    assert!((transformed_local.y - local.y).abs() < 0.000001);
}

#[test]
fn geometry_matches_horizontal_with_horizontal() {
    let a = portal_geometry_new(Vec3::ZERO, Vec3::Z, 0.0, LevelPortalKind::Horizontal);
    let b = portal_geometry_new(
        Vec3::new(10.0, 0.0, 0.0),
        Vec3::X,
        1.0,
        LevelPortalKind::Horizontal,
    );

    assert!(a.matches(&b));
}

#[test]
fn geometry_rejects_mixed_kinds() {
    let a = portal_geometry_new(Vec3::ZERO, Vec3::Z, 0.0, LevelPortalKind::Horizontal);
    let b = portal_geometry_new(Vec3::ZERO, Vec3::Y, 0.0, LevelPortalKind::Vertical);

    assert!(!a.matches(&b));
}

#[test]
fn geometry_matches_vertical_same_normal_within_epsilon() {
    let a = portal_geometry_new(Vec3::ZERO, Vec3::Y, 0.0, LevelPortalKind::Vertical);
    let b = portal_geometry_new(
        Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(0.0005, 0.9999999, 0.0).normalize_or_zero(),
        0.3,
        LevelPortalKind::Vertical,
    );

    assert!(a.matches(&b));
}

#[test]
fn geometry_rejects_vertical_different_normal() {
    let a = portal_geometry_new(Vec3::ZERO, Vec3::Y, 0.0, LevelPortalKind::Vertical);
    let b = portal_geometry_new(Vec3::ZERO, Vec3::NEG_Y, 0.0, LevelPortalKind::Vertical);

    assert!(!a.matches(&b));
}
