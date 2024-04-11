use std::{
    env,
    process::{self, Command},
};

fn main() {
    let image = format!("format=raw,file={}", env!("BIOS_IMAGE"));
    let args = vec![
        "-drive", image.as_str(),
        "-display", "sdl",
        "-serial", "stdio",
        "-s", 
        "-S"
    ];

    let mut qemu = Command::new("qemu-system-x86_64");

    for arg in args {
        qemu.arg(arg);
    }

    let exit_status = qemu.status().unwrap();
    process::exit(exit_status.code().unwrap_or(-1));
}