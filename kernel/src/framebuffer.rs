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
    pub alpha: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
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
        if color.alpha == 0 {
            return;
        }

        let info = self.framebuffer.info();

        let byte_offset = {
            let line_offset = position.y * info.stride;
            let pixel_offset = line_offset + position.x;
            pixel_offset * info.bytes_per_pixel
        };

        let alpha_color: Color = alpha_blend(
            Color::new(color.red, color.green, color.blue, 255),
            self.get_pixel(position),
            color.alpha,
        );

        let pixel_buffer = &mut self.framebuffer.buffer_mut()[byte_offset..];
        match info.pixel_format {
            PixelFormat::Rgb => {
                pixel_buffer[0] = alpha_color.red;
                pixel_buffer[1] = alpha_color.green;
                pixel_buffer[2] = alpha_color.blue;
            }
            PixelFormat::Bgr => {
                pixel_buffer[0] = alpha_color.blue;
                pixel_buffer[1] = alpha_color.green;
                pixel_buffer[2] = alpha_color.red;
            }
            PixelFormat::U8 => {
                let gray = alpha_color.red / 3 + alpha_color.green / 3 + alpha_color.blue / 3;

                pixel_buffer[0] = gray;
            }
            other => core::panic!("Unkown pixel format \"{other:?}\""),
        }
    }

    pub fn get_pixel(&mut self, position: Position) -> Color {
        let info = self.framebuffer.info();

        let byte_offset = {
            let line_offset = position.y * info.stride;
            let pixel_offset = line_offset + position.x;
            pixel_offset * info.bytes_per_pixel
        };

        let mut color = Color::new(0, 0, 0, 255);

        let pixel_buffer = &self.framebuffer.buffer_mut()[byte_offset..];
        match info.pixel_format {
            PixelFormat::Rgb => {
                color.red = pixel_buffer[0];
                color.green = pixel_buffer[1];
                color.blue = pixel_buffer[2];
            }
            PixelFormat::Bgr => {
                color.blue = pixel_buffer[0];
                color.green = pixel_buffer[1];
                color.red = pixel_buffer[2];
            }
            other => core::panic!("Unkown pixel format \"{other:?}\""),
        }

        color
    }

    pub fn info(&mut self) -> FrameBufferInfo {
        self.framebuffer.info()
    }
}

fn alpha_blend(original_color: Color, background_color: Color, alpha: u8) -> Color {
    let alpha_normalized = f32::from(alpha) / 255.0;

    let result_red = (f32::from(original_color.red) * alpha_normalized)
        + (f32::from(background_color.red) * (1.0 - alpha_normalized));
    let result_green = (f32::from(original_color.green) * alpha_normalized)
        + (f32::from(background_color.green) * (1.0 - alpha_normalized));
    let result_blue = (f32::from(original_color.blue) * alpha_normalized)
        + (f32::from(background_color.blue) * (1.0 - alpha_normalized));

    let result_red = round_f32(result_red) as u8;
    let result_green = round_f32(result_green) as u8;
    let result_blue = round_f32(result_blue) as u8;

    Color::new(result_red, result_green, result_blue, 255)
}

fn round_f32(value: f32) -> i32 {
    if value >= 0.0 {
        (value + 0.5) as i32
    } else {
        (value - 0.5) as i32
    }
}
