#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

pub mod wifi_control;

use embassy_executor::Spawner;
use esp_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*};
use esp_wifi::wifi::WifiMode;
use wifi_control::controller;

#[main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = Peripherals::take();

    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    #[cfg(target_arch = "riscv32")]
    let timer = esp_hal::systimer::SystemTimer::new(peripherals.SYSTIMER).alarm0;

    let _ = controller::connect_to_wifi(
        WifiMode::ApSta,
        spawner,
        peripherals.WIFI,
        timer,
        peripherals.TIMG0,
        peripherals.RNG,
        system.radio_clock_control,
        &clocks,
    )
    .await;

    loop {}
}
