use glam::Vec3;

use super::movement_state::PlayerMovementState;
use super::Player;
use crate::parry3d::CylinderExt;

const EPSILON: f32 = 0.001;

fn player_with_movement_state(movement_state: PlayerMovementState) -> Player {
    let mut player = Player::new(Vec3::ZERO);
    player.movement_state = movement_state;
    return player;
}

#[test]
fn movement_state_standing_collider_height_is_expected() {
    let player = player_with_movement_state(PlayerMovementState::Airborne { crouching: false });
    let collider = player.movement_state.collider();

    assert!((collider.height() - 1.4).abs() < EPSILON);
}

#[test]
fn movement_state_crouching_collider_height_is_expected() {
    let player = player_with_movement_state(PlayerMovementState::Airborne { crouching: true });
    let collider = player.movement_state.collider();

    assert!((collider.height() - 0.7).abs() < EPSILON);
}

#[test]
fn eye_position_uses_collider_half_height() {
    let mut player = Player::new(Vec3::new(1.0, 2.0, 3.0));
    player.movement_state = PlayerMovementState::Airborne { crouching: false };
    let collider = player.movement_state.collider();
    let eye = player.eye_position();

    assert!((eye.y - (2.0 + collider.half_height)).abs() < EPSILON);
}

#[test]
fn player_eye_position_matches_state_collider() {
    let mut player = Player::new(Vec3::new(0.0, 10.0, 0.0));
    player.movement_state = PlayerMovementState::Grounded {
        normal: Vec3::Y,
        crouching: false,
    };
    let eye = player.eye_position();

    assert!((eye.y - 10.7).abs() < EPSILON);
}

#[test]
fn clip_velocity_one_plane_overclips() {
    let velocity = Vec3::new(1.0, -1.0, 0.0);
    let clipped = Player::clip_velocity(velocity, &[Vec3::Y]);

    assert!((clipped.x - 1.0).abs() < EPSILON);
    assert!((clipped.y - 0.001).abs() < EPSILON);
    assert!(clipped.y > 0.0);
}

#[test]
fn clip_velocity_two_planes_uses_overclipped_crease() {
    let velocity = Vec3::new(1.0, 1.0, 2.0);
    let clipped = Player::clip_velocity(velocity, &[Vec3::X, Vec3::Y]);

    assert!(clipped.x.abs() < EPSILON);
    assert!(clipped.y.abs() < EPSILON);
    assert!((clipped.z - 2.002).abs() < EPSILON);
}

#[test]
fn clip_velocity_three_planes_zeroes_velocity() {
    let velocity = Vec3::new(1.0, 2.0, 3.0);
    let clipped = Player::clip_velocity(velocity, &[Vec3::X, Vec3::Y, Vec3::Z]);

    assert_eq!(clipped, Vec3::ZERO);
}

#[test]
fn movement_state_floating_none_collider_height_is_expected() {
    let player = player_with_movement_state(PlayerMovementState::Floating { normal: None });
    let collider = player.movement_state.collider();

    assert!((collider.height() - 0.7).abs() < EPSILON);
}

#[test]
fn movement_state_floating_some_collider_height_is_expected() {
    let player = player_with_movement_state(PlayerMovementState::Floating {
        normal: Some(Vec3::new(0.0, 1.0, 0.0)),
    });
    let collider = player.movement_state.collider();

    assert!((collider.height() - 0.7).abs() < EPSILON);
}

#[test]
fn movement_state_floating_properties_ignore_optional_normal() {
    let player_none = player_with_movement_state(PlayerMovementState::Floating { normal: None });
    let player_some = player_with_movement_state(PlayerMovementState::Floating {
        normal: Some(Vec3::new(0.0, 1.0, 0.0)),
    });
    let properties_none = player_none.movement_state.properties();
    let properties_some = player_some.movement_state.properties();

    assert_eq!(properties_none, properties_some);
}
