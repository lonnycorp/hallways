use std::process::Command;

pub const LINUX_TARGET: &str = "x86_64-unknown-linux-gnu";

pub fn linux_build() {
    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("--release")
        .arg("--target")
        .arg(LINUX_TARGET);

    let status = cmd.status().unwrap();
    if !status.success() {
        panic!("cargo build failed");
    }
}
