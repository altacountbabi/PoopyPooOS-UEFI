use core::ptr;

const RTC_BASE_ADDR: usize = 0xABCDEF00;
const RTC_SECONDS_REG: *mut u32 = RTC_BASE_ADDR as *mut u32;
const RTC_MINUTES_REG: *mut u32 = (RTC_BASE_ADDR + 4) as *mut u32;
const RTC_HOURS_REG: *mut u32 = (RTC_BASE_ADDR + 8) as *mut u32;

pub fn read_rtc_time() -> (u32, u32, u32) {
    let seconds;
    let minutes;
    let hours;

    unsafe {
        seconds = ptr::read_volatile(RTC_SECONDS_REG);
        minutes = ptr::read_volatile(RTC_MINUTES_REG);
        hours = ptr::read_volatile(RTC_HOURS_REG);
    }

    (seconds, minutes, hours)
}