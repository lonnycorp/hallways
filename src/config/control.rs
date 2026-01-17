use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Clone, Copy, Debug, PartialEq, Eq, strum::EnumIter, strum::EnumCount)]
pub enum ConfigControl {
    Forward,
    Back,
    StrafeLeft,
    StrafeRight,
    Jump,
    Crouch,
}

impl ConfigControl {
    pub fn key_default(self) -> PhysicalKey {
        return match self {
            ConfigControl::Forward => PhysicalKey::Code(KeyCode::KeyW),
            ConfigControl::Back => PhysicalKey::Code(KeyCode::KeyS),
            ConfigControl::StrafeLeft => PhysicalKey::Code(KeyCode::KeyA),
            ConfigControl::StrafeRight => PhysicalKey::Code(KeyCode::KeyD),
            ConfigControl::Jump => PhysicalKey::Code(KeyCode::Space),
            ConfigControl::Crouch => PhysicalKey::Code(KeyCode::ControlLeft),
        };
    }
}
