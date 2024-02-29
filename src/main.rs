#![no_std]
#![no_main]

use esp32c3_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay};
use esp_backtrace as _;
use esp_println::println;
use esp_wifi::wifi::WifiMode;

mod garden;
mod wifi_control;

use wifi_control::controller::WifiController;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let wifi_control = WifiController::new("Hoemies", "AreAlwaysWelcome12@", WifiMode::ApSta);
    let connected = wifi_control.connect_to_wifi(
        peripherals.SYSTIMER,
        peripherals.RNG,
        system.radio_clock_control,
        &clocks,
        peripherals.WIFI,
        &mut delay,
    );

    println!("{:?}", connected);

    panic!("This never returns");

    // let analog = peripherals.APB_SARADC.split();

    // let mut gardener = Gardener::setup(io, analog);

    // loop {
    //     let h = gardener.read_humidity();
    //     println!("Humidity percent= {:?}", h);
    //     delay.delay_ms(10000u32);
    // }
}
