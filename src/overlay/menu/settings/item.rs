use glam::Vec2;
use url::Url;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::app::AppStatus;
use crate::audio::Track;
use crate::color::Color;
use crate::config::{ConfigControl, ConfigVsyncStatus};
use crate::graphics::pipeline::sprite::SpriteModelVertex;
use crate::graphics::sprite::{
    OptionState, SpriteLabel, SpriteLabelAlignment, SpriteText, SpriteTextInput,
    SpriteTextInputNewParams, SpriteTextNewParams, SpriteTextOption, TEXT_SIZE,
};

use super::MenuSettings;
use super::MenuSettingsStateKeyEventHandleParams;

pub const MAX_ITEM_NAME_LEN: usize = 14;
pub const MAX_ITEM_VALUE_LEN: usize = 48;

const ADJUST_STEP: f32 = 0.1;
const ITEM_INDENT: f32 = TEXT_SIZE.x + 2.0;

#[derive(strum::EnumIter, strum::EnumCount, Clone, Copy)]
pub enum MenuSettingsItem {
    Volume,
    MouseSensitivity,
    Forward,
    Back,
    StrafeLeft,
    StrafeRight,
    Jump,
    Crouch,
    DefaultUrl,
    Vsync,
    Save,
    GoBack,
}

fn slider_adjust(value: &mut f32, event: &KeyEvent, move_track: &Track) {
    let mut next = *value;

    if event.physical_key == PhysicalKey::Code(KeyCode::ArrowLeft) {
        next = (next - ADJUST_STEP).clamp(0.0, 1.0);
        move_track.reset();
        move_track.play();
    }
    if event.physical_key == PhysicalKey::Code(KeyCode::ArrowRight) {
        next = (next + ADJUST_STEP).clamp(0.0, 1.0);
        move_track.reset();
        move_track.play();
    }

    *value = next;
}

fn pct_render(buffer: &mut Vec<SpriteModelVertex>, position: Vec2, value: f32, color: Color) {
    let pct = format!("{}%", (value * 100.0).round() as u32);
    let width = pct.len() as f32 * TEXT_SIZE.x;
    let x = position.x + MAX_ITEM_VALUE_LEN as f32 * TEXT_SIZE.x - width;
    for (j, byte) in pct.into_bytes().into_iter().enumerate() {
        let pos = Vec2::new(x + j as f32 * TEXT_SIZE.x, position.y);
        SpriteText::new(SpriteTextNewParams {
            c: char::from(byte),
            bold: false,
            position: pos,
            color,
        })
        .render(buffer);
    }
}

impl MenuSettingsItem {
    pub fn name(&self) -> &'static str {
        return match self {
            MenuSettingsItem::Volume => "VOLUME",
            MenuSettingsItem::MouseSensitivity => "MOUSE SENS",
            MenuSettingsItem::Forward => "FORWARDS",
            MenuSettingsItem::Back => "BACKWARDS",
            MenuSettingsItem::StrafeLeft => "STRAFE LEFT",
            MenuSettingsItem::StrafeRight => "STRAFE RIGHT",
            MenuSettingsItem::Jump => "JUMP",
            MenuSettingsItem::Crouch => "CROUCH",
            MenuSettingsItem::DefaultUrl => "DEFAULT URL",
            MenuSettingsItem::Vsync => "VSYNC",
            MenuSettingsItem::Save => "SAVE",
            MenuSettingsItem::GoBack => "BACK",
        };
    }
}

impl MenuSettings {
    fn item_control(item: MenuSettingsItem) -> Option<ConfigControl> {
        return match item {
            MenuSettingsItem::Forward => Some(ConfigControl::Forward),
            MenuSettingsItem::Back => Some(ConfigControl::Back),
            MenuSettingsItem::StrafeLeft => Some(ConfigControl::StrafeLeft),
            MenuSettingsItem::StrafeRight => Some(ConfigControl::StrafeRight),
            MenuSettingsItem::Jump => Some(ConfigControl::Jump),
            MenuSettingsItem::Crouch => Some(ConfigControl::Crouch),
            _ => None,
        };
    }

    pub(super) fn item_on_select(
        &mut self,
        item: MenuSettingsItem,
        params: &mut MenuSettingsStateKeyEventHandleParams<'_>,
    ) {
        params.select_track.reset();
        params.select_track.play();

        match item {
            MenuSettingsItem::Save => {
                if Url::parse(&self.default_url).is_ok() {
                    self.buffered_config.default_url = Url::parse(&self.default_url).unwrap();
                    *params.config = self.buffered_config.clone();
                    params.surface_config.present_mode = params.config.vsync_status.present_mode();
                    params
                        .surface
                        .configure(params.device, params.surface_config);
                    params.config.save();
                    *params.status_next = AppStatus::MenuHome;
                }
            }
            MenuSettingsItem::GoBack => {
                *params.status_next = AppStatus::MenuHome;
            }
            _ => {
                self.selected = true;
            }
        }
    }

    pub(super) fn item_key_event(
        &mut self,
        item: MenuSettingsItem,
        params: &mut MenuSettingsStateKeyEventHandleParams<'_>,
    ) {
        if !matches!(params.event.state, ElementState::Pressed) {
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
            MenuSettingsItem::Volume => {
                slider_adjust(
                    &mut self.buffered_config.volume,
                    params.event,
                    params.move_track,
                );
            }
            MenuSettingsItem::MouseSensitivity => {
                slider_adjust(
                    &mut self.buffered_config.mouse_sensitivity,
                    params.event,
                    params.move_track,
                );
            }
            MenuSettingsItem::Forward
            | MenuSettingsItem::Back
            | MenuSettingsItem::StrafeLeft
            | MenuSettingsItem::StrafeRight
            | MenuSettingsItem::Jump
            | MenuSettingsItem::Crouch => {
                let control = Self::item_control(item).unwrap();
                self.buffered_config
                    .key_set(control, params.event.physical_key);
                self.selected = false;
                params.move_track.reset();
                params.move_track.play();
            }
            MenuSettingsItem::DefaultUrl => {
                if params.event.physical_key == PhysicalKey::Code(KeyCode::Backspace) {
                    self.default_url.pop();
                }
                if let Some(text) = params.event.text.as_deref() {
                    for c in text.chars().filter(|c| !c.is_control()) {
                        self.default_url.push(c);
                    }
                }
            }
            MenuSettingsItem::Vsync => {
                if params.event.physical_key == PhysicalKey::Code(KeyCode::ArrowLeft)
                    || params.event.physical_key == PhysicalKey::Code(KeyCode::ArrowRight)
                {
                    self.buffered_config.vsync_status = match self.buffered_config.vsync_status {
                        ConfigVsyncStatus::Enabled => ConfigVsyncStatus::Disabled,
                        ConfigVsyncStatus::Disabled => ConfigVsyncStatus::Enabled,
                    };
                    params.move_track.reset();
                    params.move_track.play();
                }
            }
            MenuSettingsItem::Save | MenuSettingsItem::GoBack => {}
        }
    }

    pub(super) fn item_render(
        &mut self,
        buffer: &mut Vec<SpriteModelVertex>,
        item: MenuSettingsItem,
        position: Vec2,
        hovered: bool,
        active: bool,
        tick: u64,
    ) {
        let option_state =
            if matches!(item, MenuSettingsItem::Save) && Url::parse(&self.default_url).is_err() {
                OptionState::Disabled
            } else if active {
                OptionState::Selected
            } else {
                OptionState::Unselected
            };

        let option = SpriteTextOption::new(
            position,
            MAX_ITEM_NAME_LEN,
            hovered,
            option_state,
            item.name(),
        );
        option.render(buffer);

        let value_x = position.x + ITEM_INDENT + MAX_ITEM_NAME_LEN as f32 * TEXT_SIZE.x;
        let value_y = position.y;

        if let Some(value) = match item {
            MenuSettingsItem::Volume => Some(self.buffered_config.volume),
            MenuSettingsItem::MouseSensitivity => Some(self.buffered_config.mouse_sensitivity),
            _ => None,
        } {
            pct_render(buffer, Vec2::new(value_x, value_y), value, Color::WHITE);
        }

        if let Some(name) = match item {
            MenuSettingsItem::Vsync => Some(match self.buffered_config.vsync_status {
                ConfigVsyncStatus::Enabled => "ENABLED",
                ConfigVsyncStatus::Disabled => "DISABLED",
            }),
            MenuSettingsItem::Forward
            | MenuSettingsItem::Back
            | MenuSettingsItem::StrafeLeft
            | MenuSettingsItem::StrafeRight
            | MenuSettingsItem::Jump
            | MenuSettingsItem::Crouch => {
                let control = Self::item_control(item).unwrap();
                Some(self.key_cache.name(self.buffered_config.key_get(control)))
            }
            _ => None,
        } {
            let position = Vec2::new(value_x, value_y);
            SpriteLabel::new(
                position,
                Some(MAX_ITEM_VALUE_LEN),
                Color::WHITE,
                false,
                SpriteLabelAlignment::Right,
                name,
            )
            .render(buffer);
        }

        if let Some(input) = matches!(item, MenuSettingsItem::DefaultUrl).then_some(
            SpriteTextInput::new(SpriteTextInputNewParams {
                position: Vec2::new(value_x, value_y),
                max_len: MAX_ITEM_VALUE_LEN,
                text: &self.default_url,
                active,
                clock: tick,
            }),
        ) {
            input.render(buffer);
        }
    }
}
