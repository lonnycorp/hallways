use std::fs::File;
use std::path::Path;
use std::process::Command;

use ico::{IconDir, IconDirEntry, IconImage, ResourceType};

use crate::artifact::Artifact;
use crate::icon::{self, IconSpec, ICONSET_DIR};
use crate::version_read;

const WINDOWS_TARGET: &str = "x86_64-pc-windows-msvc";
const WINDOWS_ICON_PATH: &str = "dist/hallways.ico";

pub const ICON_SPECS: &[IconSpec] = &[
    IconSpec::Icon16,
    IconSpec::Icon32,
    IconSpec::Icon48,
    IconSpec::Icon32At2x,
    IconSpec::Icon128,
    IconSpec::Icon256,
];

pub fn windows_iconset_render() {
    icon::iconset_render(ICON_SPECS);
    return;
}

pub fn windows_build() {
    let ico_path = Path::new(WINDOWS_ICON_PATH);
    let iconset_path = Path::new(ICONSET_DIR);
    ico_render(iconset_path, ico_path);
    windows_build_with_icon();
    return;
}

fn ico_render(iconset_path: &Path, ico_path: &Path) {
    let mut icon_dir = IconDir::new(ResourceType::Icon);

    for spec in ICON_SPECS {
        let png_path = spec.path(iconset_path);
        let image = image::open(&png_path).unwrap().into_rgba8();
        let icon_image = IconImage::from_rgba_data(spec.width(), spec.height(), image.into_raw());
        let entry = IconDirEntry::encode(&icon_image).unwrap();
        icon_dir.add_entry(entry);
    }

    let mut file = File::create(ico_path).unwrap();
    icon_dir.write(&mut file).unwrap();
    return;
}

fn windows_build_with_icon() {
    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("--release")
        .arg("--target")
        .arg(WINDOWS_TARGET)
        .arg("--bin")
        .arg("hallways")
        .env("ICO_PATH", WINDOWS_ICON_PATH);

    let status = cmd.status().unwrap();
    if !status.success() {
        panic!("cargo build failed");
    }
    return;
}

pub fn windows_package() {
    let version = version_read();
    let zip_path = Path::new("dist").join(Artifact::WindowsZip.file_name(&version));
    let exe_path = Path::new("target")
        .join(WINDOWS_TARGET)
        .join("release")
        .join("hallways.exe");

    let status = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-Command")
        .arg(format!(
            "Compress-Archive -Path \"{}\" -DestinationPath \"{}\"",
            exe_path.display(),
            zip_path.display()
        ))
        .status()
        .unwrap();
    if !status.success() {
        panic!("Compress-Archive failed");
    }
    return;
}
