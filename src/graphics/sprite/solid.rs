use glam::Vec2;

use crate::color::Color;
use crate::graphics::pipeline::sprite::SpriteModelVertex;

use super::{Sprite, TextureKind};

const UV_POSITION: Vec2 = Vec2::new(32.0, 0.0);
const UV_SIZE: Vec2 = Vec2::splat(16.0);

pub struct SpriteSolid {
    position: Vec2,
    size: Vec2,
    color: Color,
}

pub struct SpriteSolidNewParams {
    pub position: Vec2,
    pub size: Vec2,
    pub color: Color,
}

impl SpriteSolid {
    pub fn new(params: SpriteSolidNewParams) -> Self {
        return Self {
            position: params.position,
            size: params.size,
            color: params.color,
        };
    }

    pub fn render(&self, buffer: &mut Vec<SpriteModelVertex>) {
        Sprite::new(
            UV_POSITION,
            UV_SIZE,
            TextureKind::System,
            self.position,
            self.size,
            self.color,
        )
        .render(buffer);
    }
}
