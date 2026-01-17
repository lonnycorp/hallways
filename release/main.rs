mod artifact;
mod icon;
mod linux;
mod win;

use std::env;
use std::fs;
use std::path::Path;

use linux::{linux_appimage_package, linux_build, linux_deb_package, linux_iconset_render};
use win::{windows_build, windows_iconset_render, windows_package};

pub const APP_NAME: &str = "Hallways";
pub const DESCRIPTION: &str = "A web browser for 3D spaces";

enum ReleaseTarget {
    Linux,
    Windows,
}

fn dist_clean() {
    let dist = Path::new("dist");
    if dist.exists() {
        fs::remove_dir_all(dist).unwrap();
    }
    fs::create_dir_all(dist).unwrap();
}

pub fn version_read() -> String {
    let contents = fs::read_to_string("Cargo.toml").unwrap();
    for line in contents.lines() {
        if let Some(version) = line.strip_prefix("version = \"") {
            if let Some(value) = version.strip_suffix('"') {
                return value.to_string();
            }
        }
    }
    panic!("version not found in Cargo.toml");
}

fn release_target_read() -> ReleaseTarget {
    let mut args = env::args();
    let _program = args.next();
    let target = match args.next().as_deref() {
        Some("linux") => ReleaseTarget::Linux,
        Some("windows") => ReleaseTarget::Windows,
        Some(value) => panic!("unknown target: {value}"),
        None => panic!("expected target as first argument: linux|windows"),
    };

    return target;
}

fn main() {
    let target = release_target_read();
    dist_clean();

    match target {
        ReleaseTarget::Linux => {
            linux_iconset_render();
            linux_build();
            linux_appimage_package();
            linux_deb_package();
        }
        ReleaseTarget::Windows => {
            windows_iconset_render();
            windows_build();
            windows_package();
        }
    }
}
