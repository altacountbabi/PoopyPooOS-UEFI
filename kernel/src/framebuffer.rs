use core::panic;

use embedded_graphics::{draw_target::DrawTarget, geometry::{OriginDimensions, Size}, pixelcolor::{Rgb888, RgbColor}, Pixel};
use bootloader_api::info::{FrameBuffer, FrameBufferInfo, PixelFormat};

pub struct Display<'f> {
    framebuffer: &'f mut FrameBuffer,
    info: FrameBufferInfo,
}

impl<'f> Display<'f> {
    pub fn new(framebuffer: &'f mut FrameBuffer) -> Display {
        let info = framebuffer.info();
        Display { framebuffer, info }
    }

    pub fn draw_pixel(&mut self, Pixel(coordinates, color): Pixel<Rgb888>) {
        let (width, height) = (self.info.width, self.info.height);
        let (x, y) = coordinates.into();

        if (0..width as i32).contains(&x) && (0..height as i32).contains(&y) {
            let color = Color {
                red: color.r(),
                green: color.g(),
                blue: color.b(),
            };

            set_pixel_in(self.framebuffer, Position { x: x as usize, y: y as usize }, color);
        }
    }
}

impl<'f> DrawTarget for Display<'f> {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels.into_iter() {
            let (x, y) = pixel.0.into();
            if (0..self.info.width as i32).contains(&x) && (0..self.info.height as i32).contains(&y) {
                let color = Color {
                    red: pixel.1.r(),
                    green: pixel.1.g(),
                    blue: pixel.1.b(),
                };
                set_pixel_in(self.framebuffer, Position { x: x as usize, y: y as usize }, color);
            }
        }
        Ok(())
    }
}

impl<'f> OriginDimensions for Display<'f> {
    fn size(&self) -> Size {
        let info = self.framebuffer.info();

        Size::new(info.width as u32, info.height as u32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

pub fn set_pixel_in(framebuffer: &mut FrameBuffer, position: Position, color: Color) {
    let info = framebuffer.info();

    let byte_offset = {
        let line_offset = position.y * info.stride;
        let pixel_offset = line_offset + position.x;
        pixel_offset * info.bytes_per_pixel
    };

    let pixel_buffer = &mut framebuffer.buffer_mut()[byte_offset..];
    match info.pixel_format {
        PixelFormat::Rgb => {
            pixel_buffer[0] = color.red;
            pixel_buffer[1] = color.green;
            pixel_buffer[2] = color.blue;
        },
        PixelFormat::Bgr => {
            pixel_buffer[0] = color.blue;
            pixel_buffer[1] = color.green;
            pixel_buffer[2] = color.red;
        }
        PixelFormat::U8 => {
            let gray = color.red / 3 + color.green / 3 + color.blue / 3;

            pixel_buffer[0] = gray;
        }
        other => panic!("unkown pixel format {other:?}")
    }
}