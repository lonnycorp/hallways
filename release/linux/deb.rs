use std::fs;
use std::path::Path;
use std::process::Command;

use super::build::LINUX_TARGET;
use crate::artifact::Artifact;
use crate::icon::ICONSET_DIR;
use crate::{version_read, APP_NAME, DESCRIPTION};

pub fn linux_deb_package() {
    let iconset_dir = Path::new(ICONSET_DIR);
    let version = version_read();
    let deb_dir = Path::new("dist/deb");

    fs::create_dir_all(deb_dir.join("DEBIAN")).unwrap();
    fs::create_dir_all(deb_dir.join("usr/bin")).unwrap();
    fs::create_dir_all(deb_dir.join("usr/share/applications")).unwrap();
    fs::create_dir_all(deb_dir.join("usr/share/icons/hicolor/256x256/apps")).unwrap();

    fs::copy(
        Path::new("target")
            .join(LINUX_TARGET)
            .join("release")
            .join("hallways"),
        deb_dir.join("usr/bin/hallways"),
    )
    .unwrap();

    fs::copy(
        iconset_dir.join("icon_256x256.png"),
        deb_dir.join("usr/share/icons/hicolor/256x256/apps/hallways.png"),
    )
    .unwrap();

    let desktop_template = fs::read_to_string("asset/release/hallways.desktop.template").unwrap();
    let desktop_entry = desktop_template
        .replace("{APP_NAME}", APP_NAME)
        .replace("{DESCRIPTION}", DESCRIPTION);
    fs::write(
        deb_dir.join("usr/share/applications/hallways.desktop"),
        &desktop_entry,
    )
    .unwrap();

    let control_template = fs::read_to_string("asset/release/deb.control.template").unwrap();
    let control = control_template
        .replace("{version}", &version)
        .replace("{DESCRIPTION}", DESCRIPTION);
    fs::write(deb_dir.join("DEBIAN/control"), control).unwrap();

    let deb_path = Path::new("dist").join(Artifact::LinuxDeb.file_name(&version));
    let status = Command::new("dpkg-deb")
        .arg("--build")
        .arg(deb_dir)
        .arg(&deb_path)
        .status()
        .unwrap();
    if !status.success() {
        panic!("dpkg-deb failed");
    }
}
