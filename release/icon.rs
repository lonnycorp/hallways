use std::fs;
use std::path::{Path, PathBuf};

use image::imageops::FilterType;

#[derive(Clone, Copy)]
pub struct IconSpecData {
    pub file_name: &'static str,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum IconSpec {
    Icon16,
    Icon16At2x,
    Icon32,
    Icon32At2x,
    Icon48,
    Icon128,
    Icon128At2x,
    Icon256,
}

pub const ICONSET_DIR: &str = "dist/Hallways.iconset";
pub const ICON_SOURCE_PATH: &str = "asset/release/logo.png";

impl IconSpec {
    fn data(&self) -> IconSpecData {
        match self {
            IconSpec::Icon16 => IconSpecData {
                file_name: "icon_16x16.png",
                width: 16,
                height: 16,
            },
            IconSpec::Icon16At2x => IconSpecData {
                file_name: "icon_16x16@2x.png",
                width: 32,
                height: 32,
            },
            IconSpec::Icon32 => IconSpecData {
                file_name: "icon_32x32.png",
                width: 32,
                height: 32,
            },
            IconSpec::Icon32At2x => IconSpecData {
                file_name: "icon_32x32@2x.png",
                width: 64,
                height: 64,
            },
            IconSpec::Icon48 => IconSpecData {
                file_name: "icon_48x48.png",
                width: 48,
                height: 48,
            },
            IconSpec::Icon128 => IconSpecData {
                file_name: "icon_128x128.png",
                width: 128,
                height: 128,
            },
            IconSpec::Icon128At2x => IconSpecData {
                file_name: "icon_128x128@2x.png",
                width: 256,
                height: 256,
            },
            IconSpec::Icon256 => IconSpecData {
                file_name: "icon_256x256.png",
                width: 256,
                height: 256,
            },
        }
    }

    pub fn width(&self) -> u32 {
        return self.data().width;
    }

    pub fn height(&self) -> u32 {
        return self.data().height;
    }

    pub fn path(&self, iconset_dir: &Path) -> PathBuf {
        return iconset_dir.join(self.data().file_name);
    }
}

pub fn iconset_render(specs: &[IconSpec]) {
    let source = image::open(Path::new(ICON_SOURCE_PATH)).unwrap();
    let out_path = Path::new(ICONSET_DIR);
    if out_path.exists() {
        fs::remove_dir_all(out_path).unwrap();
    }
    fs::create_dir_all(out_path).unwrap();

    for spec in specs {
        let output = spec.path(out_path);
        let resized = source.resize_exact(spec.width(), spec.height(), FilterType::Lanczos3);
        resized.save(output).unwrap();
    }
}
