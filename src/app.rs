mod gpu;
mod handler;
mod key_event;
mod mouse_motion;
mod render;
mod resize;
mod update;

use std::sync::Arc;
use std::time::Instant;

use rayon::ThreadPool;
use rayon::ThreadPoolBuilder;
use rodio::{OutputStream, Sink};
use winit::window::CursorGrabMode;

use crate::audio;
use crate::config::Config;
use crate::overlay;
use crate::player::Player;
use crate::ASSET;

use self::gpu::AppGPUState;

pub use handler::SIM_STEP;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AppStatus {
    Intro,
    MenuHome,
    MenuVisit,
    MenuSettings,
    Simulation,
}

pub struct App {
    gpu_state: Option<AppGPUState>,
    asset_thread_pool: Arc<ThreadPool>,
    _audio_stream: OutputStream,
    config: Config,
    status: AppStatus,
    status_next: AppStatus,
    intro: overlay::Intro,
    menu: overlay::MenuHome,
    menu_settings: overlay::MenuSettings,
    menu_visit: overlay::MenuVisit,
    log: overlay::Log,
    tick: u64,
    master_sink: Sink,
    cross_fader: audio::CrossFader,
    jingle_track: audio::Track,
    select_track: audio::Track,
    move_track: audio::Track,
    player: Player,
    last_update: Instant,
}

const JINGLE_AUDIO_PATH: &str = "audio/jingle.wav";
const SELECT_AUDIO_PATH: &str = "audio/select.wav";
const MOVE_AUDIO_PATH: &str = "audio/move.wav";
const ASSET_THREAD_POOL_SIZE: usize = 6;

impl App {
    pub fn new() -> Self {
        let asset_thread_pool = Arc::new(
            ThreadPoolBuilder::new()
                .num_threads(ASSET_THREAD_POOL_SIZE)
                .build()
                .unwrap(),
        );
        let config = Config::load();
        let intro = crate::overlay::Intro::new();
        let menu = crate::overlay::MenuHome::new();
        let menu_settings = crate::overlay::MenuSettings::new(&config);
        let menu_visit = crate::overlay::MenuVisit::new(&config);
        let log = crate::overlay::Log::new();

        let (_audio_stream, audio) = rodio::OutputStream::try_default().unwrap();
        let (mixer_ctrl, mixer_src) = rodio::dynamic_mixer::mixer::<f32>(2, 48000);
        let master_sink = rodio::Sink::try_new(&audio).unwrap();
        master_sink.append(mixer_src);
        let mut cross_fader = audio::CrossFader::new();
        mixer_ctrl.add(cross_fader.source());
        let jingle_track_data =
            audio::TrackData::new(ASSET.get_file(JINGLE_AUDIO_PATH).unwrap().contents(), false)
                .unwrap();
        let jingle_track = audio::Track::new(jingle_track_data);
        mixer_ctrl.add(jingle_track.source());
        let select_track_data =
            audio::TrackData::new(ASSET.get_file(SELECT_AUDIO_PATH).unwrap().contents(), false)
                .unwrap();
        let select_track = audio::Track::new(select_track_data);
        mixer_ctrl.add(select_track.source());
        let move_track_data =
            audio::TrackData::new(ASSET.get_file(MOVE_AUDIO_PATH).unwrap().contents(), false)
                .unwrap();
        let move_track = audio::Track::new(move_track_data);
        mixer_ctrl.add(move_track.source());
        let player = crate::player::Player::new(glam::Vec3::ZERO);

        return Self {
            gpu_state: None,
            asset_thread_pool,
            _audio_stream,
            config,
            status: AppStatus::Intro,
            status_next: AppStatus::Intro,
            intro,
            menu,
            menu_settings,
            menu_visit,
            log,
            tick: 0,
            master_sink,
            cross_fader,
            jingle_track,
            select_track,
            move_track,
            player,
            last_update: Instant::now(),
        };
    }

    pub(super) fn status_swap(&mut self) {
        if self.status == self.status_next {
            return;
        }

        let gpu_state = self.gpu_state.as_ref().unwrap();
        self.status = self.status_next;

        if self.status == AppStatus::Simulation {
            let _ = gpu_state.handle.set_cursor_grab(CursorGrabMode::Confined);
            gpu_state.handle.set_cursor_visible(false);
        } else {
            let _ = gpu_state.handle.set_cursor_grab(CursorGrabMode::None);
            gpu_state.handle.set_cursor_visible(true);
        }

        self.player.status_change(&self.status);
        self.menu_visit.status_change_handle(&self.status);
        self.menu_settings
            .status_change_handle(&self.status, &self.config);
    }
}
