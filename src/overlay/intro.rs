use std::f32::consts::PI;

use glam::Vec2;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::app::AppStatus;
use crate::audio::Track;
use crate::color::Color;
use crate::graphics::pipeline::sprite::SpriteModelVertex;
use crate::graphics::sprite::{
    SpriteLogo, SpriteSolid, SpriteSolidNewParams, SpriteText, SpriteTextNewParams, TEXT_SIZE,
};

const TIME_STEP: f32 = 0.002;

const INTRO_START: f32 = 0.15;

const TITLE: &[u8] = b"LONNYCORP";
const TITLE_CHAR_BOUNCE_DURATION: f32 = 0.05;
const TITLE_CHAR_STAGGER_DELAY: f32 = 0.01;
const TITLE_SPACING: f32 = 2.0;
const LOGO_UV_SIZE: f32 = 480.0;
const TITLE_BOUNCE_HEIGHT: f32 = 32.0;

fn ramp(t: f32, start: f32, end: f32) -> f32 {
    return ((t - start) / (end - start)).clamp(0.0, 1.0);
}

pub struct IntroRenderParams<'a> {
    pub buffer: &'a mut Vec<SpriteModelVertex>,
    pub resolution: Vec2,
    pub status: AppStatus,
}

pub struct Intro {
    time: f32,
    jingle_played: bool,
}

impl Intro {
    pub fn new() -> Self {
        return Self {
            time: 0.0,
            jingle_played: false,
        };
    }

    pub fn update(
        &mut self,
        status: &AppStatus,
        status_next: &mut AppStatus,
        jingle_track: &Track,
    ) {
        if !matches!(status, AppStatus::Intro) {
            return;
        }

        self.time = (self.time + TIME_STEP).min(1.0);
        if self.time >= 1.0 {
            *status_next = AppStatus::MenuHome;
        }

        if !self.jingle_played && self.time >= INTRO_START {
            jingle_track.reset();
            jingle_track.play();
            self.jingle_played = true;
        }
    }

    pub fn render(&self, params: &mut IntroRenderParams<'_>) {
        if !matches!(params.status, AppStatus::Intro) {
            return;
        }

        SpriteSolid::new(SpriteSolidNewParams {
            position: Vec2::ZERO,
            size: params.resolution,
            color: Color::WHITE,
        })
        .render(params.buffer);
        SpriteLogo::new(params.resolution / 2.0).render(params.buffer);

        let title_width = ((TITLE.len() - 1) as f32 * TITLE_SPACING + 1.0) * TEXT_SIZE.x;
        let title_x = (params.resolution.x - title_width) / 2.0;
        let title_y = params.resolution.y / 2.0 + LOGO_UV_SIZE / 2.0 + TEXT_SIZE.y;

        for (i, &c) in TITLE.iter().enumerate() {
            let char_start = INTRO_START + i as f32 * TITLE_CHAR_STAGGER_DELAY;
            let char_life = ramp(
                self.time,
                char_start,
                char_start + TITLE_CHAR_BOUNCE_DURATION,
            );
            let x = title_x + i as f32 * TITLE_SPACING * TEXT_SIZE.x;
            let y_offset = (char_life * PI).sin() * TITLE_BOUNCE_HEIGHT;
            SpriteText::new(SpriteTextNewParams {
                c: c as char,
                bold: true,
                position: Vec2::new(x, title_y - y_offset),
                color: Color::BLACK,
            })
            .render(params.buffer);
        }
    }

    pub fn key_event_handle(
        &mut self,
        event: &KeyEvent,
        status: &AppStatus,
        status_next: &mut AppStatus,
    ) {
        if !matches!(status, AppStatus::Intro) {
            return;
        }
        if !matches!(event.state, ElementState::Pressed) {
            return;
        }
        if event.physical_key == PhysicalKey::Code(KeyCode::Escape)
            || event.physical_key == PhysicalKey::Code(KeyCode::Enter)
        {
            *status_next = AppStatus::MenuHome;
        }
    }
}
