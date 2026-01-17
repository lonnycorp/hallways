fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os != "windows" {
        return;
    }

    println!("cargo:rerun-if-env-changed=ICO_PATH");

    let ico_path = match std::env::var("ICO_PATH") {
        Ok(path) => path,
        Err(_) => return,
    };

    println!("cargo:rerun-if-changed={ico_path}");
    let mut resource = winresource::WindowsResource::new();
    resource.set_icon(&ico_path);
    resource.compile().unwrap();
    return;
}
