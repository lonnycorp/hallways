use glam::Vec2;

use crate::color::Color;
use crate::graphics::pipeline::sprite::SpriteModelVertex;

use crate::graphics::sprite::{Sprite, TextureKind};

const GLYPH_WIDTH: f32 = 8.0;
const GLYPH_HEIGHT: f32 = 16.0;

pub const GLYPH_SIZE: Vec2 = Vec2::new(GLYPH_WIDTH, GLYPH_HEIGHT);

pub enum Glyph {
    Selector,
}

impl Glyph {
    fn uv_position(&self) -> Vec2 {
        return match self {
            Glyph::Selector => Vec2::new(0.0, 0.0),
        };
    }
}

pub struct SpriteGlyph {
    pub glyph: Glyph,
    pub position: Vec2,
    pub color: Color,
}

pub struct SpriteGlyphNewParams {
    pub glyph: Glyph,
    pub position: Vec2,
    pub color: Color,
}

impl SpriteGlyph {
    pub fn new(params: SpriteGlyphNewParams) -> Self {
        return Self {
            glyph: params.glyph,
            position: params.position,
            color: params.color,
        };
    }

    pub fn render(&self, buffer: &mut Vec<SpriteModelVertex>) {
        Sprite::new(
            self.glyph.uv_position(),
            GLYPH_SIZE,
            TextureKind::System,
            self.position,
            GLYPH_SIZE,
            self.color,
        )
        .render(buffer);
    }
}
