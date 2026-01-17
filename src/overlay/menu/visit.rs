mod item;

use url::Url;

pub struct MenuVisit {
    hovered: usize,
    selected: bool,
    visiting: Option<Url>,
    level_url: String,
    visit_status: Option<MenuVisitLoadStatus>,
}

#[derive(PartialEq, Eq, Clone)]
enum MenuVisitLoadStatus {
    Loading,
    Failed { message: String },
    Ready,
}

use crate::app::AppStatus;
use crate::audio::{CrossFader, Track};
use crate::color::Color;
use crate::config::Config;
use crate::graphics::pipeline::sprite::SpriteModelVertex;
use crate::graphics::sprite::{SpriteBorder, SpriteLabel, SpriteLabelAlignment, TEXT_SIZE};
use crate::level::cache::{LevelCache, LevelCacheResult};
use crate::overlay::{Log, LogData};
use crate::player::{Player, PlayerSpawnParams};
use glam::Vec2;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

use strum::{EnumCount, IntoEnumIterator};

use self::item::{MenuVisitItem, MAX_ITEM_NAME_LEN, MAX_ITEM_VALUE_LEN};

const BORDER: f32 = 3.0;
const TEXT_PADDING: f32 = 3.0;
const SCREEN_PADDING: f32 = 6.0;
const INSET: f32 = BORDER + TEXT_PADDING;
const ITEM_COUNT: usize = MenuVisitItem::COUNT;
const ITEM_INDENT: f32 = TEXT_SIZE.x + 2.0;
const ROW_WIDTH: f32 = ITEM_INDENT + (MAX_ITEM_NAME_LEN + MAX_ITEM_VALUE_LEN) as f32 * TEXT_SIZE.x;
const BOX_WIDTH: f32 = ROW_WIDTH + INSET * 2.0;
const BOX_HEIGHT: f32 = ITEM_COUNT as f32 * TEXT_SIZE.y + INSET * 2.0;
const STATUS_MAX_CHARS: usize = ((BOX_WIDTH - INSET * 2.0) / TEXT_SIZE.x) as usize;
const LOADING_MESSAGE: &str = "Loading...";
const WHITE: Color = Color::WHITE;

pub struct MenuVisitStateUpdateParams<'a> {
    pub status: &'a AppStatus,
    pub status_next: &'a mut AppStatus,
    pub player: &'a mut Player,
    pub cross_fader: &'a mut CrossFader,
    pub log: &'a mut Log,
    pub cache: &'a mut LevelCache,
    pub tick: u64,
}

pub struct MenuVisitStateRenderParams<'a> {
    pub buffer: &'a mut Vec<SpriteModelVertex>,
    pub status: AppStatus,
    pub tick: u64,
}

pub struct MenuVisitStateKeyEventHandleParams<'a> {
    pub event: &'a KeyEvent,
    pub status: &'a AppStatus,
    pub status_next: &'a mut AppStatus,
    pub select_track: &'a Track,
    pub move_track: &'a Track,
    pub player: &'a mut Player,
    pub cross_fader: &'a mut CrossFader,
    pub cache: &'a mut LevelCache,
    pub tick: u64,
}

impl MenuVisit {
    pub fn clear(&mut self) {
        self.hovered = 0;
        self.selected = false;
        self.visiting = None;
        self.visit_status = None;
    }

    pub fn new(config: &Config) -> Self {
        return Self {
            hovered: 0,
            selected: false,
            visiting: None,
            level_url: config.default_url.to_string(),
            visit_status: None,
        };
    }

    pub fn update(&mut self, params: &mut MenuVisitStateUpdateParams<'_>) {
        if !matches!(params.status, AppStatus::MenuVisit) {
            return;
        }

        if let Some(visiting_url) = self.visiting.as_ref() {
            match params.cache.get(visiting_url, params.tick) {
                LevelCacheResult::Loading => {
                    if !matches!(self.visit_status, Some(MenuVisitLoadStatus::Loading)) {
                        self.visit_status = Some(MenuVisitLoadStatus::Loading);
                    }
                }
                LevelCacheResult::Ready(level) => {
                    if !matches!(self.visit_status, Some(MenuVisitLoadStatus::Ready)) {
                        self.visit_status = Some(MenuVisitLoadStatus::Ready);
                        params.log.push(LogData::Entered {
                            name: level.meta().name.to_string(),
                            portal_name: None,
                        });
                        params.player.spawn(PlayerSpawnParams {
                            level_url: Some(visiting_url.clone()),
                            cache: params.cache,
                            cross_fader: params.cross_fader,
                            current_tick: params.tick,
                        });
                        *params.status_next = AppStatus::Simulation;
                    }
                }
                LevelCacheResult::Failed(error) => {
                    if !matches!(self.visit_status, Some(MenuVisitLoadStatus::Failed { .. })) {
                        self.visit_status = Some(MenuVisitLoadStatus::Failed {
                            message: error.to_string(),
                        });
                    }
                }
            }
        }
    }

    pub fn render(&self, params: &mut MenuVisitStateRenderParams<'_>) {
        if !matches!(params.status, AppStatus::MenuVisit) {
            return;
        }

        let items: Vec<MenuVisitItem> = MenuVisitItem::iter().collect();

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

        let status_message = match self.visit_status.as_ref() {
            Some(MenuVisitLoadStatus::Loading) => Some(LOADING_MESSAGE),
            Some(MenuVisitLoadStatus::Failed { message }) => Some(message.as_str()),
            Some(MenuVisitLoadStatus::Ready) | None => None,
        };
        if let Some(message) = status_message {
            let status_y = SCREEN_PADDING + BOX_HEIGHT + SCREEN_PADDING;
            let status_height = TEXT_SIZE.y + INSET * 2.0;
            SpriteBorder::new(
                Vec2::new(SCREEN_PADDING, status_y),
                Vec2::new(BOX_WIDTH, status_height),
            )
            .render(params.buffer);

            let text_pos = Vec2::new(SCREEN_PADDING + INSET, status_y + INSET);
            SpriteLabel::new(
                text_pos,
                Some(STATUS_MAX_CHARS),
                WHITE,
                false,
                SpriteLabelAlignment::Left,
                message,
            )
            .render(params.buffer);
        }
    }

    pub fn key_event(&mut self, params: &mut MenuVisitStateKeyEventHandleParams<'_>) {
        if !matches!(params.status, AppStatus::MenuVisit) {
            return;
        }
        if !matches!(params.event.state, ElementState::Pressed) {
            return;
        }

        let items: Vec<MenuVisitItem> = MenuVisitItem::iter().collect();
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
                params.select_track.reset();
                params.select_track.play();
                self.item_on_select(items[self.hovered], params);
            }
            _ => {}
        }
    }

    pub fn status_change_handle(&mut self, status: &AppStatus) {
        if matches!(status, AppStatus::MenuVisit) {
            self.clear();
        }
    }
}
