mod border;
mod solid;
mod system;
mod text;
mod texture_kind;

pub use border::SpriteBorder;
pub use solid::{SpriteSolid, SpriteSolidNewParams};
pub use system::{Glyph, SpriteGlyph, SpriteGlyphNewParams, SpriteLogo};
pub use text::{
    OptionState, SpriteLabel, SpriteLabelAlignment, SpriteText, SpriteTextInput,
    SpriteTextInputNewParams, SpriteTextNewParams, SpriteTextOption, TEXT_SIZE,
};
pub use texture_kind::TextureKind;

use glam::Vec2;

use crate::color::Color;
use crate::graphics::pipeline::sprite::SpriteModelVertex;

// CCW winding: TL(0), BL(1), BR(2), TL(0), BR(2), TR(3)
const WINDING: [usize; 6] = [0, 1, 2, 0, 2, 3];

pub struct Sprite {
    uv_position: Vec2,
    uv_size: Vec2,
    texture_kind: TextureKind,
    color: Color,
    position: Vec2,
    size: Vec2,
}

impl Sprite {
    pub fn new(
        uv_position: Vec2,
        uv_size: Vec2,
        texture_kind: TextureKind,
        position: Vec2,
        size: Vec2,
        color: Color,
    ) -> Self {
        return Self {
            uv_position,
            uv_size,
            texture_kind,
            color,
            position,
            size,
        };
    }

    pub fn render(&self, buffer: &mut Vec<SpriteModelVertex>) {
        let min = self.position;
        let max = self.position + self.size;

        let uv_min = self.uv_position;
        let uv_max = self.uv_position + self.uv_size;
        let texture_ix = self.texture_kind.texture_ix();
        let color = self.color;

        // Corners: TL, BL, BR, TR
        let positions = [min, Vec2::new(min.x, max.y), max, Vec2::new(max.x, min.y)];
        let uvs = [
            uv_min,
            Vec2::new(uv_min.x, uv_max.y),
            uv_max,
            Vec2::new(uv_max.x, uv_min.y),
        ];

        for i in WINDING {
            buffer.push(SpriteModelVertex {
                position: positions[i],
                uv: uvs[i],
                texture_ix,
                color,
            });
        }
    }
}
