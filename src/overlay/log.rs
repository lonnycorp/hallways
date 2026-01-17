use std::collections::VecDeque;

use glam::Vec2;

use crate::app::AppStatus;
use crate::color::Color;
use crate::graphics::pipeline::sprite::SpriteModelVertex;
use crate::graphics::sprite::{
    SpriteLabel, SpriteLabelAlignment, SpriteSolid, SpriteSolidNewParams, TEXT_SIZE,
};

const BUFFER_CAPACITY: usize = 5;

const MAX_LEVEL_NAME_LEN: usize = 32;
const MAX_LEVEL_URL_LEN: usize = 28;
const MAX_ERROR_LEN: usize = 24;
const MAX_PORTAL_NAME_LEN: usize = 16;

const SCREEN_PADDING: f32 = 6.0;
const BG_PADDING_X: f32 = 2.0;
const BG_PADDING_Y: f32 = 1.0;

const WHITE: Color = Color::WHITE;
const BLACK: Color = Color::BLACK;
const GREEN: Color = Color::new(0, 255, 0, 255);
const RED: Color = Color::new(255, 0, 0, 255);
const BLUE: Color = Color::new(0, 128, 255, 255);

pub enum LogData {
    LoadSucceeded {
        name: String,
    },
    LoadFailed {
        url: String,
        error: String,
    },
    Entered {
        name: String,
        portal_name: Option<String>,
    },
}

pub struct Log {
    entries: VecDeque<LogData>,
}

pub struct LogRenderParams<'a> {
    pub buffer: &'a mut Vec<SpriteModelVertex>,
    pub resolution: Vec2,
    pub status: AppStatus,
}

pub struct LogDataRenderParams<'a> {
    pub buffer: &'a mut Vec<SpriteModelVertex>,
    pub y: f32,
    pub left_x: f32,
}

impl Log {
    pub fn new() -> Self {
        return Self {
            entries: VecDeque::new(),
        };
    }

    pub fn push(&mut self, data: LogData) {
        if self.entries.len() >= BUFFER_CAPACITY {
            self.entries.pop_front();
        }
        self.entries.push_back(data);
    }

    pub fn render(&self, params: LogRenderParams<'_>) {
        if !matches!(params.status, AppStatus::Simulation) {
            return;
        }
        if self.entries.is_empty() {
            return;
        }

        let line_count = self.entries.len().min(BUFFER_CAPACITY);
        let left_x = SCREEN_PADDING + BG_PADDING_X;
        let y_start = params.resolution.y - SCREEN_PADDING - line_count as f32 * TEXT_SIZE.y;

        for (i, entry) in self.entries.iter().take(BUFFER_CAPACITY).enumerate() {
            let y = y_start + i as f32 * TEXT_SIZE.y;
            entry.render(LogDataRenderParams {
                buffer: params.buffer,
                y,
                left_x,
            });
        }
    }
}

impl LogData {
    pub fn render(&self, params: LogDataRenderParams<'_>) {
        let buffer = params.buffer;
        let y = params.y;
        let left_x = params.left_x;
        match self {
            LogData::LoadSucceeded { name } => {
                let level_label = SpriteLabel::new(
                    Vec2::new(left_x, y),
                    None,
                    WHITE,
                    false,
                    SpriteLabelAlignment::Left,
                    "level: ",
                );
                let name_label = SpriteLabel::new(
                    Vec2::new(level_label.position().x + level_label.size().x, y),
                    Some(MAX_LEVEL_NAME_LEN),
                    WHITE,
                    false,
                    SpriteLabelAlignment::Left,
                    name,
                );
                let space_label = SpriteLabel::new(
                    Vec2::new(name_label.position().x + name_label.size().x, y),
                    None,
                    WHITE,
                    false,
                    SpriteLabelAlignment::Left,
                    " ",
                );
                let status_label = SpriteLabel::new(
                    Vec2::new(space_label.position().x + space_label.size().x, y),
                    None,
                    GREEN,
                    false,
                    SpriteLabelAlignment::Left,
                    "[load succeeded]",
                );

                let line_end_x = status_label.position().x + status_label.size().x;
                SpriteSolid::new(SpriteSolidNewParams {
                    position: Vec2::new(left_x - BG_PADDING_X, y - BG_PADDING_Y),
                    size: Vec2::new(
                        line_end_x - left_x + BG_PADDING_X * 2.0,
                        TEXT_SIZE.y + BG_PADDING_Y * 2.0,
                    ),
                    color: BLACK,
                })
                .render(buffer);

                level_label.render(buffer);
                name_label.render(buffer);
                space_label.render(buffer);
                status_label.render(buffer);
            }
            LogData::LoadFailed { url, error } => {
                let level_label = SpriteLabel::new(
                    Vec2::new(left_x, y),
                    None,
                    WHITE,
                    false,
                    SpriteLabelAlignment::Left,
                    "level: ",
                );
                let url_label = SpriteLabel::new(
                    Vec2::new(level_label.position().x + level_label.size().x, y),
                    Some(MAX_LEVEL_URL_LEN),
                    WHITE,
                    false,
                    SpriteLabelAlignment::Left,
                    url,
                );
                let space_label = SpriteLabel::new(
                    Vec2::new(url_label.position().x + url_label.size().x, y),
                    None,
                    WHITE,
                    false,
                    SpriteLabelAlignment::Left,
                    " ",
                );
                let status_label = SpriteLabel::new(
                    Vec2::new(space_label.position().x + space_label.size().x, y),
                    None,
                    RED,
                    false,
                    SpriteLabelAlignment::Left,
                    "[load failed]",
                );
                let open_label = SpriteLabel::new(
                    Vec2::new(status_label.position().x + status_label.size().x, y),
                    None,
                    WHITE,
                    false,
                    SpriteLabelAlignment::Left,
                    " (",
                );
                let error_label = SpriteLabel::new(
                    Vec2::new(open_label.position().x + open_label.size().x, y),
                    Some(MAX_ERROR_LEN),
                    WHITE,
                    false,
                    SpriteLabelAlignment::Left,
                    error,
                );
                let close_label = SpriteLabel::new(
                    Vec2::new(error_label.position().x + error_label.size().x, y),
                    None,
                    WHITE,
                    false,
                    SpriteLabelAlignment::Left,
                    ")",
                );

                let line_end_x = close_label.position().x + close_label.size().x;
                SpriteSolid::new(SpriteSolidNewParams {
                    position: Vec2::new(left_x - BG_PADDING_X, y - BG_PADDING_Y),
                    size: Vec2::new(
                        line_end_x - left_x + BG_PADDING_X * 2.0,
                        TEXT_SIZE.y + BG_PADDING_Y * 2.0,
                    ),
                    color: BLACK,
                })
                .render(buffer);

                level_label.render(buffer);
                url_label.render(buffer);
                space_label.render(buffer);
                status_label.render(buffer);
                open_label.render(buffer);
                error_label.render(buffer);
                close_label.render(buffer);
            }
            LogData::Entered { name, portal_name } => {
                let level_label = SpriteLabel::new(
                    Vec2::new(left_x, y),
                    None,
                    WHITE,
                    false,
                    SpriteLabelAlignment::Left,
                    "level: ",
                );
                let name_label = SpriteLabel::new(
                    Vec2::new(level_label.position().x + level_label.size().x, y),
                    Some(MAX_LEVEL_NAME_LEN),
                    WHITE,
                    false,
                    SpriteLabelAlignment::Left,
                    name,
                );
                let space_label = SpriteLabel::new(
                    Vec2::new(name_label.position().x + name_label.size().x, y),
                    None,
                    WHITE,
                    false,
                    SpriteLabelAlignment::Left,
                    " ",
                );
                let status_label = SpriteLabel::new(
                    Vec2::new(space_label.position().x + space_label.size().x, y),
                    None,
                    BLUE,
                    false,
                    SpriteLabelAlignment::Left,
                    "[entered]",
                );

                if let Some(portal_name) = portal_name {
                    let open_label = SpriteLabel::new(
                        Vec2::new(status_label.position().x + status_label.size().x, y),
                        None,
                        WHITE,
                        false,
                        SpriteLabelAlignment::Left,
                        " (",
                    );
                    let portal_label = SpriteLabel::new(
                        Vec2::new(open_label.position().x + open_label.size().x, y),
                        Some(MAX_PORTAL_NAME_LEN),
                        WHITE,
                        false,
                        SpriteLabelAlignment::Left,
                        portal_name,
                    );
                    let close_label = SpriteLabel::new(
                        Vec2::new(portal_label.position().x + portal_label.size().x, y),
                        None,
                        WHITE,
                        false,
                        SpriteLabelAlignment::Left,
                        ")",
                    );

                    let line_end_x = close_label.position().x + close_label.size().x;
                    SpriteSolid::new(SpriteSolidNewParams {
                        position: Vec2::new(left_x - BG_PADDING_X, y - BG_PADDING_Y),
                        size: Vec2::new(
                            line_end_x - left_x + BG_PADDING_X * 2.0,
                            TEXT_SIZE.y + BG_PADDING_Y * 2.0,
                        ),
                        color: BLACK,
                    })
                    .render(buffer);

                    level_label.render(buffer);
                    name_label.render(buffer);
                    space_label.render(buffer);
                    status_label.render(buffer);
                    open_label.render(buffer);
                    portal_label.render(buffer);
                    close_label.render(buffer);
                    return;
                }

                let line_end_x = status_label.position().x + status_label.size().x;
                SpriteSolid::new(SpriteSolidNewParams {
                    position: Vec2::new(left_x - BG_PADDING_X, y - BG_PADDING_Y),
                    size: Vec2::new(
                        line_end_x - left_x + BG_PADDING_X * 2.0,
                        TEXT_SIZE.y + BG_PADDING_Y * 2.0,
                    ),
                    color: BLACK,
                })
                .render(buffer);

                level_label.render(buffer);
                name_label.render(buffer);
                space_label.render(buffer);
                status_label.render(buffer);
            }
        }
    }
}
