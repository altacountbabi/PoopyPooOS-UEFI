use alloc::vec::Vec;
use png_decoder::{self, PngHeader};

use crate::framebuffer::Color;

pub struct PngData {
    pub pixels: Vec<Color>,
    pub header: PngHeader
}

pub fn decode_png(raw_data: Vec<u8>) -> PngData {
    let png_data = png_decoder::decode(raw_data.as_slice()).unwrap();

    let raw_pixels = png_data.1;
    let mut pixels: Vec<Color> = Vec::new();

    for chunk in raw_pixels.chunks_exact(4) {
        let alpha = alpha_blend(Color::new(chunk[0], chunk[1], chunk[2]), Color::new(0, 0, 0), chunk[3]);
        pixels.push(alpha);
    }

    PngData { pixels, header: png_data.0 }
}

fn alpha_blend(source_color: Color, dest_color: Color, alpha: u8) -> Color {
    let alpha_normalized = f32::from(alpha) / 255.0;

    let result_red = (f32::from(source_color.red) * alpha_normalized) + (f32::from(dest_color.red) * (1.0 - alpha_normalized));
    let result_green = (f32::from(source_color.green) * alpha_normalized) + (f32::from(dest_color.green) * (1.0 - alpha_normalized));
    let result_blue = (f32::from(source_color.blue) * alpha_normalized) + (f32::from(dest_color.blue) * (1.0 - alpha_normalized));

    let result_red = round_f32(result_red) as u8;
    let result_green = round_f32(result_green) as u8;
    let result_blue = round_f32(result_blue) as u8;

    Color::new(result_red, result_green, result_blue)
}

fn round_f32(value: f32) -> i32 {
    if value >= 0.0 {
        (value + 0.5) as i32
    } else {
        (value - 0.5) as i32
    }
}