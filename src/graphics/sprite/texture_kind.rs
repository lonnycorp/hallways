#[derive(Debug, Clone, Copy)]
pub enum TextureKind {
    Text,
    System,
}

impl TextureKind {
    pub fn texture_ix(&self) -> u32 {
        return match self {
            TextureKind::Text => 0,
            TextureKind::System => 1,
        };
    }
}
