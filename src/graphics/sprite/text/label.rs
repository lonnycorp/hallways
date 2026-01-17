use glam::Vec2;

use crate::color::Color;
use crate::graphics::pipeline::sprite::SpriteModelVertex;

use super::{SpriteText, SpriteTextNewParams, TEXT_SIZE};

#[derive(Clone, Copy)]
pub enum SpriteLabelAlignment {
    Left,
    Right,
}

pub struct SpriteLabel<'a> {
    position: Vec2,
    max_len: usize,
    visible_len: usize,
    color: Color,
    bold: bool,
    alignment: SpriteLabelAlignment,
    text: &'a str,
}

impl<'a> SpriteLabel<'a> {
    pub fn new(
        position: Vec2,
        max_len: Option<usize>,
        color: Color,
        bold: bool,
        alignment: SpriteLabelAlignment,
        text: &'a str,
    ) -> Self {
        let text_len = text.chars().count();
        let max_len = max_len.unwrap_or(text_len);
        let visible_len = text_len.min(max_len);
        return Self {
            position,
            max_len,
            visible_len,
            color,
            bold,
            alignment,
            text,
        };
    }

    pub fn position(&self) -> Vec2 {
        let start_x = match self.alignment {
            SpriteLabelAlignment::Left => self.position.x,
            SpriteLabelAlignment::Right => {
                self.position.x + (self.max_len - self.visible_len) as f32 * TEXT_SIZE.x
            }
        };
        return Vec2::new(start_x, self.position.y);
    }

    pub fn size(&self) -> Vec2 {
        return Vec2::new(self.visible_len as f32 * TEXT_SIZE.x, TEXT_SIZE.y);
    }

    pub fn render(&self, buffer: &mut Vec<SpriteModelVertex>) {
        let text = self.text;
        let position = self.position;
        let start_x = match self.alignment {
            SpriteLabelAlignment::Left => position.x,
            SpriteLabelAlignment::Right => {
                position.x + (self.max_len - self.visible_len) as f32 * TEXT_SIZE.x
            }
        };
        let bold = self.bold;
        let color = self.color;
        for (i, c) in text.chars().take(self.visible_len).enumerate() {
            let char_position = Vec2::new(start_x + i as f32 * TEXT_SIZE.x, position.y);
            SpriteText::new(SpriteTextNewParams {
                c,
                bold,
                position: char_position,
                color,
            })
            .render(buffer);
        }
    }
}
