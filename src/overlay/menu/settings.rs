mod item;
mod key;

use crate::config::Config;

use self::key::MenuSettingsKeyCache;

pub struct MenuSettings {
    hovered: usize,
    selected: bool,
    buffered_config: Config,
    key_cache: MenuSettingsKeyCache,
    default_url: String,
}

use glam::Vec2;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

use strum::{EnumCount, IntoEnumIterator};

use crate::app::AppStatus;
use crate::audio::Track;
use crate::graphics::pipeline::sprite::SpriteModelVertex;
use crate::graphics::sprite::{SpriteBorder, TEXT_SIZE};

use self::item::{MenuSettingsItem, MAX_ITEM_NAME_LEN, MAX_ITEM_VALUE_LEN};

const BORDER: f32 = 3.0;
const TEXT_PADDING: f32 = 3.0;
const SCREEN_PADDING: f32 = 6.0;
const INSET: f32 = BORDER + TEXT_PADDING;
const ITEM_INDENT: f32 = TEXT_SIZE.x + 2.0;

const ITEM_COUNT: usize = MenuSettingsItem::COUNT;
const ROW_WIDTH: f32 = ITEM_INDENT + (MAX_ITEM_NAME_LEN + MAX_ITEM_VALUE_LEN) as f32 * TEXT_SIZE.x;
const BOX_WIDTH: f32 = ROW_WIDTH + INSET * 2.0;
const BOX_HEIGHT: f32 = ITEM_COUNT as f32 * TEXT_SIZE.y + INSET * 2.0;

pub struct MenuSettingsStateRenderParams<'a> {
    pub buffer: &'a mut Vec<SpriteModelVertex>,
    pub status: AppStatus,
    pub tick: u64,
}

pub struct MenuSettingsStateKeyEventHandleParams<'a> {
    pub event: &'a KeyEvent,
    pub status: &'a AppStatus,
    pub status_next: &'a mut AppStatus,
    pub config: &'a mut Config,
    pub surface: &'a wgpu::Surface<'static>,
    pub device: &'a wgpu::Device,
    pub surface_config: &'a mut wgpu::SurfaceConfiguration,
    pub select_track: &'a Track,
    pub move_track: &'a Track,
}

impl MenuSettings {
    pub fn clear(&mut self, config: &Config) {
        self.buffered_config = config.clone();
        self.default_url = config.default_url.to_string();
        self.hovered = 0;
        self.selected = false;
    }

    pub fn new(config: &Config) -> Self {
        return Self {
            hovered: 0,
            selected: false,
            buffered_config: config.clone(),
            key_cache: MenuSettingsKeyCache::new(),
            default_url: config.default_url.to_string(),
        };
    }

    pub fn update(&mut self, status: AppStatus) {
        if !matches!(status, AppStatus::MenuSettings) {
            return;
        }
    }

    pub fn render(&mut self, params: &mut MenuSettingsStateRenderParams<'_>) {
        if !matches!(params.status, AppStatus::MenuSettings) {
            return;
        }

        let items: Vec<MenuSettingsItem> = MenuSettingsItem::iter().collect();

        let box_pos = Vec2::new(SCREEN_PADDING, SCREEN_PADDING);
        SpriteBorder::new(box_pos, Vec2::new(BOX_WIDTH, BOX_HEIGHT)).render(params.buffer);

        let content_x = box_pos.x + INSET;
        let content_y = box_pos.y + INSET;

        for (i, item) in items.iter().enumerate() {
            let y = content_y + i as f32 * TEXT_SIZE.y;
            let hovered = i == self.hovered;
            let active = hovered && self.selected;
            self.item_render(
                params.buffer,
                *item,
                Vec2::new(content_x, y),
                hovered,
                active,
                params.tick,
            );
        }
    }

    pub fn key_event(&mut self, params: &mut MenuSettingsStateKeyEventHandleParams<'_>) {
        if !matches!(params.status, AppStatus::MenuSettings) {
            return;
        }
        if !matches!(params.event.state, ElementState::Pressed) {
            return;
        }

        let items: Vec<MenuSettingsItem> = MenuSettingsItem::iter().collect();
        if self.selected {
            self.item_key_event(items[self.hovered], params);
            return;
        }

        match params.event.physical_key {
            PhysicalKey::Code(KeyCode::ArrowUp) => {
                self.hovered = (self.hovered + ITEM_COUNT - 1) % ITEM_COUNT;
                params.move_track.reset();
                params.move_track.play();
            }
            PhysicalKey::Code(KeyCode::ArrowDown) => {
                self.hovered = (self.hovered + 1) % ITEM_COUNT;
                params.move_track.reset();
                params.move_track.play();
            }
            PhysicalKey::Code(KeyCode::Escape) => {
                params.move_track.reset();
                params.move_track.play();
                *params.status_next = AppStatus::MenuHome;
            }
            PhysicalKey::Code(KeyCode::Enter) => {
                self.item_on_select(items[self.hovered], params);
            }
            _ => {}
        }
    }

    pub fn status_change_handle(&mut self, status: &AppStatus, config: &Config) {
        if matches!(status, AppStatus::MenuSettings) {
            self.clear(config);
        }
    }
}
