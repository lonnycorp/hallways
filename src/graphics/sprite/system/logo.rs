use glam::Vec2;

use crate::color::Color;
use crate::graphics::pipeline::sprite::SpriteModelVertex;
use crate::graphics::sprite::{Sprite, TextureKind};

const UV_POSITION: Vec2 = Vec2::new(0.0, 16.0);
const UV_SIZE: Vec2 = Vec2::splat(480.0);

pub struct SpriteLogo {
    pub center: Vec2,
}

impl SpriteLogo {
    pub fn new(center: Vec2) -> Self {
        return Self { center };
    }

    pub fn render(&self, buffer: &mut Vec<SpriteModelVertex>) {
        let position = self.center - UV_SIZE / 2.0;
        Sprite::new(
            UV_POSITION,
            UV_SIZE,
            TextureKind::System,
            position,
            UV_SIZE,
            Color::WHITE,
        )
        .render(buffer);
    }
}
