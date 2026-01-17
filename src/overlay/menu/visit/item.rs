use glam::Vec2;
use url::Url;
use winit::event::ElementState;
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::app::AppStatus;
use crate::graphics::pipeline::sprite::SpriteModelVertex;
use crate::graphics::sprite::{
    OptionState, SpriteTextInput, SpriteTextInputNewParams, SpriteTextOption, TEXT_SIZE,
};
use crate::player::PlayerSpawnParams;

use super::MenuVisit;
use super::MenuVisitStateKeyEventHandleParams;

const ITEM_INDENT: f32 = TEXT_SIZE.x + 2.0;

pub const MAX_ITEM_NAME_LEN: usize = 14;
pub const MAX_ITEM_VALUE_LEN: usize = 48;

#[derive(strum::EnumIter, strum::EnumCount, Clone, Copy)]
pub enum MenuVisitItem {
    LevelUrl,
    Visit,
    GoBack,
}

impl MenuVisitItem {
    pub fn name(&self) -> &'static str {
        return match self {
            MenuVisitItem::LevelUrl => "LEVEL URL",
            MenuVisitItem::Visit => "VISIT",
            MenuVisitItem::GoBack => "BACK",
        };
    }
}

impl MenuVisit {
    pub(super) fn item_on_select(
        &mut self,
        item: MenuVisitItem,
        params: &mut MenuVisitStateKeyEventHandleParams<'_>,
    ) {
        match item {
            MenuVisitItem::LevelUrl => {
                self.selected = true;
            }
            MenuVisitItem::Visit => {
                if let Ok(url) = Url::parse(&self.level_url) {
                    params.cache.clear();
                    params.player.spawn(PlayerSpawnParams {
                        level_url: None,
                        cache: params.cache,
                        cross_fader: params.cross_fader,
                        current_tick: params.tick,
                    });
                    self.visiting = Some(url);
                    self.selected = true;
                }
            }
            MenuVisitItem::GoBack => {
                *params.status_next = AppStatus::MenuHome;
            }
        }
    }

    pub(super) fn item_key_event(
        &mut self,
        item: MenuVisitItem,
        params: &mut MenuVisitStateKeyEventHandleParams<'_>,
    ) {
        if !matches!(params.event.state, ElementState::Pressed) {
            return;
        }

        if let MenuVisitItem::Visit = item {
            if self.visiting.is_some() {
                if params.event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                    self.visiting = None;
                    self.selected = false;
                    self.visit_status = None;
                    params.move_track.reset();
                    params.move_track.play();
                }
                return;
            }
            self.selected = false;
            return;
        }
        if params.event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
            self.selected = false;
            params.move_track.reset();
            params.move_track.play();
            return;
        }
        if params.event.physical_key == PhysicalKey::Code(KeyCode::Enter) {
            self.selected = false;
            params.move_track.reset();
            params.move_track.play();
            return;
        }
        match item {
            MenuVisitItem::LevelUrl => {
                if params.event.physical_key == PhysicalKey::Code(KeyCode::Backspace) {
                    self.level_url.pop();
                }
                if let Some(text) = params.event.text.as_deref() {
                    for c in text.chars().filter(|c| !c.is_control()) {
                        self.level_url.push(c);
                    }
                }
            }
            MenuVisitItem::Visit | MenuVisitItem::GoBack => {}
        }
    }

    pub(super) fn item_render(
        &self,
        buffer: &mut Vec<SpriteModelVertex>,
        item: MenuVisitItem,
        position: Vec2,
        hovered: bool,
        active: bool,
        tick: u64,
    ) {
        let option_state = match item {
            MenuVisitItem::Visit => {
                if Url::parse(&self.level_url).is_err() {
                    OptionState::Disabled
                } else if active {
                    OptionState::Selected
                } else {
                    OptionState::Unselected
                }
            }
            _ => {
                if active {
                    OptionState::Selected
                } else {
                    OptionState::Unselected
                }
            }
        };

        let option = SpriteTextOption::new(
            position,
            MAX_ITEM_NAME_LEN,
            hovered,
            option_state,
            item.name(),
        );
        option.render(buffer);

        if let Some(input) = matches!(item, MenuVisitItem::LevelUrl).then_some(
            SpriteTextInput::new(SpriteTextInputNewParams {
                position: Vec2::new(
                    position.x + ITEM_INDENT + MAX_ITEM_NAME_LEN as f32 * TEXT_SIZE.x,
                    position.y,
                ),
                max_len: MAX_ITEM_VALUE_LEN,
                text: &self.level_url,
                active,
                clock: tick,
            }),
        ) {
            input.render(buffer);
        }
    }
}
