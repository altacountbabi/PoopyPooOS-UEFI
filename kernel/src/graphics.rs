use embedded_graphics::{geometry::Point, pixelcolor::Rgb888, Pixel};

use crate::framebuffer;

pub struct Graphics<'a> {
    display: framebuffer::Display<'a>
}

impl Graphics<'_> {
    pub fn new(display: framebuffer::Display) -> Graphics {
        Graphics { display }
    }

    pub fn draw_rectangle(&mut self, position: framebuffer::Position, width: usize, height: usize, color: Rgb888) {
        for y in 0..height {
            for x in 0..width {
                let position = Point::new((position.x + x) as i32, (position.y + y) as i32);
    
                self.display.draw_pixel(Pixel(position, color));
            }
        }
    }

    #[allow(dead_code)]
    pub fn draw_pixel(&mut self, Pixel(coordinates, color): Pixel<Rgb888>) {
        self.display.draw_pixel(Pixel(coordinates, color))
    }
}