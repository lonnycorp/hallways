mod item;

pub struct MenuHome {
    selected: usize,
}

use crate::app::AppStatus;
use crate::audio::Track;
use crate::graphics::pipeline::sprite::SpriteModelVertex;
use crate::graphics::sprite::{SpriteBorder, TEXT_SIZE};
use strum::{EnumCount, IntoEnumIterator};
use winit::event::{ElementState, KeyEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};

use self::item::{MenuHomeItem, MAX_ITEM_LEN};

const BORDER: f32 = 3.0;
const TEXT_PADDING: f32 = 3.0;
const SCREEN_PADDING: f32 = 6.0;
const INSET: f32 = BORDER + TEXT_PADDING;
const ITEM_INDENT: f32 = TEXT_SIZE.x + 2.0;

const ROW_WIDTH: f32 = ITEM_INDENT + MAX_ITEM_LEN as f32 * TEXT_SIZE.x;
const BOX_WIDTH: f32 = ROW_WIDTH + INSET * 2.0;
const BOX_HEIGHT: f32 = MenuHomeItem::COUNT as f32 * TEXT_SIZE.y + INSET * 2.0;

pub struct MenuHomeStateKeyboardEventHandleParams<'a> {
    pub event: &'a KeyEvent,
    pub event_loop: &'a ActiveEventLoop,
    pub status: &'a AppStatus,
    pub status_next: &'a mut AppStatus,
    pub select_track: &'a Track,
    pub move_track: &'a Track,
}

impl MenuHome {
    pub fn new() -> Self {
        return Self { selected: 0 };
    }

    pub fn key_event(&mut self, params: &mut MenuHomeStateKeyboardEventHandleParams<'_>) {
        if !matches!(params.status, AppStatus::MenuHome) {
            return;
        }
        if !matches!(params.event.state, ElementState::Pressed) {
            return;
        }

        match params.event.physical_key {
            PhysicalKey::Code(KeyCode::ArrowUp) => {
                self.selected = (self.selected + MenuHomeItem::COUNT - 1) % MenuHomeItem::COUNT;
                params.move_track.reset();
                params.move_track.play();
            }
            PhysicalKey::Code(KeyCode::ArrowDown) => {
                self.selected = (self.selected + 1) % MenuHomeItem::COUNT;
                params.move_track.reset();
                params.move_track.play();
            }
            PhysicalKey::Code(KeyCode::Escape) => {
                params.move_track.reset();
                params.move_track.play();
                *params.status_next = AppStatus::Simulation;
            }
            PhysicalKey::Code(KeyCode::Enter) => {
                if let Some(item) = MenuHomeItem::iter().nth(self.selected) {
                    self.item_on_select(item, params);
                }
            }
            _ => {}
        }
    }

    pub fn update(&mut self, status: AppStatus) {
        if !matches!(status, AppStatus::MenuHome) {
            return;
        }
    }

    pub fn render(&self, buffer: &mut Vec<SpriteModelVertex>, status: AppStatus) {
        if !matches!(status, AppStatus::MenuHome) {
            return;
        }

        let box_pos = glam::Vec2::new(SCREEN_PADDING, SCREEN_PADDING);
        SpriteBorder::new(box_pos, glam::Vec2::new(BOX_WIDTH, BOX_HEIGHT)).render(buffer);

        let content_x = box_pos.x + INSET;
        let content_y = box_pos.y + INSET;

        for (i, item) in MenuHomeItem::iter().enumerate() {
            let y = content_y + i as f32 * TEXT_SIZE.y;
            let hovered = i == self.selected;
            self.item_render(buffer, item, glam::Vec2::new(content_x, y), hovered);
        }
    }
}
