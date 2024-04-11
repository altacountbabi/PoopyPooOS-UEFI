use x86_64::instructions::port::{Port, PortWriteOnly};

#[allow(dead_code)]
pub fn reboot() -> ! {
    let mut port = Port::<u8>::new(0x64);

    unsafe {
        port.write(0xFE);
    }

    loop {}
}

// broken
#[allow(dead_code)]
pub fn suspend() -> ! {
    let mut acpi_port = PortWriteOnly::<u16>::new(0xB004);

    unsafe {
        acpi_port.write(0x0804);
    }

    loop {}
}

// broken
#[allow(dead_code)]
pub fn poweroff() -> ! {
    let mut acpi_cmd_port = PortWriteOnly::<u16>::new(0x604);

    unsafe {
        acpi_cmd_port.write(0x2000);
    }

    loop {}
}