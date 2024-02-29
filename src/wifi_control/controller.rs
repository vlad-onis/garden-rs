use embedded_svc::{
    ipv4::Interface,
    wifi::{AccessPointInfo, ClientConfiguration, Configuration, Wifi},
};

use esp32c3_hal::{
    Rng,
    clock::Clocks,
    peripherals::{RNG, SYSTIMER, WIFI},
    prelude::*,
    system::RadioClockControl,
    Delay,
};

use esp_wifi::{
    current_millis, 
    initialize,
    EspWifiInitFor, 
    InitializationError,
    wifi::{utils::create_network_interface, WifiError, WifiMode, WifiStaDevice},
    wifi_interface::WifiStack,
};

use esp_backtrace as _;
use esp_println::println;

use smoltcp::iface::SocketStorage;
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
        wifi: WIFI,
        delay: &mut Delay,
    ) -> Result<(), WifiControllerError> {
        
        // Do not connect to a ssid and pass if the controller is not 
        // in Access Point - Station or Station mode
        if self.mode == WifiMode::Ap {
            return Err(WifiControllerError::NotStationMode);
        }

        let timer = esp32c3_hal::systimer::SystemTimer::new(sys_timer_peripheral).alarm0;

        let init = initialize(
            EspWifiInitFor::Wifi,
            timer,
            Rng::new(rng_peripheral),
            radio_clock_control,
            clocks,
        )?;

        println!("HERE1");

        let mut socket_set_entries: [SocketStorage; 3] = Default::default();
        let (iface, device, mut controller, sockets) =
            create_network_interface(&init, wifi, WifiStaDevice, &mut socket_set_entries).unwrap();
        let wifi_stack = WifiStack::new(iface, device, sockets, current_millis);

        let client_config = Configuration::Client(ClientConfiguration {
            ssid: self.ssid.try_into().unwrap(),
            password: self.password.try_into().unwrap(),
            ..Default::default()
        });
        let res = controller.set_configuration(&client_config);
        println!("wifi_set_configuration returned {:?}", res);

        controller.start().unwrap();
        println!("is wifi started: {:?}", controller.is_started());

        println!("Start Wifi Scan");
        let res: Result<(heapless::Vec<AccessPointInfo, 10>, usize), WifiError> =
            controller.scan_n();
        if let Ok((res, _count)) = res {
            for ap in res {
                println!("{:?}", ap);
            }
        }

        println!("{:?}", controller.get_capabilities());

        // wait to get connected
        println!("Wait to get connected");
        loop {
            while controller.connect().is_err() {}

            let res = controller.is_connected();
            match res {
                Ok(connected) => {
                    if connected {
                        break;
                    }
                }
                Err(err) => {
                    println!("{:?}", err);
                    delay.delay_ms(1000u32);
                    continue;
                }
            }
        }
        println!("{:?}", controller.is_connected());

        // wait for getting an ip address
        println!("Wait to get an ip address");
        loop {
            wifi_stack.work();

            if wifi_stack.is_iface_up() {
                println!("got ip {:?}", wifi_stack.get_ip_info());
                break;
            }
        }

        Ok(())
    }
}
