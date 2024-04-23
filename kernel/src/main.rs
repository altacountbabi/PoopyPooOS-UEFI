#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use bootloader_api::{config::Mapping, BootInfo, BootloaderConfig};
use x86_64::structures::paging::Translate;

use crate::memory::BootInfoFrameAllocator;

mod serial;
mod interrupts;
mod gdt;
mod memory;

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
*/

fn kernel_init() {
    gdt::init();
    interrupts::init();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};


bootloader_api::entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);
#[allow(unconditional_panic)]
#[allow(unused_assignments)]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {    
    // let frame_buffer_optional = &mut boot_info.framebuffer;
    // let frame_buffer_option = frame_buffer_optional.as_mut();
    // let frame_buffer_struct = frame_buffer_option.unwrap();
    // let frame_buffer_info = frame_buffer_struct.info().clone();
    // let raw_frame_buffer = frame_buffer_struct.buffer_mut();

    println!("Clearing framebuffer...");

    // raw_frame_buffer.fill(0);

    kernel_init();

    let (ramdisk_len, ramdisk_addr) = (boot_info.ramdisk_len, boot_info.ramdisk_addr);

    unsafe {
        let ramdisk_ptr: *const u8 = ramdisk_addr.into_option().unwrap() as *const u8;
        let ramdisk_slice = core::slice::from_raw_parts(ramdisk_ptr, ramdisk_len.try_into().unwrap());

        for &byte in ramdisk_slice {
            print!("{}", char::from_u32(byte as u32).unwrap());
        }
    }

    println!("");

    use x86_64::VirtAddr;
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    let mapper = unsafe { memory::init(phys_mem_offset) };
    let _frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_regions)
    };
    println!("Phyisical memory offset: 0x{:x}", phys_mem_offset.as_u64());

    {
        let addresses = [
            0x8000000000,
            0x29000,
            0x2a000,
        ];
        
        for &address in &addresses {
            let virt = VirtAddr::new(address);
            let phys = mapper.translate_addr(virt);
            println!("{:?} -> {:?}", virt, phys);
        }
    }

    hlt_loop();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    log::error!("{}", info);
    hlt_loop();
}
