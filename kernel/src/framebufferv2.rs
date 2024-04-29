// DOES NOT WORK

use crate::println;
use bootloader_api::info::{FrameBuffer, PixelFormat};
use core::slice;
use spin::Mutex;

pub struct Buffer<'a> {
    pub width: usize,
    pub height: usize,
    pub pixels: &'a mut [u8],

    framebuffer: Mutex<&'a mut FrameBuffer>,
    back_buffer: &'a mut [u8],
}

impl<'a> Buffer<'a> {
    pub fn new(framebuffer: &'a mut FrameBuffer) -> Self {
        let fb_info = framebuffer.info();
        let (fb_width, fb_height): (usize, usize) = (fb_info.width, fb_info.height);
        let byte_len = fb_info.width * fb_info.height * fb_info.bytes_per_pixel;

        let pixels =
            unsafe { slice::from_raw_parts_mut(framebuffer.buffer_mut().as_mut_ptr(), byte_len) };
        let back_buffer =
            unsafe { slice::from_raw_parts_mut(framebuffer.buffer_mut().as_mut_ptr(), byte_len) };

        println!("{}", byte_len);

        Self {
            width: fb_width,
            height: fb_height,
            pixels,
            back_buffer,
            framebuffer: Mutex::new(framebuffer),
        }
    }

    #[allow(unused_variables)]
    #[allow(unused_assignments)]
    pub fn update(&mut self) {
        let mut buffer = self.framebuffer.lock().buffer_mut();
        self.pixels.clone_from_slice(&self.back_buffer);
        buffer = self.pixels;
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        let fb_info = self.framebuffer.lock().info();
        let byte_offset = {
            let line_offset = y * fb_info.stride;
            let pixel_offset = line_offset + x;
            pixel_offset
        };

        let pixel_buffer = &mut self.back_buffer[byte_offset..];
        match fb_info.pixel_format {
            PixelFormat::Rgb => {
                pixel_buffer[0] = r;
                pixel_buffer[1] = g;
                pixel_buffer[2] = b;
            }
            PixelFormat::Bgr => {
                pixel_buffer[0] = b;
                pixel_buffer[1] = g;
                pixel_buffer[2] = r;
            }
            PixelFormat::U8 => {
                let gray = r / 3 + g / 3 + b / 3;
                pixel_buffer[0] = gray;
            }
            other => panic!("unknown pixel format {:?}", other),
        }
    }
}
