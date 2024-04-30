use alloc::vec::Vec;
use png_decoder::{self, PngHeader};

use crate::framebuffer::Color;

pub struct PngData {
    pub pixels: Vec<Color>,
    pub header: PngHeader,
}

pub fn decode_png(raw_data: Vec<u8>) -> PngData {
    let png_data = png_decoder::decode(raw_data.as_slice()).unwrap();

    let raw_pixels = png_data.1;
    let mut pixels: Vec<Color> = Vec::new();

    for chunk in raw_pixels.chunks_exact(4) {
        pixels.push(Color::new(chunk[0], chunk[1], chunk[2], chunk[3]));
    }

    PngData {
        pixels,
        header: png_data.0,
    }
}