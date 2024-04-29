use std::{
    env,
    process::{self, Command},
};

fn main() {
    let image = format!("format=raw,file={}", env!("UEFI_IMAGE"));
    let bios = ovmf_prebuilt::ovmf_pure_efi().display().to_string();
    let args = vec![
        "-drive", image.as_str(),
        "-display", "sdl",
        "-m", "2G",
        "-serial", "stdio",
        "-bios", bios.as_str()
    ];

    let mut qemu = Command::new("qemu-system-x86_64");

    for arg in args {
        qemu.arg(arg);
    }

    let exit_status = qemu.status().unwrap();
    process::exit(exit_status.code().unwrap_or(-1));
}