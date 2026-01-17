use glam::Vec2;
use strum::{EnumCount, EnumIter};

use crate::app::AppStatus;
use crate::graphics::pipeline::sprite::SpriteModelVertex;
use crate::graphics::sprite::{OptionState, SpriteTextOption};

use super::MenuHome;
use super::MenuHomeStateKeyboardEventHandleParams;

pub const MAX_ITEM_LEN: usize = 12;

#[derive(EnumCount, EnumIter)]
pub enum MenuHomeItem {
    Visit,
    Settings,
    Quit,
}

impl MenuHomeItem {
    pub fn name(&self) -> &'static str {
        return match self {
            MenuHomeItem::Visit => "VISIT",
            MenuHomeItem::Settings => "SETTINGS",
            MenuHomeItem::Quit => "QUIT",
        };
    }
}

impl MenuHome {
    pub(super) fn item_on_select(
        &self,
        item: MenuHomeItem,
        params: &mut MenuHomeStateKeyboardEventHandleParams<'_>,
    ) {
        params.select_track.reset();
        params.select_track.play();

        match item {
            MenuHomeItem::Visit => *params.status_next = AppStatus::MenuVisit,
            MenuHomeItem::Settings => *params.status_next = AppStatus::MenuSettings,
            MenuHomeItem::Quit => params.event_loop.exit(),
        }
    }

    pub(super) fn item_render(
        &self,
        buffer: &mut Vec<SpriteModelVertex>,
        item: MenuHomeItem,
        position: Vec2,
        hovered: bool,
    ) {
        SpriteTextOption::new(
            position,
            MAX_ITEM_LEN,
            hovered,
            OptionState::Unselected,
            item.name(),
        )
        .render(buffer);
    }
}
