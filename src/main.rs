#![no_std]
#![no_main]

use esp32c3_hal::{clock::ClockControl, gpio::IO, peripherals::Peripherals, prelude::*, Delay};
use esp_backtrace as _;
use esp_println::{dbg, println};
use esp_wifi::wifi::WifiMode;

mod garden;
mod wifi_control;

use garden_rs::garden::gardener::Gardener;
use wifi_control::controller::WifiController;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let wifi_control = WifiController::new("MERCUSYS_1A43", "56272697", WifiMode::ApSta);
    let connected = wifi_control.connect_to_wifi(
        peripherals.SYSTIMER,
        peripherals.RNG,
        system.radio_clock_control,
        &clocks,
        peripherals.WIFI,
        &mut delay,
    );

    dbg!("Connected to wifi: {connected:?}");

    let analog = peripherals.APB_SARADC.split();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut gardener = Gardener::setup(io, analog);

    loop {
        let h = gardener.read_humidity();
        println!("Humidity percent= {:?}", h);
        delay.delay_ms(3000u32);
    }
}
