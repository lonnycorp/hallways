#[cfg(test)]
mod test;

use bytemuck::{Pod, Zeroable};
use serde::Deserialize;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Pod, Zeroable, Deserialize)]
#[serde(from = "[u8; 4]")]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const EMPTY: Color = Color::new(0, 0, 0, 0);
    pub const BLACK: Color = Color::new(0, 0, 0, 255);
    pub const GRAY: Color = Color::new(128, 128, 128, 255);
    pub const WHITE: Color = Color::new(255, 255, 255, 255);
    pub const CYAN: Color = Color::new(0, 255, 255, 255);
    pub const MAGENTA: Color = Color::new(255, 0, 255, 255);

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        return Self { r, g, b, a };
    }
}

impl From<[u8; 4]> for Color {
    fn from(value: [u8; 4]) -> Self {
        return Self::new(value[0], value[1], value[2], value[3]);
    }
}

impl From<Color> for wgpu::Color {
    fn from(value: Color) -> Self {
        const MAX_COMPONENT: f64 = 255.0;

        return Self {
            r: value.r as f64 / MAX_COMPONENT,
            g: value.g as f64 / MAX_COMPONENT,
            b: value.b as f64 / MAX_COMPONENT,
            a: value.a as f64 / MAX_COMPONENT,
        };
    }
}

impl From<wgpu::Color> for Color {
    fn from(value: wgpu::Color) -> Self {
        const MIN_COMPONENT: f64 = 0.0;
        const MAX_COMPONENT: f64 = 1.0;
        const U8_MAX: f64 = 255.0;

        return Self::new(
            (value.r.clamp(MIN_COMPONENT, MAX_COMPONENT) * U8_MAX).round() as u8,
            (value.g.clamp(MIN_COMPONENT, MAX_COMPONENT) * U8_MAX).round() as u8,
            (value.b.clamp(MIN_COMPONENT, MAX_COMPONENT) * U8_MAX).round() as u8,
            (value.a.clamp(MIN_COMPONENT, MAX_COMPONENT) * U8_MAX).round() as u8,
        );
    }
}

impl std::ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        return Color::new(
            ((self.r as u16 * rhs.r as u16 + 127) / 255) as u8,
            ((self.g as u16 * rhs.g as u16 + 127) / 255) as u8,
            ((self.b as u16 * rhs.b as u16 + 127) / 255) as u8,
            ((self.a as u16 * rhs.a as u16 + 127) / 255) as u8,
        );
    }
}
