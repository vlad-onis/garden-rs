use esp32c3_hal::Rng;
use esp32c3_hal::{
    clock::Clocks,
    peripherals::{RNG, SYSTIMER},
    system::RadioClockControl,
};

use esp_backtrace as _;
use esp_wifi::{initialize, wifi::WifiMode, EspWifiInitFor, InitializationError};

use thiserror_no_std::Error;

#[derive(Debug, Error)]
pub enum WifiControllerError {
    #[error("Cannot connect to wifi if the controller is not in AP_STA or STA mode")]
    NotStationMode,

    #[error("Failed to initialize Wifi")]
    WifiInitError(#[from] InitializationError),
}

pub struct WifiController<'a> {
    pub ssid: &'a str,
    pub password: &'a str,
    pub mode: WifiMode,
}

impl<'a> WifiController<'a> {
    pub fn new(ssid: &'a str, password: &'a str, mode: WifiMode) -> WifiController<'a> {
        WifiController {
            ssid,
            password,
            mode,
        }
    }

    pub fn connect_to_wifi(
        &self,
        sys_timer_peripheral: SYSTIMER,
        rng_peripheral: RNG,
        radio_clock_control: RadioClockControl,
        clocks: &Clocks,
    ) -> Result<(), WifiControllerError> {
        // Do not connect to a ssid and pass if the controller is not in Access Point
        // or Station mode
        if self.mode != WifiMode::ApSta || self.mode != WifiMode::Sta {
            return Err(WifiControllerError::NotStationMode);
        }

        let timer = esp32c3_hal::systimer::SystemTimer::new(sys_timer_peripheral).alarm0;

        let _init = initialize(
            EspWifiInitFor::Wifi,
            timer,
            Rng::new(rng_peripheral),
            radio_clock_control,
            clocks,
        )?;

        Ok(())
    }
}
