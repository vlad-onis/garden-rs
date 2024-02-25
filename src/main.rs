#![no_std]
#![no_main]

use esp32c3_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*};
use esp_backtrace as _;

mod garden;
mod wifi_control;

use esp_wifi::wifi::WifiMode;
use wifi_control::controller::WifiController;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let controller = WifiController::new("Test", "Test", WifiMode::ApSta);

    let _ = controller.connect_to_wifi(
        peripherals.SYSTIMER,
        peripherals.RNG,
        system.radio_clock_control,
        &clocks,
    );

    panic!("this never returns");
    // let analog = peripherals.APB_SARADC.split();

    // let mut gardener = Gardener::setup(io, analog);

    // loop {
    //     let h = gardener.read_humidity();
    //     println!("Humidity percent= {:?}", h);
    //     delay.delay_ms(10000u32);
    // }
}
