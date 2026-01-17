use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use rodio::Source;

pub struct Silent {
    alive: Arc<AtomicBool>,
    channels: u16,
    sample_rate: u32,
}

pub struct SilentSource {
    alive: Arc<AtomicBool>,
    channels: u16,
    sample_rate: u32,
}

impl Silent {
    pub fn new(channels: u16, sample_rate: u32) -> Self {
        return Self {
            alive: Arc::new(AtomicBool::new(true)),
            channels,
            sample_rate,
        };
    }

    pub fn source(&self) -> SilentSource {
        return SilentSource {
            alive: Arc::clone(&self.alive),
            channels: self.channels,
            sample_rate: self.sample_rate,
        };
    }
}

impl Drop for Silent {
    fn drop(&mut self) {
        self.alive.store(false, Ordering::Release);
    }
}

impl Iterator for SilentSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.alive.load(Ordering::Acquire) {
            return Some(0.0);
        }

        return None;
    }
}

impl Source for SilentSource {
    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn channels(&self) -> u16 {
        return self.channels;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}
