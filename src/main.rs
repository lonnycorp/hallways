// Avoid spawning a console window on Windows builds.
#![windows_subsystem = "windows"]

mod app;
mod audio;
mod color;
mod config;
mod gltf;
mod graphics;
mod level;
mod overlay;
mod parry3d;
mod player;
mod window;

use app::App;
use include_dir::include_dir;
use window::Window;

pub static ASSET: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/asset");

const WINDOW_TITLE: &str = "Hallways";

fn main() {
    Window::new(WINDOW_TITLE, App::new()).run();
}
