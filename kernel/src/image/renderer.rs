use crate::{
    framebuffer::{Color, Framebuffer, Position},
    image::png::PngData,
};

pub fn render_png(framebuffer: &mut Framebuffer, image: PngData, image_position: Position) {
    let (image_width, image_height) = (image.header.width, image.header.height);

    for y in 0..image_height {
        for x in 0..image_width {
            if image.pixels[(y * image_width as usize as u32 + x) as usize] == Color::new(0, 0, 0) {
                continue;
            }

            let pixel_x = x + image_position.x as u32;
            let pixel_y = y + image_position.y as u32;

            let (fb_width, fb_height): (u32, u32) = (
                framebuffer.info().width.try_into().unwrap(),
                framebuffer.info().height.try_into().unwrap(),
            );

            if pixel_x > fb_width || pixel_y > fb_height {
                continue;
            }

            let pixel_position =
                Position::new(pixel_x.try_into().unwrap(), pixel_y.try_into().unwrap());
            let pixel_position2 = Position::new(x.try_into().unwrap(), y.try_into().unwrap());

            framebuffer.draw_pixel(
                Position::new(pixel_position.x, pixel_position.y),
                image.pixels[pixel_position2.y * image_width as usize + pixel_position2.x],
            );
        }
    }
}
