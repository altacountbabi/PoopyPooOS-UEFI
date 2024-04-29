use bootloader_api::info::{FrameBuffer, FrameBufferInfo, PixelFormat};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

pub struct Framebuffer<'a> {
    framebuffer: &'a mut FrameBuffer,
}

impl Framebuffer<'_> {
    pub fn new(framebuffer: &mut FrameBuffer) -> Framebuffer {
        Framebuffer { framebuffer }
    }

    pub fn draw_pixel(&mut self, position: Position, color: Color) {
        let info = self.framebuffer.info();

        let byte_offset = {
            let line_offset = position.y * info.stride;
            let pixel_offset = line_offset + position.x;
            pixel_offset * info.bytes_per_pixel
        };

        let pixel_buffer = &mut self.framebuffer.buffer_mut()[byte_offset..];
        match info.pixel_format {
            PixelFormat::Rgb => {
                pixel_buffer[0] = color.red;
                pixel_buffer[1] = color.green;
                pixel_buffer[2] = color.blue;
            }
            PixelFormat::Bgr => {
                pixel_buffer[0] = color.blue;
                pixel_buffer[1] = color.green;
                pixel_buffer[2] = color.red;
            }
            PixelFormat::U8 => {
                let gray = color.red / 3 + color.green / 3 + color.blue / 3;

                pixel_buffer[0] = gray;
            }
            other => core::panic!("unkown pixel format {other:?}"),
        }
    }

    pub fn info(&mut self) -> FrameBufferInfo {
        self.framebuffer.info()
    }
}
