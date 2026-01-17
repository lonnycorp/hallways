use glam::Vec2;

use crate::app::AppStatus;
use crate::color::Color;
use crate::graphics::pipeline::sprite::SpriteModelVertex;
use crate::graphics::sprite::{SpriteBorder, SpriteLabel, SpriteLabelAlignment, TEXT_SIZE};
use crate::level::cache::{LevelCache, LevelCacheResult};
use crate::player::Player;

const BORDER: f32 = 3.0;
const TEXT_PADDING: f32 = 3.0;
const SCREEN_PADDING: f32 = 6.0;
const LINE_COUNT: f32 = 2.0;
const MAX_CHARS: usize = 42;
const LABEL_LEN: usize = 7;
const INSET: f32 = BORDER + TEXT_PADDING;
const BOX_WIDTH: f32 = MAX_CHARS as f32 * TEXT_SIZE.x + INSET * 2.0;
const BOX_HEIGHT: f32 = LINE_COUNT * TEXT_SIZE.y + INSET * 2.0;
const TEXT_COLOR: Color = Color::WHITE;
const LEVEL_LABEL: &str = "LEVEL";
const AUTHOR_LABEL: &str = "AUTHOR";

pub struct BannerRenderParams<'a> {
    pub buffer: &'a mut Vec<SpriteModelVertex>,
    pub resolution: Vec2,
    pub status: AppStatus,
    pub player: &'a Player,
    pub cache: &'a mut LevelCache,
    pub tick: u64,
}

pub fn banner_render(params: BannerRenderParams<'_>) {
    match params.status {
        AppStatus::MenuHome | AppStatus::MenuVisit | AppStatus::MenuSettings => {}
        _ => {
            return;
        }
    }

    let Some(level_url) = params.player.level_url() else {
        return;
    };

    let level = match params.cache.get(level_url, params.tick) {
        LevelCacheResult::Ready(level) => level,
        _ => return,
    };

    let box_pos = Vec2::new(
        params.resolution.x - BOX_WIDTH - SCREEN_PADDING,
        params.resolution.y - BOX_HEIGHT - SCREEN_PADDING,
    );
    SpriteBorder::new(box_pos, Vec2::new(BOX_WIDTH, BOX_HEIGHT)).render(params.buffer);

    let label_x = box_pos.x + INSET;
    let value_x = label_x + LABEL_LEN as f32 * TEXT_SIZE.x;
    let text_y = box_pos.y + INSET;

    let meta = level.meta();
    let author = meta.author.as_deref().unwrap_or("N/A");

    let lines: [(&str, &str, bool); 2] = [
        (LEVEL_LABEL, meta.name.as_str(), true),
        (AUTHOR_LABEL, author, false),
    ];

    for (i, &(label, value, bold)) in lines.iter().enumerate() {
        let y = text_y + i as f32 * TEXT_SIZE.y;
        SpriteLabel::new(
            Vec2::new(label_x, y),
            Some(LABEL_LEN),
            TEXT_COLOR,
            false,
            SpriteLabelAlignment::Left,
            label,
        )
        .render(params.buffer);
        SpriteLabel::new(
            Vec2::new(value_x, y),
            Some(MAX_CHARS - LABEL_LEN),
            TEXT_COLOR,
            bold,
            SpriteLabelAlignment::Left,
            value,
        )
        .render(params.buffer);
    }
}
