#![no_std]
#![no_main]

use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_storage::FlashStorage;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut flash = FlashStorage::new(peripherals.FLASH);
    let mut buffer = [0u8; esp_bootloader_esp_idf::partitions::PARTITION_TABLE_MAX_LEN];
    if esp_bootloader_esp_idf::partitions::read_partition_table(&mut flash, &mut buffer).is_ok() {
        if let Ok(mut ota) = esp_bootloader_esp_idf::ota_updater::OtaUpdater::new(&mut flash, &mut buffer) {
            let _ = ota.set_current_ota_state(esp_bootloader_esp_idf::ota::OtaImageState::Valid);
        }
    }

    let _red_off = Output::new(peripherals.GPIO46, Level::High, OutputConfig::default());
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
