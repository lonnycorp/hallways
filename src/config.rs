mod control;
pub use control::ConfigControl;

use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use strum::{EnumCount, IntoEnumIterator};
use url::Url;
use winit::keyboard::PhysicalKey;

const CONFIG_PATH: &str = "hallways/config.json";
const DEFAULT_URL: &str = "https://tlonny.github.io/hallways-nostalgia/hangar.json";

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfigVsyncStatus {
    Enabled,
    Disabled,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub volume: f32,
    pub mouse_sensitivity: f32,
    pub default_url: Url,
    pub vsync_status: ConfigVsyncStatus,
    controls: [PhysicalKey; ConfigControl::COUNT],
}

fn config_path() -> PathBuf {
    let dir = dirs::config_local_dir().unwrap();
    return dir.join(CONFIG_PATH);
}

impl Config {
    pub fn load() -> Self {
        let path = config_path();
        let mut config = fs::read_to_string(&path)
            .ok()
            .and_then(|data| serde_json::from_str::<Self>(&data).ok())
            .unwrap_or_else(Self::new);
        config.volume = config.volume.clamp(0.0, 1.0);
        return config;
    }

    pub fn save(&self) {
        let path = config_path();
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, serde_json::to_string(self).unwrap()).unwrap();
    }

    pub fn new() -> Self {
        let controls: [PhysicalKey; ConfigControl::COUNT] = ConfigControl::iter()
            .map(|c| c.key_default())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        return Self {
            volume: 1.0,
            mouse_sensitivity: 1.0,
            default_url: Url::parse(DEFAULT_URL).unwrap(),
            vsync_status: ConfigVsyncStatus::Enabled,
            controls,
        };
    }

    pub fn key_get(&self, control: ConfigControl) -> &PhysicalKey {
        return &self.controls[control as usize];
    }

    pub fn key_set(&mut self, control: ConfigControl, key: PhysicalKey) {
        self.controls[control as usize] = key;
    }
}

impl ConfigVsyncStatus {
    pub fn present_mode(self) -> wgpu::PresentMode {
        return match self {
            ConfigVsyncStatus::Enabled => wgpu::PresentMode::Fifo,
            ConfigVsyncStatus::Disabled => wgpu::PresentMode::AutoNoVsync,
        };
    }
}
