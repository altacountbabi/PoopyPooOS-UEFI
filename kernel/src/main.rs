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

bootloader_api::entry_point!(kernel_main, config = &config::BOOTLOADER_CONFIG);
#[allow(unconditional_panic)]
#[allow(unused_assignments)]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    kernel_init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    let mut mapper = unsafe { memory::memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_regions) };
    println!("Phyisical memory offset: 0x{:x}", phys_mem_offset.as_u64());

    println!(
        "{}x{}",
        boot_info.framebuffer.as_ref().unwrap().info().width,
        boot_info.framebuffer.as_ref().unwrap().info().height
    );
    println!("{:#?}", boot_info.framebuffer.as_ref().unwrap().info());

    println!("Initializing heap");
    memory::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Failed to initialize the heap.");

    println!("Reading ramdisk");
    let (ramdisk_len, ramdisk_addr) = (boot_info.ramdisk_len, boot_info.ramdisk_addr);
    let ramdisk = read_ramdisk(ramdisk_addr, ramdisk_len);

    let mut fb = Framebuffer::new(boot_info.framebuffer.as_mut().unwrap());
    render_png(&mut fb, decode_png(ramdisk), Position::new(50, 50));

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.run();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
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
