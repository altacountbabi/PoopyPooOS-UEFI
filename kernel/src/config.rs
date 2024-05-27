use bootloader_api::{config::Mapping, BootloaderConfig};

pub const HEAP_START: usize = 0x444444440000;
pub const HEAP_SIZE: usize = 5 * 1024 * 1024; // 30 MiB

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);

    config
};
