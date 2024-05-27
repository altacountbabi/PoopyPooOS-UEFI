#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

use alloc::vec::Vec;
use bootloader_api::{info::Optional, BootInfo};
use x86_64::VirtAddr;

use crate::{
    framebuffer::{Framebuffer, Position},
    image::{png::decode_png, renderer::render_png},
    memory::memory::BootInfoFrameAllocator,
    task::{executor::Executor, Task},
};

mod config;
mod framebuffer;
mod gdt;
mod image;
mod interrupts;
mod memory;
mod serial;
mod task;

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

const CPU_FREQUENCY: u64 = 2_000_000_000;

fn delay_cycles(cycles: u64) {
    for _ in 0..cycles {
        x86_64::instructions::nop();
    }
}

fn delay_ms(ms: u64) {
    let cycles_per_ms = CPU_FREQUENCY / 1_000;
    delay_cycles(ms * cycles_per_ms);
}

bootloader_api::entry_point!(kernel_main, config = &config::BOOTLOADER_CONFIG);
#[allow(unconditional_panic)]
#[allow(unused_assignments)]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    kernel_init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    let mut mapper = unsafe { memory::memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_regions) };
    println!("Phyisical memory offset: 0x{:x}", phys_mem_offset.as_u64());

    println!("Initializing heap");
    memory::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Failed to initialize the heap.");

    println!("Reading ramdisk");
    let (ramdisk_len, ramdisk_addr) = (boot_info.ramdisk_len, boot_info.ramdisk_addr);
    let ramdisk = read_ramdisk(ramdisk_addr, ramdisk_len);

    // acpi::init(boot_info.rsdp_addr.into_option().unwrap());

    let mut fb = Framebuffer::new(boot_info.framebuffer.as_mut().unwrap());

    println!(
        "Framebuffer resolution is: {}x{}",
        fb.info().width,
        fb.info().height
    );

    render_png(&mut fb, decode_png(ramdisk), Position::new(50, 50));

    let mut executor = Executor::new();
    executor.spawn(Task::new(async_example(1)));
    executor.run();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

async fn async_example(num: u8) {
    delay_ms((num * 2) as u64);
    println!("Waited for {}", (num * 2));
    println!("{}", num);
}

fn read_ramdisk(ramdisk_addr: Optional<u64>, ramdisk_len: u64) -> Vec<u8> {
    let mut ramdisk: Vec<u8> = Vec::new();

    unsafe {
        let ramdisk_ptr: *const u8 = ramdisk_addr.into_option().unwrap() as *const u8;
        let ramdisk_slice =
            core::slice::from_raw_parts(ramdisk_ptr, ramdisk_len.try_into().unwrap());

        for &byte in ramdisk_slice {
            ramdisk.push(byte);
        }
    }

    ramdisk
}
