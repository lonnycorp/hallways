use glam::Vec3;
use parry3d::shape::Cylinder;

const HEIGHT: f32 = 1.4;
const CROUCH_HEIGHT: f32 = 0.7;
const WIDTH: f32 = 0.6;
const SPEED: f32 = 14.0;
const CROUCH_SPEED: f32 = 4.0;

pub const GROUND_ACCEL: f32 = 80.0;
pub const AIR_ACCEL: f32 = 12.0;
pub const GROUND_FRICTION: f32 = 30.0;
pub const AIR_FRICTION: f32 = 1.0;
pub const FLOAT_ACCEL: f32 = GROUND_ACCEL;
pub const FLOAT_FRICTION: f32 = GROUND_FRICTION;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerMovementDynamics {
    pub speed: f32,
    pub accel: f32,
    pub friction: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerMovementState {
    Grounded { normal: Vec3, crouching: bool },
    Airborne { crouching: bool },
    Floating { normal: Option<Vec3> },
}

impl PlayerMovementState {
    fn height(&self) -> f32 {
        return match self {
            PlayerMovementState::Grounded { crouching, .. }
            | PlayerMovementState::Airborne { crouching } => {
                if *crouching {
                    CROUCH_HEIGHT
                } else {
                    HEIGHT
                }
            }
            PlayerMovementState::Floating { .. } => CROUCH_HEIGHT,
        };
    }

    pub fn collider(&self) -> Cylinder {
        let height = self.height();
        return Cylinder::new(height / 2.0, WIDTH / 2.0);
    }

    pub fn speed(&self) -> f32 {
        return match self {
            PlayerMovementState::Grounded { crouching, .. }
            | PlayerMovementState::Airborne { crouching } => {
                if *crouching {
                    CROUCH_SPEED
                } else {
                    SPEED
                }
            }
            PlayerMovementState::Floating { .. } => SPEED,
        };
    }

    pub fn properties(&self) -> PlayerMovementDynamics {
        let speed = self.speed();
        return match self {
            PlayerMovementState::Grounded { .. } => PlayerMovementDynamics {
                speed,
                accel: GROUND_ACCEL,
                friction: GROUND_FRICTION,
            },
            PlayerMovementState::Airborne { .. } => PlayerMovementDynamics {
                speed,
                accel: AIR_ACCEL,
                friction: AIR_FRICTION,
            },
            PlayerMovementState::Floating { .. } => PlayerMovementDynamics {
                speed,
                accel: FLOAT_ACCEL,
                friction: FLOAT_FRICTION,
            },
        };
    }
}
