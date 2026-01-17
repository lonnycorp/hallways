use crate::player::PlayerUpdateParams;

use super::App;

impl App {
    pub(super) fn update(&mut self) {
        let context = self.gpu_state.as_mut().unwrap();

        self.master_sink.set_volume(self.config.volume);
        context.cache.update(&mut self.log, self.tick);
        self.cross_fader.update();

        let status = self.status;

        self.intro
            .update(&self.status, &mut self.status_next, &self.jingle_track);
        self.menu.update(status);
        self.menu_settings.update(status);
        self.menu_visit
            .update(&mut crate::overlay::MenuVisitStateUpdateParams {
                status: &self.status,
                status_next: &mut self.status_next,
                player: &mut self.player,
                cross_fader: &mut self.cross_fader,
                log: &mut self.log,
                cache: &mut context.cache,
                tick: self.tick,
            });

        self.player.update(PlayerUpdateParams {
            status,
            cache: &mut context.cache,
            cross_fader: &mut self.cross_fader,
            log: &mut self.log,
            current_tick: self.tick,
        });

        self.tick += 1;
    }
}
