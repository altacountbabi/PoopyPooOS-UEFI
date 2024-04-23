use bootloader::{DiskImageBuilder, BootConfig};
use std::{env, path::PathBuf};


fn main() {
    let config = {
        let mut default = BootConfig::default();
        default.frame_buffer_logging = false;

        default
    };

    let kernel_path = env::var("CARGO_BIN_FILE_KERNEL").unwrap();
    let mut disk_builder = DiskImageBuilder::new(PathBuf::from(kernel_path));
    disk_builder.set_boot_config(&config);
    let mut ramdisk_path = PathBuf::new();
    ramdisk_path.push("/home");
    ramdisk_path.push("real");
    ramdisk_path.push("projects");
    ramdisk_path.push("kernel");
    ramdisk_path.push("ramdisk");
    disk_builder.set_ramdisk(ramdisk_path);

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let uefi_path = out_dir.join("poopypooos-uefi.img");
    let bios_path = out_dir.join("poopypooos-bios.img");

    _ = disk_builder.create_uefi_image(&uefi_path);
    _ = disk_builder.create_bios_image(&bios_path);

    println!("cargo:rustc-env=UEFI_IMAGE={}", uefi_path.display());
    println!("cargo:rustc-env=BIOS_IMAGE={}", bios_path.display());
}