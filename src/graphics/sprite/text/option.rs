use glam::Vec2;

use super::TEXT_SIZE;
use super::{SpriteText, SpriteTextNewParams};
use crate::color::Color;
use crate::graphics::pipeline::sprite::SpriteModelVertex;
use crate::graphics::sprite::{Glyph, SpriteGlyph, SpriteGlyphNewParams};

const INDENT: f32 = TEXT_SIZE.x + 2.0;

pub enum OptionState {
    Disabled,
    Unselected,
    Selected,
}

pub struct SpriteTextOption<'a> {
    position: Vec2,
    max_len: usize,
    hovered: bool,
    state: OptionState,
    text: &'a str,
}

impl<'a> SpriteTextOption<'a> {
    pub fn new(
        position: Vec2,
        max_len: usize,
        hovered: bool,
        state: OptionState,
        text: &'a str,
    ) -> Self {
        return Self {
            position,
            max_len,
            hovered,
            state,
            text,
        };
    }

    pub fn render(&self, buffer: &mut Vec<SpriteModelVertex>) {
        let color = match self.state {
            OptionState::Disabled => Color::GRAY,
            OptionState::Unselected => Color::WHITE,
            OptionState::Selected => Color::CYAN,
        };

        let selector_color = match self.state {
            OptionState::Disabled => Color::GRAY,
            OptionState::Unselected => Color::WHITE,
            OptionState::Selected => Color::CYAN,
        };

        if self.hovered {
            SpriteGlyph::new(SpriteGlyphNewParams {
                glyph: Glyph::Selector,
                position: self.position,
                color: selector_color,
            })
            .render(buffer);
        }

        let len = self.text.len().min(self.max_len);
        let visible = &self.text[..len];
        let text_position = Vec2::new(self.position.x + INDENT, self.position.y);
        for (i, c) in visible.chars().enumerate() {
            let char_position =
                Vec2::new(text_position.x + i as f32 * TEXT_SIZE.x, text_position.y);
            SpriteText::new(SpriteTextNewParams {
                c,
                bold: false,
                position: char_position,
                color,
            })
            .render(buffer);
        }
    }
}
