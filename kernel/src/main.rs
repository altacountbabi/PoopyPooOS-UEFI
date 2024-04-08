#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader_api::BootInfo;

bootloader_api::entry_point!(kernel_main);
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(frambuffer) = boot_info.framebuffer.as_mut() {
        for byte in frambuffer.buffer_mut() {
            *byte = 0x90;
        }
    }
    
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}