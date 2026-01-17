use super::Color;

const EPSILON: f64 = 0.0000001;

fn close(lhs: f64, rhs: f64) -> bool {
    return (lhs - rhs).abs() <= EPSILON;
}

#[test]
fn white_maps_to_wgpu_white() {
    let white: wgpu::Color = Color::WHITE.into();
    assert_eq!(white, wgpu::Color::WHITE);
}

#[test]
fn black_maps_to_wgpu_black() {
    let black: wgpu::Color = Color::BLACK.into();
    assert_eq!(black, wgpu::Color::BLACK);
}

#[test]
fn empty_maps_to_wgpu_transparent() {
    let empty: wgpu::Color = Color::EMPTY.into();
    assert_eq!(empty, wgpu::Color::TRANSPARENT);
}

#[test]
fn roundtrip_color_preserves_u8_components() {
    let color = Color::new(13, 101, 197, 245);

    let wgpu: wgpu::Color = color.into();
    let roundtrip: Color = wgpu.into();

    assert!(close(wgpu.r, 13.0 / 255.0));
    assert!(close(wgpu.g, 101.0 / 255.0));
    assert!(close(wgpu.b, 197.0 / 255.0));
    assert!(close(wgpu.a, 245.0 / 255.0));
    assert_eq!(roundtrip, color);
}
