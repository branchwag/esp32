#![no_std]
#![no_main]

use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut led = Output::new(peripherals.GPIO45, Level::High, OutputConfig::default());

    loop {
        led.set_low();
        let t = Instant::now();
        while t.elapsed() < Duration::from_millis(500) {}

        led.set_high();
        let t = Instant::now();
        while t.elapsed() < Duration::from_millis(500) {}
    }
}
