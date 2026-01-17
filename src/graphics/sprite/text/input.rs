use glam::Vec2;

use crate::color::Color;
use crate::graphics::pipeline::sprite::SpriteModelVertex;

use super::{SpriteText, SpriteTextNewParams, TEXT_SIZE};

const TEXT_COLOR: Color = Color::WHITE;
const BLINK_PERIOD: u64 = 30;

pub struct SpriteTextInput<'a> {
    position: Vec2,
    max_len: usize,
    text: &'a str,
    active: bool,
    clock: u64,
}

pub struct SpriteTextInputNewParams<'a> {
    pub position: Vec2,
    pub max_len: usize,
    pub text: &'a str,
    pub active: bool,
    pub clock: u64,
}

impl<'a> SpriteTextInput<'a> {
    pub fn new(params: SpriteTextInputNewParams<'a>) -> Self {
        return Self {
            position: params.position,
            max_len: params.max_len,
            text: params.text,
            active: params.active,
            clock: params.clock,
        };
    }

    pub fn render(&self, buffer: &mut Vec<SpriteModelVertex>) {
        let color = TEXT_COLOR;
        let max_len_zero = self.max_len == 0;

        let visible: &str = if max_len_zero {
            ""
        } else if self.active {
            let start = self.text.len().saturating_sub(self.max_len - 1);
            &self.text[start..]
        } else {
            let end = self.text.len().min(self.max_len);
            &self.text[..end]
        };

        let start_x = self.position.x;
        let y = self.position.y;
        let mut visible_len = 0;
        for (i, c) in visible.chars().enumerate() {
            visible_len = i + 1;
            let position = Vec2::new(start_x + i as f32 * TEXT_SIZE.x, y);
            SpriteText::new(SpriteTextNewParams {
                c,
                bold: false,
                position,
                color,
            })
            .render(buffer);
        }

        let cursor_visible =
            !max_len_zero && self.active && (self.clock / BLINK_PERIOD).is_multiple_of(2);
        if cursor_visible {
            let cursor_x = start_x + visible_len as f32 * TEXT_SIZE.x;
            let cursor_pos = Vec2::new(cursor_x, y);
            SpriteText::new(SpriteTextNewParams {
                c: '_',
                bold: false,
                position: cursor_pos,
                color,
            })
            .render(buffer);
        }
    }
}
