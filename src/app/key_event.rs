use winit::event::{ElementState, KeyEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::app::AppStatus;
use crate::overlay::{
    MenuHomeStateKeyboardEventHandleParams, MenuSettingsStateKeyEventHandleParams,
    MenuVisitStateKeyEventHandleParams,
};
use crate::player::PlayerKeyEventParams;

use super::App;

impl App {
    pub(super) fn key_event(&mut self, event: &KeyEvent, event_loop: &ActiveEventLoop) {
        let status = self.status;
        if matches!(status, AppStatus::Simulation)
            && matches!(event.state, ElementState::Pressed)
            && event.physical_key == PhysicalKey::Code(KeyCode::Escape)
        {
            self.move_track.reset();
            self.move_track.play();
            self.status_next = AppStatus::MenuHome;
        }

        self.intro
            .key_event_handle(event, &self.status, &mut self.status_next);

        self.player.key_event(PlayerKeyEventParams {
            status,
            event,
            config: &self.config,
            current_tick: self.tick,
        });

        self.menu
            .key_event(&mut MenuHomeStateKeyboardEventHandleParams {
                event,
                event_loop,
                status: &self.status,
                status_next: &mut self.status_next,
                select_track: &self.select_track,
                move_track: &self.move_track,
            });

        self.menu_visit
            .key_event(&mut MenuVisitStateKeyEventHandleParams {
                event,
                status: &self.status,
                status_next: &mut self.status_next,
                select_track: &self.select_track,
                move_track: &self.move_track,
                player: &mut self.player,
                cross_fader: &mut self.cross_fader,
                cache: &mut self.gpu_state.as_mut().unwrap().cache,
                tick: self.tick,
            });

        let gpu_state = self.gpu_state.as_mut().unwrap();
        self.menu_settings
            .key_event(&mut MenuSettingsStateKeyEventHandleParams {
                event,
                status: &self.status,
                status_next: &mut self.status_next,
                config: &mut self.config,
                surface: gpu_state.surface.as_ref(),
                device: gpu_state.device.as_ref(),
                surface_config: &mut gpu_state.surface_config,
                select_track: &self.select_track,
                move_track: &self.move_track,
            });
    }
}
