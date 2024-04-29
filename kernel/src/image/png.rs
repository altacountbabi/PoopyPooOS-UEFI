use alloc::vec::Vec;
use png_decoder::{self, PngHeader};

use crate::framebuffer::Color;

pub struct PngData {
    pub pixels: Vec<Color>,
    pub header: PngHeader
}

pub fn decode_png(raw_data: Vec<u8>) -> PngData {
    let png_data = png_decoder::decode(raw_data.as_slice()).unwrap();

    let pixels = png_data.1;
    let mut blt_pixels: Vec<Color> = Vec::new();

    for chunk in pixels.chunks_exact(4) {
        blt_pixels.push(Color::new(chunk[0], chunk[1], chunk[2]));
    }

    PngData { pixels: blt_pixels, header: png_data.0 }
}