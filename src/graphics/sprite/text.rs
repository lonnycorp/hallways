mod input;
mod label;
mod option;

pub use input::{SpriteTextInput, SpriteTextInputNewParams};
pub use label::{SpriteLabel, SpriteLabelAlignment};
pub use option::{OptionState, SpriteTextOption};

use glam::Vec2;

use crate::color::Color;
use crate::graphics::pipeline::sprite::SpriteModelVertex;
use crate::graphics::sprite::{Sprite, TextureKind};

const TEXT_WIDTH: f32 = 8.0;
const TEXT_HEIGHT: f32 = 16.0;
const CHARS_PER_ROW: usize = 16;
const FIRST_CHAR: usize = 32; // space
const BOLD_ROW_OFFSET: usize = 8;

pub const TEXT_SIZE: Vec2 = Vec2::new(TEXT_WIDTH, TEXT_HEIGHT);

pub struct SpriteText {
    c: char,
    bold: bool,
    position: Vec2,
    color: Color,
}

pub struct SpriteTextNewParams {
    pub c: char,
    pub bold: bool,
    pub position: Vec2,
    pub color: Color,
}

impl SpriteText {
    pub fn new(params: SpriteTextNewParams) -> Self {
        return Self {
            c: params.c,
            bold: params.bold,
            position: params.position,
            color: params.color,
        };
    }

    pub fn render(&self, buffer: &mut Vec<SpriteModelVertex>) {
        let code = (self.c as usize).wrapping_sub(FIRST_CHAR);
        let code = if code >= 96 { 0 } else { code };
        let row_offset = if self.bold { BOLD_ROW_OFFSET } else { 0 };
        let col = (FIRST_CHAR + code) % CHARS_PER_ROW;
        let row = (FIRST_CHAR + code) / CHARS_PER_ROW + row_offset;

        let uv_position = Vec2::new(col as f32 * TEXT_WIDTH, row as f32 * TEXT_HEIGHT);
        let uv_size = Vec2::new(TEXT_WIDTH, TEXT_HEIGHT);

        Sprite::new(
            uv_position,
            uv_size,
            TextureKind::Text,
            self.position,
            TEXT_SIZE,
            self.color,
        )
        .render(buffer);
    }
}
