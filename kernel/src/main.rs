#![no_std]
#![no_main]

// use core::f32::INFINITY;
use bootloader_api::/*{info::FrameBuffer,*/BootInfo/*}*/;
// use embedded_graphics::{geometry::Point, pixelcolor::Rgb888, Pixel};
// use framebuffer::{Color, Display, Position};
// use graphics::Graphics;

// mod framebuffer;
// mod graphics;
mod serial;

/*
fn delay(ms: u32) {
    let cycles_per_ms: u32 = 16_000;
    let iterations = cycles_per_ms * ms;
    
    for _ in 0..iterations {
        x86_64::instructions::nop();
    }
}

fn delay_cycles(cycles: u32) {
    for _ in 0..cycles {
        x86_64::instructions::nop();
    }
}

fn position_center(fb_width: usize, fb_height: usize, width: usize, height: usize) -> Position {
    Position {
        x: (fb_width / 2) - (width / 2),
        y: (fb_height / 2) - (height / 2)
    }
}

fn draw_rectangle(mut framebuffer: &mut FrameBuffer, position: framebuffer::Position, width: usize, height: usize, color: Color) {
    for y in 0..height {
        for x in 0..width {
            let position = Position {
                x: position.x + x,
                y: position.y + y
            };

            framebuffer::set_pixel_in(&mut framebuffer, position, color);
        }
    }
}

fn clear(mut framebuffer: &mut FrameBuffer, color: Color) {
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
*/

bootloader_api::entry_point!(kernel_main);
fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    /*
    let framebuffer = boot_info.framebuffer.as_mut().unwrap();
    let fb_info = framebuffer.info();
    let display = Display::new(framebuffer);
    let mut graphics = Graphics::new(display);

    // let time = rtc::read_rtc_time();
    // println!(time.0);


    loop {
        for i in 0..256 as u32 {
            draw_rectangle(framebuffer, framebuffer::Position { x: 0, y: 0 }, fb_info.width, fb_info.height, Color { red: i as u8, green: 0, blue: 0 });
        }

        for i in 0..256 as u32 {
            draw_rectangle(framebuffer, framebuffer::Position { x: 0, y: 0 }, fb_info.width, fb_info.height, Color { red: 0, green: i as u8, blue: 0 });
        }

        for i in 0..256 as u32 {
            draw_rectangle(framebuffer, framebuffer::Position { x: 0, y: 0 }, fb_info.width, fb_info.height, Color { red: 0, green: 0, blue: i as u8 });
        }
    }
    */

    

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
