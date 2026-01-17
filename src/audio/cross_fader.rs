use rodio::dynamic_mixer::{DynamicMixer, DynamicMixerController};

use super::silent::Silent;
use super::{Track, TrackData};

const RAMP_SPEED: f32 = 0.01;
const MIXER_CHANNELS: u16 = 2;
const MIXER_SAMPLE_RATE: u32 = 48000;

pub struct CrossFader {
    mixer: std::sync::Arc<DynamicMixerController<f32>>,
    source: Option<DynamicMixer<f32>>,
    _silent: Silent,
    tracks: [Option<Track>; 2],
    current: Option<usize>,
    volumes: [f32; 2],
}

impl CrossFader {
    pub fn new() -> Self {
        let (local_mixer, local_source) =
            rodio::dynamic_mixer::mixer::<f32>(MIXER_CHANNELS, MIXER_SAMPLE_RATE);
        let silent = Silent::new(MIXER_CHANNELS, MIXER_SAMPLE_RATE);
        local_mixer.add(silent.source());

        return Self {
            mixer: local_mixer,
            source: Some(local_source),
            _silent: silent,
            tracks: [None, None],
            current: None,
            volumes: [0.0, 0.0],
        };
    }

    pub fn source(&mut self) -> DynamicMixer<f32> {
        return self.source.take().unwrap();
    }

    pub fn track_fade_in(&mut self, track_data: TrackData) {
        let next = match self.current {
            Some(current) => (current + 1) % 2,
            None => 0,
        };

        let track = Track::new(track_data);
        track.volume_set(0.0);
        track.play();
        self.mixer.add(track.source());

        self.tracks[next] = Some(track);
        self.volumes[next] = 0.0;
        self.current = Some(next);
    }

    pub fn track_fade_out(&mut self) {
        self.current = None;
    }

    pub fn update(&mut self) {
        for i in 0..2 {
            let target = if self.current == Some(i) { 1.0 } else { 0.0 };
            let delta = (target - self.volumes[i]).clamp(-RAMP_SPEED, RAMP_SPEED);
            self.volumes[i] += delta;

            if let Some(track) = self.tracks[i].as_ref() {
                track.volume_set(self.volumes[i]);
            }

            if self.current != Some(i) && self.volumes[i] <= 0.0 {
                self.tracks[i] = None;
            }
        }
    }
}
